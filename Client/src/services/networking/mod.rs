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
use crate::{info, services, App, EventReader, Plugin, Quat, SystemStage};
use bevy::ecs::schedule::StageLabel;
use bevy::prelude::{ResMut, Vec3};
use nalgebra::Vector3;
use rustcraft_protocol::protocol::serverbound::authenticate::UserAuthenticate;
use rustcraft_protocol::protocol::Protocol;
use std::net::TcpStream;

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
            .insert_resource(ClientListener::new("127.0.0.1".parse().unwrap(), 8000).unwrap())
            .add_stage(ClientState::Networking, SystemStage::single_threaded())
            .add_system_to_stage(ClientState::Networking, connect_event)
            .add_system_to_stage(ClientState::Networking, disconnect_event)
            .add_system_to_stage(ClientState::Networking, messages_update)
            .add_system(network_location_sync)
            .add_event::<ReceivePacket>()
            .add_event::<SendPacket>()
            .add_event::<ConnectionEvent>()
            .add_event::<DisconnectionEvent>()
            .add_event::<AuthorizationEvent>()
            .add_system(network_chunk_sync)
            .insert_resource(LastNetworkTranslationSync(Vec3::default()))
            .insert_resource(LastNetworkRotationSync(Quat::default()))
            .insert_resource(TransportSystem::default());
    }
}

pub struct TransportSystem {}

impl Default for TransportSystem {
    fn default() -> Self {
        TransportSystem {}
    }
}

#[derive(StageLabel)]
enum ClientState {
    Networking,
}

pub fn connect_event(client: EventReader<ConnectionEvent>) {
    if client.is_empty() {
        return;
    }
    info!("Client connected to");
}

pub fn disconnect_event(client: EventReader<DisconnectionEvent>) {
    if client.is_empty() {
        return;
    }
    info!("Client disconnected from");
}
