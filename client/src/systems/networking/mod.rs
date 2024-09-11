use crate::systems::networking::chunk::network_chunk_sync;
use crate::systems::networking::location_sync::{
    LastNetworkRotationSync, LastNetworkTranslationSync, network_location_sync,
};
use crate::systems::networking::messages::messages_update;
use bevy::prelude::*;
use rc_networking::client::NetworkingClientPlugin;
use rc_shared::constants::{GameObjectId, UserId};
use std::collections::HashMap;
use crate::authentication::GameAuthentication;
use crate::systems::connection::connect::{accept_server_connection_intent, connect_to_server, ConnectToServerIntent, PendingServerConnection};

mod chunk;
mod location_sync;
pub mod messages;

pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(NetworkingClientPlugin)
            .add_systems(Update, messages_update)
            .add_systems(Update, network_location_sync)
            .add_systems(Update, network_chunk_sync)
            .insert_resource(LastNetworkTranslationSync(Vec3::default()))
            .insert_resource(LastNetworkRotationSync(Quat::default()));

        let authentication = app.world().get_resource::<GameAuthentication>().unwrap();
        let system = NetworkingSystem::new(authentication.account_id);

        app.insert_resource(system);
    }
}

#[derive(Resource)]
pub struct NetworkingSystem {
    pub entity_mapping: HashMap<GameObjectId, Entity>,
    pub user_id: UserId
}

impl NetworkingSystem {
    pub fn new(user_id: u64) -> Self {
        NetworkingSystem {
            user_id: UserId(user_id),
            entity_mapping: Default::default()
        }
    }
}