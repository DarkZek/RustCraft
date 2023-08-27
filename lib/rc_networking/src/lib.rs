pub mod constants;
pub mod protocol;
pub mod types;

use bevy::prelude::{Res, Resource};
use protocol::Protocol;
use std::net::SocketAddr;
use std::time::{Duration, SystemTime};

pub use client::*;
pub use server::*;

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
        Protocol::PlayerMove(_)
        | Protocol::EntityMoved(_)
        | Protocol::PlayerRotate(_)
        | Protocol::EntityRotated(_) => Channel::Unreliable,

        Protocol::BlockUpdate(_)
        | Protocol::ChatSent(_)
        | Protocol::DespawnEntity(_)
        | Protocol::SpawnEntity(_)
        | Protocol::PartialChunkUpdate(_)
        | Protocol::UpdateLoading(_)
        | Protocol::RequestChunk(_) => Channel::Reliable,

        Protocol::FullChunkUpdate(_) => Channel::Block,
    }
}

fn has_resource<T: Resource>(resource: Option<Res<T>>) -> bool {
    match resource.is_some() {
        true => true,
        false => false,
    }
}

mod client {
    use crate::constants::UserId;
    use crate::types::{ReceivePacket, SendPacket};
    use crate::*;
    use bevy::app::AppExit;
    use bevy::prelude::*;
    use std::ops::{Deref, DerefMut};

    pub struct RenetClientPlugin;

    impl Plugin for RenetClientPlugin {
        fn build(&self, app: &mut App) {
            // TODO
        }
    }

    fn update_system(mut client: ResMut<Client>, time: Res<Time>) {
        // TODO
    }

    fn read_packets_system(mut client: ResMut<Client>, mut recv: EventWriter<ReceivePacket>) {
        fn send(client: &mut Client, recv: &mut EventWriter<ReceivePacket>, channel: Channel) {
            //while let Some(bytes) = client.receive_message(channel) {
            //    let protocol: Protocol = bincode::deserialize(&bytes).unwrap();
            // TODO
            //}
        }
        send(&mut client, &mut recv, Channel::Unreliable);
        send(&mut client, &mut recv, Channel::Reliable);
        send(&mut client, &mut recv, Channel::Block);
    }

    fn write_packets_system(mut client: ResMut<Client>, mut to_send: EventReader<SendPacket>) {
        to_send.iter().for_each(|v: &SendPacket| {
            let ser = bincode::serialize(&v.0).unwrap();
            let channel = get_channel(&v.0);
            // TODO
        })
    }

    fn detect_shutdown_system(mut client: ResMut<Client>, mut bevy_shutdown: EventReader<AppExit>) {
        for _ in bevy_shutdown.iter() {
            info!("Shutting down server");
            // TODO
        }
    }

    fn send_packets_system(
        mut client: ResMut<Client>,
        //mut renet_error: EventWriter<NetcodeTransportError>,
    ) {
        // TODO
    }

    #[derive(Resource)]
    pub struct Client(pub usize);

    impl Deref for Client {
        type Target = usize;

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
    use crate::constants::UserId;
    use crate::types::{ReceivePacket, SendPacket};
    use crate::*;
    use bevy::app::AppExit;
    use bevy::prelude::*;
    use std::ops::{Deref, DerefMut};

    pub struct RenetServerPlugin;

    impl Plugin for RenetServerPlugin {
        fn build(&self, app: &mut App) {
            // TODO
        }
    }

    fn update_system(
        mut server: ResMut<Server>,
        //mut renet_error: EventWriter<NetcodeTransportError>,
        //mut server_events: EventWriter<ServerEvent>,
        time: Res<Time>,
    ) {
        // TODO
    }

    fn read_packets_system(mut server: ResMut<Server>, mut recv: EventWriter<ReceivePacket>) {
        // server.clients_id().iter().for_each(|&user_id| {
        //     fn send(
        //         server: &mut Server,
        //         recv: &mut EventWriter<ReceivePacket>,
        //         user_id: u64,
        //         channel: Channel,
        //     ) {
        //         // while let Some(bytes) = server.receive_message(user_id, channel) {
        //         //     let protocol: Protocol = bincode::deserialize(&bytes).unwrap();
        //         //     recv.send(ReceivePacket(protocol, UserId(user_id)));
        //         // }
        //     }
        //     send(&mut server, &mut recv, user_id, Channel::Unreliable);
        //     send(&mut server, &mut recv, user_id, Channel::Reliable);
        //     send(&mut server, &mut recv, user_id, Channel::Block);
        // })
    }

    fn write_packets_system(mut server: ResMut<Server>, mut to_send: EventReader<SendPacket>) {
        to_send.iter().for_each(|v| {
            let ser = bincode::serialize(&v.0).unwrap();
            let channel = get_channel(&v.0);
            // TODO
        })
    }

    fn detect_shutdown_system(mut server: ResMut<Server>, mut bevy_shutdown: EventReader<AppExit>) {
        for _ in bevy_shutdown.iter() {
            info!("Shutting down server");
            // TODO
        }
    }

    fn send_packets_system(
        mut server: ResMut<Server>,
        //mut renet_error: EventWriter<NetcodeTransportError>,
    ) {
        // TODO
    }

    #[derive(Resource)]
    pub struct Server(pub usize);

    impl Deref for Server {
        type Target = usize;

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
