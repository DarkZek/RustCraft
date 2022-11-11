use crate::services::networking::chunk::network_chunk_sync;
use crate::services::networking::events::authorization::AuthorizationEvent;
use crate::services::networking::events::connection::ConnectionEvent;
use crate::services::networking::events::disconnect::DisconnectionEvent;
use crate::services::networking::location_sync::{
    network_location_sync, LastNetworkRotationSync, LastNetworkTranslationSync,
};
use crate::services::networking::messages::messages_update;
use crate::services::networking::transport::connection::{connection_upkeep, send_packets};
use crate::services::networking::transport::listener::ClientListener;
use crate::services::networking::transport::packet::{ReceivePacket, SendPacket};
use bevy::app::{AppExit, CoreStage};
use bevy::ecs::schedule::StageLabel;
use bevy::prelude::*;
use bevy::prelude::{info, Entity, ResMut, SystemSet, Vec3};

use rc_protocol::constants::EntityId;

use crate::state::AppState;
use rc_protocol::protocol::serverbound::disconnect::Disconnect;
use rc_protocol::protocol::Protocol;
use rc_protocol::stream::GameStream;
use std::collections::HashMap;
use std::net::{IpAddr, TcpStream};

mod chunk;
mod events;
mod location_sync;
mod messages;
pub mod transport;

pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(send_packets)
            .add_system(connection_upkeep)
            .insert_resource(ClientListener::new().unwrap())
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

pub fn connect_to_server(mut system: ResMut<ClientListener>) {
    let ip = "127.0.0.1";
    let port = 25567;

    let stream = TcpStream::connect(format!("{}:{}", ip, port)).unwrap();

    stream.set_nonblocking(true);

    info!("Connecting to server on {}:{}", ip, port);

    system.stream = Some(GameStream::new(stream));
}

pub struct TransportSystem {
    entity_mapping: HashMap<EntityId, Entity>,
}

impl Default for TransportSystem {
    fn default() -> Self {
        TransportSystem {
            entity_mapping: Default::default(),
        }
    }
}

#[allow(unused_must_use)]
pub fn detect_shutdowns(shutdown: EventReader<AppExit>, mut system: ResMut<ClientListener>) {
    if !shutdown.is_empty() {
        println!("Sending disconnect to server");
        // Tell server we're quitting
        if let Some(mut val) = system.stream.take() {
            val.write_packet(&Protocol::Disconnect(Disconnect::new(0)));
        }
    }
}
