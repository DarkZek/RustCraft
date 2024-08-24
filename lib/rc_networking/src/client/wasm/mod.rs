use std::net::SocketAddr;
use bevy::prelude::{App, Plugin, Resource};
use crate::events::connection::NetworkConnectionEvent;
use crate::events::disconnect::NetworkDisconnectionEvent;
use crate::types::{ReceivePacket, SendPacket};

pub struct QuinnClientPlugin;

impl Plugin for QuinnClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ReceivePacket>()
            .add_event::<SendPacket>()
            .add_event::<NetworkConnectionEvent>()
            .add_event::<NetworkDisconnectionEvent>()
            .insert_resource(NetworkingClient {});
    }
}

#[derive(Resource)]
pub struct NetworkingClient {

}

impl NetworkingClient {
    pub fn connect(&mut self, addr: SocketAddr, user_id: u64) {

    }
}