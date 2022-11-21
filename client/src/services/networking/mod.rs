use crate::services::networking::chunk::network_chunk_sync;
use crate::services::networking::events::authorization::AuthorizationEvent;
use crate::services::networking::events::connection::ConnectionEvent;
use crate::services::networking::events::disconnect::DisconnectionEvent;
use crate::services::networking::location_sync::{
    network_location_sync, LastNetworkRotationSync, LastNetworkTranslationSync,
};
use crate::services::networking::messages::messages_update;

use bevy::prelude::*;
use bevy::prelude::{info, Entity, SystemSet, Vec3};

use rc_protocol::constants::EntityId;

use crate::state::AppState;
use rc_networking::renet::ClientAuthentication;
use rc_networking::*;

use rc_protocol::types::{ReceivePacket, SendPacket};
use std::collections::HashMap;
use std::net::{SocketAddr, UdpSocket};
use std::time::SystemTime;

mod chunk;
mod events;
mod location_sync;
mod messages;

pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RenetClientPlugin)
            // Once the game is in the Main Menu connect to server as we have no main screen yet
            .add_system_set(
                SystemSet::on_enter(AppState::Connecting).with_system(connect_to_server),
            )
            .add_system(messages_update)
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

pub fn connect_to_server(mut commands: Commands) {
    let bind_addr: SocketAddr = ([127, 0, 0, 1], 0).into();
    let server_addr: SocketAddr = ([127, 0, 0, 1], 25568).into();
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let socket = UdpSocket::bind(bind_addr).unwrap();
    let user_id = current_time.as_millis() as u64;
    let client = rc_networking::renet::RenetClient::new(
        current_time,
        socket,
        user_id,
        get_renet_connection_config(),
        ClientAuthentication::Secure {
            connect_token: get_simple_connect_token(user_id, vec![server_addr]),
        },
    )
    .unwrap();

    commands.insert_resource(Client(client));

    info!("Connecting to server on {}", server_addr);
}

#[derive(Resource)]
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
