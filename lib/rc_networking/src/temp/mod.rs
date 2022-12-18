use bevy::ecs::schedule::ShouldRun;
use bevy::prelude::*;
use crate::Channel;
use crate::protocol::Protocol;

pub mod constants;
pub mod protocol;
pub mod types;

fn get_channel(protocol: &Protocol) -> Channel {
    match protocol {
        Protocol::PlayerMove(_)
        | Protocol::EntityMoved(_)
        | Protocol::PlayerRotate(_)
        | Protocol::EntityRotated(_) => Channel::Unreliable,

        Protocol::BlockUpdate(_)
        | Protocol::ChatSent(_)
        | Protocol::DespawnEntity(_)
        | Protocol::SpawnEntity(_)
        | Protocol::RequestChunk(_) => Channel::Reliable,

        Protocol::PartialChunkUpdate(_) => Channel::Chunk,
    }
}

fn has_resource<T: Resource>(resource: Option<Res<T>>) -> ShouldRun {
    match resource.is_some() {
        true => ShouldRun::Yes,
        false => ShouldRun::No,
    }
}

pub mod client2 {
    use std::ops::{Deref, DerefMut};

    use bevy::app::AppExit;
    use bevy::prelude::*;
    use renet::{RenetClient, RenetError};

    use crate::temp::*;
    use crate::constants::UserId;
    use crate::types::{ReceivePacket, SendPacket};

    pub struct RenetClientPlugin;

    impl Plugin for RenetClientPlugin {
        fn build(&self, app: &mut App) {
            use bevy::prelude::CoreStage::*;

            app.add_event::<RenetError>()
                .add_system_to_stage(
                    PreUpdate,
                    update_system.with_run_criteria(has_resource::<Client>),
                )
                .add_system_to_stage(
                    PreUpdate,
                    read_packets_system
                        .after(update_system)
                        .with_run_criteria(has_resource::<Client>),
                )
                .add_system_to_stage(
                    PostUpdate,
                    detect_shutdown_system
                        .after(bevy::window::exit_on_all_closed)
                        .with_run_criteria(has_resource::<Client>),
                )
                .add_system_to_stage(
                    PostUpdate,
                    write_packets_system
                        .before(send_packets_system)
                        .with_run_criteria(has_resource::<Client>),
                )
                .add_system_to_stage(
                    PostUpdate,
                    send_packets_system.with_run_criteria(has_resource::<Client>),
                );
        }
    }

    fn update_system(
        mut client: ResMut<Client>,
        mut renet_error: EventWriter<RenetError>,
        time: Res<Time>,
        mut commands: Commands,
    ) {
        if let Err(e) = client.update(time.delta()) {
            if let RenetError::IO(err) = e {
                // Assume IO errors are not recoverable
                commands.remove_resource::<Client>();
                error!("IO Error with server connection {:?}. Terminating.", err);
            } else {
                error!("Renet Update: {}", e);
                renet_error.send(e);
            }
        }
    }

    fn read_packets_system(mut client: ResMut<Client>, mut recv: EventWriter<ReceivePacket>) {
        fn send(client: &mut Client, recv: &mut EventWriter<ReceivePacket>, channel: Channel) {
            while let Some(bytes) = client.receive_message(channel) {
                let protocol: Protocol = bincode::deserialize(&bytes).unwrap();
                recv.send(ReceivePacket(protocol, UserId(client.client_id())));
            }
        }
        send(&mut client, &mut recv, Channel::Unreliable);
        send(&mut client, &mut recv, Channel::Reliable);
        send(&mut client, &mut recv, Channel::Chunk);
    }

    fn write_packets_system(mut client: ResMut<Client>, mut to_send: EventReader<SendPacket>) {
        to_send.iter().for_each(|v: &SendPacket| {
            let ser = bincode::serialize(&v.0).unwrap();
            let channel = get_channel(&v.0);
            client.send_message(channel, ser);
        })
    }

    fn detect_shutdown_system(mut client: ResMut<Client>, mut bevy_shutdown: EventReader<AppExit>) {
        for _ in bevy_shutdown.iter() {
            info!("Shutting down server");
            client.disconnect();
        }
    }

