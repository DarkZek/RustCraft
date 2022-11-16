use std::net::SocketAddr;
use std::time::SystemTime;
use bevy::prelude::{Res, Resource};
use bevy::ecs::schedule::ShouldRun;
use renet::{ConnectToken, RenetConnectionConfig};
use rc_protocol::protocol::Protocol;

pub use renet;
pub use client::*;
pub use server::*;

pub const PROTOCOL_ID: u64 = 4302467916224429941;

// current private key is SHA256 hash of format!("{}{}", PROTOCOL_ID, "RustCraft");
pub const PRIVATE_KEY: [u8; 32] = [
    0x2e, 0x7c, 0x89, 0x9c, 0xf6, 0x46, 0x8d, 0x19,
    0x4b, 0x38, 0x14, 0xfd, 0xea, 0xa8, 0x7a, 0xce,
    0xf2, 0xc7, 0x2d, 0x99, 0x2b, 0x1b, 0xe2, 0x5d,
    0x29, 0x2d, 0xd3, 0x26, 0x52, 0x71, 0x8a, 0x1b
];

pub fn get_renet_connection_config() -> RenetConnectionConfig {
    let config = RenetConnectionConfig::default();
    // for adding channels append to these (append the same Config to both, ideally using clone)
    //config.receive_channels_config
    //config.send_channels_config
    config
}

pub fn get_simple_connect_token(client_id: u64, addresses: Vec<SocketAddr>) -> ConnectToken {
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();

    ConnectToken::generate(
        current_time,
        PROTOCOL_ID,
        100,
        client_id,
        120,
        addresses,
        None,
        &PRIVATE_KEY,
    ).unwrap()
}

#[derive(Copy, Clone)]
pub enum Channel {
    Reliable,
    Unreliable,
    Block,
}

impl From<Channel> for u8 {
    fn from(value: Channel) -> Self {
        match value {
            Channel::Reliable => 0,
            Channel::Unreliable => 1,
            Channel::Block => 2,
        }
    }
}

fn get_channel(protocol: &Protocol) -> Channel {
    match protocol {
        Protocol::PlayerMove(_) |
        Protocol::EntityMoved(_) |
        Protocol::PlayerRotate(_) |
        Protocol::EntityRotated(_) |
        Protocol::Disconnect(_) => { Channel::Unreliable }

        Protocol::Ping(_) |
        Protocol::Pong(_) |
        Protocol::PlayerJoin(_) |
        Protocol::PlayerLeave(_) |
        Protocol::BlockUpdate(_) |
        Protocol::ChatSent(_) |
        Protocol::DespawnEntity(_) |
        Protocol::UserAuthenticate(_) |
        Protocol::SpawnEntity(_) => { Channel::Reliable }

        Protocol::PartialChunkUpdate(_) => { Channel::Block }
    }
}

fn has_resource<T: Resource>(resource: Option<Res<T>>) -> ShouldRun {
    match resource.is_some() {
        true => ShouldRun::Yes,
        false => ShouldRun::No,
    }
}

mod client {
    use std::ops::{Deref, DerefMut};
    use bevy::prelude::*;
    use renet::{RenetClient, RenetError};
    use rc_protocol::constants::UserId;
    use rc_protocol::types::{ReceivePacket, SendPacket};
    use crate::*;

    pub struct RenetClientPlugin;

    impl Plugin for RenetClientPlugin {
        fn build(&self, app: &mut App) {
            use bevy::prelude::CoreStage::*;

            app
                .add_event::<RenetError>()
                .add_system_to_stage(PreUpdate, update_system
                    .with_run_criteria(has_resource::<Client>)
                )
                .add_system_to_stage(PreUpdate, read_packets_system
                    .after(update_system)
                    .with_run_criteria(has_resource::<Client>)
                )
                .add_system_to_stage(PostUpdate, write_packets_system
                    .before(send_packets_system)
                    .with_run_criteria(has_resource::<Client>)
                )
                .add_system_to_stage(PostUpdate, send_packets_system
                    .with_run_criteria(has_resource::<Client>)
                );
        }
    }

