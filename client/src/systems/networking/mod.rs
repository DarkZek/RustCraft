use crate::state::AppState;
use crate::systems::networking::chunk::network_chunk_sync;
use crate::systems::networking::location_sync::{
    network_location_sync, LastNetworkRotationSync, LastNetworkTranslationSync,
};
use crate::systems::networking::messages::messages_update;
use bevy::log::info;
use bevy::prelude::*;
use rc_networking::client::{NetworkingClient, NetworkingClientPlugin};
use rc_shared::constants::{GameObjectId, UserId};
use std::collections::HashMap;
use std::net::SocketAddr;

mod chunk;
mod location_sync;
mod messages;

pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(NetworkingClientPlugin)
            // Once the game is in the Main Menu connect to server as we have no main screen yet
            .add_systems(OnEnter(AppState::Connecting), connect_to_server)
            .add_systems(Update, messages_update)
            .add_systems(Update, network_location_sync)
            .add_systems(Update, network_chunk_sync)
            .insert_resource(LastNetworkTranslationSync(Vec3::default()))
            .insert_resource(LastNetworkRotationSync(Quat::default()))
            .insert_resource(NetworkingSystem::default());
    }
}

/// Connects to the local server instance
pub fn connect_to_server(
    mut client: ResMut<NetworkingClient>,
    networking_system: Res<NetworkingSystem>
) {
    let server_addr = "https://test.marshalldoes.dev:25568".parse().unwrap();

    info!("Connecting to server on {}", server_addr);

    client.connect(server_addr, networking_system.user_id.0);
}

#[derive(Resource)]
pub struct NetworkingSystem {
    pub entity_mapping: HashMap<GameObjectId, Entity>,
    pub user_id: UserId
}

impl Default for NetworkingSystem {
    fn default() -> Self {

        let user_id = env!("USER_ID").parse::<u64>().unwrap();

        NetworkingSystem {
            entity_mapping: Default::default(),
            user_id: UserId(user_id),
        }
    }
}