    fn send_packets_system(mut client: ResMut<Client>, mut renet_error: EventWriter<RenetError>) {
        if let Err(e) = client.send_packets() {
            error!("Renet Send: {}", e);
            renet_error.send(e);
        }
    }

    #[derive(Resource)]
    pub struct Client(pub RenetClient);

    impl Deref for Client {
        type Target = RenetClient;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl DerefMut for Client {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
}

pub mod server2 {
    use std::ops::{Deref, DerefMut};

    use bevy::app::AppExit;
    use bevy::prelude::*;
    use renet::{RenetError, RenetServer, ServerEvent};

    use crate::temp::*;
    use crate::constants::UserId;
    use crate::types::{ReceivePacket, SendPacket};

    pub struct RenetServerPlugin;

    impl Plugin for RenetServerPlugin {
        fn build(&self, app: &mut App) {
            use bevy::prelude::CoreStage::*;
            app.add_event::<RenetError>()
                .add_event::<ServerEvent>()
                .add_system_to_stage(
                    PreUpdate,
                    update_system.with_run_criteria(has_resource::<Server>),
                )
                .add_system_to_stage(
                    PreUpdate,
                    read_packets_system
                        .after(update_system)
                        .with_run_criteria(has_resource::<Server>),
                )
                .add_system(detect_shutdown_system.with_run_criteria(has_resource::<Server>))
                .add_system_to_stage(
                    PostUpdate,
                    write_packets_system
                        .before(send_packets_system)
                        .with_run_criteria(has_resource::<Server>),
                )
                .add_system_to_stage(
                    PostUpdate,
                    send_packets_system.with_run_criteria(has_resource::<Server>),
                );
        }
    }

    fn update_system(
        mut server: ResMut<Server>,
        mut renet_error: EventWriter<RenetError>,
        mut server_events: EventWriter<ServerEvent>,
        time: Res<Time>,
    ) {
        if let Err(e) = server.update(time.delta()) {
            error!("Renet Update: {}", e);
            renet_error.send(RenetError::IO(e));
        }

        while let Some(event) = server.get_event() {
            info!("{:?}", event);
            server_events.send(event);
        }
    }

    fn read_packets_system(mut server: ResMut<Server>, mut recv: EventWriter<ReceivePacket>) {
        server.clients_id().iter().for_each(|&user_id| {
            fn send(
                server: &mut Server,
                recv: &mut EventWriter<ReceivePacket>,
                user_id: u64,
                channel: Channel,
            ) {
                while let Some(bytes) = server.receive_message(user_id, channel) {
                    let protocol: Protocol = bincode::deserialize(&bytes).unwrap();
                    recv.send(ReceivePacket(protocol, UserId(user_id)));
                }
            }
            send(&mut server, &mut recv, user_id, Channel::Unreliable);
            send(&mut server, &mut recv, user_id, Channel::Reliable);
            send(&mut server, &mut recv, user_id, Channel::Chunk);
        })
    }

    fn write_packets_system(mut server: ResMut<Server>, mut to_send: EventReader<SendPacket>) {
        to_send.iter().for_each(|v| {
            let ser = bincode::serialize(&v.0).unwrap();
            let channel = get_channel(&v.0);
            server.send_message(v.1 .0, channel, ser);
        })
    }

    fn detect_shutdown_system(mut server: ResMut<Server>, mut bevy_shutdown: EventReader<AppExit>) {
        for _ in bevy_shutdown.iter() {
            info!("Shutting down server");
            server.disconnect_clients();
        }
    }

    fn send_packets_system(mut server: ResMut<Server>, mut renet_error: EventWriter<RenetError>) {
        if let Err(e) = server.send_packets() {
            error!("Renet Send: {}", e);
            renet_error.send(RenetError::IO(e))
        }
    }

    #[derive(Resource)]
    pub struct Server(pub RenetServer);

    impl Deref for Server {
        type Target = RenetServer;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl DerefMut for Server {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
}