    fn update_system(
        mut client: ResMut<Client>,
        mut renet_error: EventWriter<RenetError>,
        time: Res<Time>,
    ) {
        if let Err(e) = client.update(time.delta()) {
            renet_error.send(e);
        }
    }

    fn read_packets_system(
        mut client: ResMut<Client>,
        mut recv: EventWriter<ReceivePacket>,
    ) {
        fn send(client: &mut Client, recv: &mut EventWriter<ReceivePacket>, channel: Channel) {
            while let Some(bytes) = client.receive_message(channel) {
                let protocol: Protocol = bincode::deserialize(&bytes).unwrap();
                recv.send(ReceivePacket(protocol, UserId(client.client_id())));
            }
        }
        send(&mut client, &mut recv, Channel::Unreliable);
        send(&mut client, &mut recv, Channel::Reliable);
        send(&mut client, &mut recv, Channel::Block);
    }

    fn write_packets_system(
        mut client: ResMut<Client>,
        mut to_send: EventReader<SendPacket>,
    ) {
        to_send
            .iter()
            .for_each(|v: &SendPacket| {
                let ser = bincode::serialize(&v.0).unwrap();
                let channel = get_channel(&v.0);
                client.send_message(channel, ser);
            })
    }

    fn send_packets_system(
        mut client: ResMut<Client>,
        mut renet_error: EventWriter<RenetError>,
    ) {
        if let Err(e) = client.send_packets() {
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

pub mod server {
    use std::ops::{Deref, DerefMut};
    use bevy::prelude::*;
    use renet::{RenetError, RenetServer, ServerEvent};
    use rc_protocol::constants::UserId;
    use rc_protocol::types::{ReceivePacket, SendPacket};
    use crate::*;

    pub struct RenetServerPlugin;

    impl Plugin for RenetServerPlugin {
        fn build(&self, app: &mut App) {
            use bevy::prelude::CoreStage::*;
            app
                .add_event::<RenetError>()
                .add_event::<ServerEvent>()
                .add_system_to_stage(PreUpdate, update_system
                    .with_run_criteria(has_resource::<Server>)
                )
                .add_system_to_stage(PreUpdate, read_packets_system
                    .after(update_system)
                    .with_run_criteria(has_resource::<Server>)
                )
                .add_system_to_stage(PostUpdate, write_packets_system
                    .before(send_packets_system)
                    .with_run_criteria(has_resource::<Server>)
                )
                .add_system_to_stage(PostUpdate, send_packets_system
                    .with_run_criteria(has_resource::<Server>)
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
            renet_error.send(RenetError::IO(e));
        }

        while let Some(event) = server.get_event() {
            server_events.send(event);
        }
    }

    fn read_packets_system(
        mut server: ResMut<Server>,
        mut recv: EventWriter<ReceivePacket>,
    ) {
        server
            .clients_id()
            .iter()
            .for_each(|&user_id| {
                fn send(server: &mut Server, recv: &mut EventWriter<ReceivePacket>, user_id: u64, channel: Channel) {
                    while let Some(bytes) = server.receive_message(user_id, channel) {
                        let protocol: Protocol = bincode::deserialize(&bytes).unwrap();
                        recv.send(ReceivePacket(protocol, UserId(user_id)));
                    }
                }
                send(&mut server, &mut recv, user_id, Channel::Unreliable);
                send(&mut server, &mut recv, user_id, Channel::Reliable);
                send(&mut server, &mut recv, user_id, Channel::Block);
            })
    }

    fn write_packets_system(
        mut server: ResMut<Server>,
        mut to_send: EventReader<SendPacket>,
    ) {
        to_send
            .iter()
            .for_each(|v| {
                let ser = bincode::serialize(&v.0).unwrap();
                let channel = get_channel(&v.0);
                server.send_message(v.1.0, channel, ser);
            })
    }

    fn send_packets_system(
        mut server: ResMut<Server>,
        mut renet_error: EventWriter<RenetError>,
    ) {
        if let Err(e) = server.send_packets() {
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
