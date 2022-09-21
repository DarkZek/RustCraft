use std::thread;
use std::time::Duration;
use bevy_ecs::prelude::Query;
use bevy_ecs::system::ResMut;
use bevy_log::info;
use bevy_testing_protocol::channels::Channels;
use bevy_testing_protocol::protocol::{Protocol};
use naia_bevy_server::Server;
use nalgebra::Vector3;
use crate::game::chunk::ChunkData;
use crate::resources::Global;

pub fn tick(
    mut server: Server<Protocol, Channels>,
   // mut position_query: Query<&mut Vector3<f32>>,
) {
    // All game logic should happen here, on a tick event

    // Update scopes of entities
    for (_, user_key, entity) in server.scope_checks() {

        // This indicates the Entity should be in this scope.
        server.user_scope(&user_key).include(&entity);
    }

    server.send_all_updates();
}