use std::collections::HashSet;
use std::fs;
use crate::game::inventory::Inventory;
use crate::game::transform::Transform;
use bevy::ecs::change_detection::ResMut;
use bevy::ecs::event::EventReader;
use bevy::ecs::prelude::{Commands, EventWriter};
use bevy::ecs::system::Query;
use bevy::log::info;
use nalgebra::Vector3;
use rc_shared::game_objects::{GameObjectData, PlayerGameObjectData};
use std::sync::atomic::Ordering;
use crate::events::authorize::AuthorizationEvent;
use crate::game::world::data::GAME_OBJECT_ID_COUNTER;
use rc_shared::helpers::global_to_local_position;
use crate::systems::chunk::ChunkSystem;
use crate::{TransportSystem, WorldData};
use rc_shared::constants::GameObjectId;
use crate::systems::game_object::spawn::SpawnGameObjectRequest;
use rc_networking::types::SendPacket;
use crate::events::join::PlayerSpawnEvent;
use crate::game::world::deserialized_player::DeserializedPlayerData;
use crate::game::world::WORLD_SPAWN_LOCATION;

pub fn authorization_event(
    mut event_reader: EventReader<AuthorizationEvent>,
    global: ResMut<WorldData>,
    mut transport: ResMut<TransportSystem>,
    send_packet: EventWriter<SendPacket>,
    mut commands: Commands,
    transforms: Query<&Transform>,
    mut chunk_system: ResMut<ChunkSystem>,
    mut spawn_game_object: EventWriter<SpawnGameObjectRequest>,
    mut player_spawn_event: EventWriter<PlayerSpawnEvent>
) {
    for client in event_reader.read() {
        // Load player data
        let path = format!("./world/players/{}", client.user_id.0);
        let (transform, mut inventory) = if fs::exists(&path).unwrap() {
            let player_data = serde_json::from_str::<DeserializedPlayerData>(&
                fs::read_to_string(&path).unwrap()
            ).unwrap();

            let mut transform = Transform::from_translation(player_data.position);

            transform.rotation = player_data.rotation;

            (transform, player_data.inventory)
        } else {
            (Transform::from_translation(WORLD_SPAWN_LOCATION), Inventory::default())
        };

        // Recompute inventory
        inventory.dirty = true;

        info!("Player {:?} logged in. Sending chunks.", client.user_id);

        // Create new game_object for player
        let game_object_id = GameObjectId(GAME_OBJECT_ID_COUNTER.fetch_add(1, Ordering::SeqCst));

        // Store player game_object
        let game_user = transport
            .clients
            .get_mut(&client.user_id)
            .unwrap();

        game_user.game_object_id = Some(game_object_id);

        let entity = commands.spawn(inventory).id();

        spawn_game_object.send(SpawnGameObjectRequest {
            transform,
            id: game_object_id,
            entity: Some(entity),
            data: GameObjectData::Player(PlayerGameObjectData {
                user_id: client.user_id,
                username: game_user.name.clone(),
            })
        });

        chunk_system.user_loaded_chunks.insert(client.user_id, HashSet::default());
        chunk_system.user_loaded_columns.insert(client.user_id, HashSet::default());

        let chunks = global.chunks.keys();

        let (player_chunk, _) = global_to_local_position(Vector3::new(
            transform.position.x as i32,
            transform.position.y as i32,
            transform.position.z as i32,
        ));

        let chunk_load_radius = 5;

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

        player_spawn_event.send(PlayerSpawnEvent {
            id: client.user_id,
        });
    }
}
