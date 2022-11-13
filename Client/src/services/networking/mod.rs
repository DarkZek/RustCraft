use crate::services::networking::chunk::network_chunk_sync;
use crate::services::networking::events::authorization::AuthorizationEvent;
use crate::services::networking::events::connection::ConnectionEvent;
use crate::services::networking::events::disconnect::DisconnectionEvent;
use crate::services::networking::location_sync::{
    LastNetworkRotationSync, LastNetworkTranslationSync, network_location_sync,
};
use crate::services::networking::messages::messages_update;
use bevy::app::{AppExit, CoreStage};

use bevy::prelude::*;
use bevy::prelude::{Entity, info, ResMut, SystemSet, Vec3};

use rc_protocol::constants::{EntityId, UserId};

use crate::state::AppState;
use rc_protocol::protocol::serverbound::disconnect::Disconnect;
use rc_protocol::protocol::Protocol;
use rc_protocol::stream::GameStream;
use std::collections::HashMap;
use std::net::{IpAddr, TcpStream};
use std::str::FromStr;
use rc_networking::client::ClientSocket;
use rc_protocol::types::{ReceivePacket, SendPacket};
use crate::services::networking::connection::{connection_upkeep, send_packets};

mod chunk;
mod events;
mod location_sync;
mod messages;
pub mod connection;

pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {

        app.add_system(send_packets)
            .add_system(connection_upkeep)
            // Once the game is in the Main Menu connect to server as we have no main screen yet
            .add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(connect_to_server))
            .add_system(messages_update)
            .add_system(network_location_sync)
            .add_event::<ReceivePacket>()
            .add_event::<SendPacket>()
            .add_event::<ConnectionEvent>()
            .add_event::<DisconnectionEvent>()
            .add_event::<AuthorizationEvent>()
            .add_system(network_chunk_sync)
            .add_system_to_stage(CoreStage::PostUpdate, detect_shutdowns)
            .insert_resource(LastNetworkTranslationSync(Vec3::default()))
            .insert_resource(LastNetworkRotationSync(Quat::default()))
            .insert_resource(TransportSystem::default());
    }
}

pub fn connect_to_server(mut system: ResMut<TransportSystem>) {
    let ip = "127.0.0.1";
    let port = 25567;

    let socket = ClientSocket::connect(IpAddr::from_str(ip).unwrap(), port).unwrap();

    info!("Connecting to server on {}:{}", ip, port);

    system.socket = Some(socket);
}

pub struct TransportSystem {
    entity_mapping: HashMap<EntityId, Entity>,
    socket: Option<ClientSocket>,
    
    disconnect: bool
}

impl Default for TransportSystem {

    fn default() -> Self {

        TransportSystem {
            entity_mapping: Default::default(),
            socket: None,
            disconnect: false
        }
    }
}

#[allow(unused_must_use)]
pub fn detect_shutdowns(shutdown: EventReader<AppExit>, mut system: ResMut<TransportSystem>) {
    if !shutdown.is_empty() {
        println!("Sending disconnect to server");
        // Tell server we're quitting
        if let Some(mut val) = system.socket.take() {
            val.send_packet(SendPacket(Protocol::Disconnect(Disconnect::new(0)), UserId(0)));
            val.shutdown();
        }
    }
}
