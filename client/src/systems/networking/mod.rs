use crate::systems::networking::chunk::network_chunk_sync;
use crate::systems::networking::location_sync::{
    network_location_sync, LastNetworkRotationSync, LastNetworkTranslationSync,
};
use crate::systems::networking::messages::messages_update;

use bevy::prelude::*;
use bevy::prelude::{info, Entity, Vec3};

use rc_networking::constants::EntityId;

use crate::state::AppState;


use rc_networking::client::{NetworkingClient, QuinnClientPlugin};

use std::collections::HashMap;
use std::net::{SocketAddr};


mod chunk;
mod location_sync;
mod messages;

pub struct ClientNetworkingPlugin;

impl Plugin for ClientNetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(QuinnClientPlugin)
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

pub fn connect_to_server(mut client: ResMut<NetworkingClient>) {
    let server_addr: SocketAddr = ([127, 0, 0, 1], 25568).into();

    client.connect(server_addr);

    info!("Connecting to server on {}", server_addr);
}

#[derive(Resource)]
pub struct NetworkingSystem {
    entity_mapping: HashMap<EntityId, Entity>,
}

impl Default for NetworkingSystem {
    fn default() -> Self {
        NetworkingSystem {
            entity_mapping: Default::default(),
        }
    }
}
