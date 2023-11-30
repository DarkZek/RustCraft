use crate::game::transform::Transform;
use bevy::ecs::change_detection::ResMut;
use bevy::ecs::event::EventReader;
use bevy::ecs::prelude::{Commands, EventWriter};
use bevy::ecs::system::Query;
use bevy::log::info;

use nalgebra::Vector3;
use std::sync::atomic::Ordering;

use crate::events::authorize::AuthorizationEvent;
use crate::game::world::data::GAME_OBJECT_ID_COUNTER;
use crate::helpers::global_to_local_position;
use crate::systems::chunk::ChunkSystem;
use crate::{TransportSystem, WorldData};
use rc_networking::constants::GameObjectId;


use crate::systems::game_object::spawn::SpawnGameObjectRequest;


use rc_networking::types::SendPacket;

pub fn authorization_event(
    mut event_reader: EventReader<AuthorizationEvent>,
    global: ResMut<WorldData>,
    mut transport: ResMut<TransportSystem>,
    send_packet: EventWriter<SendPacket>,
    commands: Commands,
    transforms: Query<&Transform>,
    mut chunk_system: ResMut<ChunkSystem>,
    mut spawn_game_object: EventWriter<SpawnGameObjectRequest>,
) {
    for client in event_reader.read() {
        info!("Player {:?} logged in. Sending chunks.", client.user_id);

        let transform = Transform::default();

        // Create new game_object for player
        let game_object_id = GameObjectId(GAME_OBJECT_ID_COUNTER.fetch_add(1, Ordering::SeqCst));

        // Store player game_object
        transport
            .clients
            .get_mut(&client.user_id)
            .unwrap()
            .game_object_id = game_object_id;

        spawn_game_object.send(SpawnGameObjectRequest {
            transform,
            id: Some(game_object_id),
            object_type: 0,
        });

        let chunks = global.chunks.keys();

        let (player_chunk, _) = global_to_local_position(Vector3::new(
            transform.position.x as i32,
            transform.position.y as i32,
            transform.position.z as i32,
        ));

        let chunk_load_radius = 3;

        // Load chunks around player
        for x in (player_chunk.x - chunk_load_radius)..(player_chunk.x + chunk_load_radius) {
            for y in (player_chunk.y - chunk_load_radius)..(player_chunk.y + chunk_load_radius) {
                for z in (player_chunk.z - chunk_load_radius)..(player_chunk.z + chunk_load_radius)
                {
                    chunk_system
                        .requesting_chunks
                        .entry(client.user_id)
                        .or_insert_with(|| vec![])
                        .push(Vector3::new(x, y, z));
                }
            }
        }

        chunk_system
            .chunk_outstanding_requests
            .insert(client.user_id, 0);

        // List this user as still loading in content, so we know to send them a packet to close the loading screen once chunks have been sent
        transport.initialising_clients.insert(client.user_id);
    }
}
