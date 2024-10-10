use crate::game::chunk::ChunkData;
use crate::{TransportSystem, WorldData};

use bevy::prelude::*;

use nalgebra::{Vector2, Vector3};
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelRefIterator;
use rc_shared::constants::UserId;
use rc_networking::events::disconnect::NetworkDisconnectionEvent;
use rc_networking::protocol::clientbound::chunk_update::FullChunkUpdate;
use rc_networking::protocol::Protocol;
use rc_networking::types::{ReceivePacket, SendPacket};
use std::collections::{HashMap, HashSet};
use rc_networking::protocol::clientbound::chunk_column_update::ChunkColumnUpdate;
use crate::config::{ServerConfig, WorldType};
use crate::game::world::serialized::DeserializedChunkData;

const MAX_OUTSTANDING_CHUNK_REQUESTS: usize = 80;
const CHUNKS_GENERATED_PER_TICK: usize = 40;

pub struct ChunkPlugin;

impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChunkSystem {
            user_loaded_chunks: Default::default(),
            generating_chunks: Default::default(),
            requesting_chunks: Default::default(),
            chunk_outstanding_requests: Default::default(),
            user_loaded_columns: Default::default()
        })
        .add_systems(Update, handle_disconnections)
        .add_systems(Update, get_chunk_requests)
        .add_systems(Update, request_chunks)
        .add_systems(Update, generate_chunks);
    }
}

#[derive(Resource)]
pub struct ChunkSystem {
    pub user_loaded_chunks: HashMap<UserId, HashSet<Vector3<i32>>>,
    pub user_loaded_columns: HashMap<UserId, HashSet<Vector2<i32>>>,
    pub generating_chunks: HashSet<Vector3<i32>>,
    pub requesting_chunks: HashMap<UserId, Vec<Vector3<i32>>>,
    // How many chunk requests have been send and are waiting acknowledgement
    pub chunk_outstanding_requests: HashMap<UserId, usize>,
}

/// Handles chunk requests coming in, and chunk data going out
pub fn request_chunks(
    mut system: ResMut<ChunkSystem>,
    world: Res<WorldData>,
    mut send_packets: EventWriter<SendPacket>,
    mut receive_packets: EventReader<ReceivePacket>,
    transport: Res<TransportSystem>,
    transforms: Query<&mut Transform>,
) {
    // Manually split because ResMut removes default borrow splitting
    let ChunkSystem {
        requesting_chunks,
        chunk_outstanding_requests,
        generating_chunks,
        user_loaded_chunks,
        user_loaded_columns
    } = &mut *system;

    // Remove packets received
    for packet in receive_packets.read() {
        if let Protocol::AcknowledgeChunk(data) = packet.0 {
            // Remove one
            *chunk_outstanding_requests.get_mut(&packet.1).unwrap() =
                (chunk_outstanding_requests.get(&packet.1).unwrap() - 1).max(0);
        }
    }

    for (user, chunks) in requesting_chunks {
        let Some(Some(game_object_id)) = transport.clients.get(&user).map(|v| v.game_object_id) else {
            error!("has no game object");
            continue
        };

        // TODO: order chunk loading once we have sorted out the duplicate transform types
        // let Some(transform) = world
        //     .get_game_object(&game_object_id)
        //     .and_then(|entity| transforms.get(entity).ok()) else {
        //     println!("Has no transform");
        //     continue;
        // };
        //
        // let pos = from_bevy_vec3(transform.translation);
        //
        // chunks.sort_by(|a, b| {
        //     let a_dist: i32 = (&pos - (a.cast::<f32>() * CHUNK_SIZE as f32)).norm() as i32;
        //     let b_dist: i32 = (&pos - (b.cast::<f32>() * CHUNK_SIZE as f32)).norm() as i32;
        //     a_dist.cmp(&b_dist)
        // });

        // Chunks being held until next send cycle
        let mut holding_chunks = Vec::new();

        while chunk_outstanding_requests.get(user).unwrap() < &MAX_OUTSTANDING_CHUNK_REQUESTS {
            // If no chunks left
            let Some(chunk_pos) = chunks.pop() else {
                break;
            };

            // Check if chunk exists
            if world.chunks.contains_key(&chunk_pos) {
                let chunk = world.chunks.get(&chunk_pos).unwrap();

                // Send to user
                send_packets.send(SendPacket(
                    Protocol::FullChunkUpdate(FullChunkUpdate::new(
                        chunk.world.clone(),
                        chunk.position.x,
                        chunk.position.y,
                        chunk.position.z,
                    )),
                    *user,
                ));

                user_loaded_chunks.get_mut(user).unwrap().insert(chunk.position);

                // Send chunk column
                let columns = user_loaded_columns.get_mut(user).unwrap();
                let position = Vector2::new(chunk.position.x, chunk.position.z);
                if !columns.contains(&position) {
                    // Send chunk column update
                    columns.insert(position);

                    if let Some(data) = world.chunks_columns.get(&position) {
                        send_packets.send(SendPacket(
                            Protocol::ChunkColumnUpdate(ChunkColumnUpdate::new(
                                position,
                                data.clone()
                            )),
                            *user,
                        ));
                    }
                }

                // Increase outstanding acknowledgement requests
                *chunk_outstanding_requests.get_mut(user).unwrap() += 1;
            } else {
                if !generating_chunks.contains(&chunk_pos) {
                    generating_chunks.insert(chunk_pos);
                }

                holding_chunks.push(chunk_pos);
                continue;
            }
        }

        chunks.append(&mut holding_chunks);
    }
}

pub fn generate_chunks(
    mut system: ResMut<ChunkSystem>,
    mut world: ResMut<WorldData>,
    config: Res<ServerConfig>
) {
    // Generate X chunks per loop
    let chunks_per_loop = system
        .generating_chunks
        .len()
        .min(CHUNKS_GENERATED_PER_TICK);

    let build_chunks = system
        .generating_chunks
        .drain()
        .take(chunks_per_loop)
        .collect::<Vec<Vector3<i32>>>();

    #[cfg(not(target_arch = "wasm32"))]
    let iterator = build_chunks.par_iter();
    #[cfg(target_arch = "wasm32")]
    let iterator = build_chunks.iter();

    let chunks = iterator
        .map(|pos| {
            // Try load chunk, or generate
            match WorldData::try_load_chunk(*pos) {
                Ok(Some(chunk)) => Some(chunk.data),
                Ok(None) => None,
                Err(err) => {
                    error!("Error reading chunk data: {:?}", err);
                    None
                }
            }.unwrap_or_else(|| {
                // Generate the chunk
                match config.world_type {
                    WorldType::Regular => ChunkData::generate(*pos),
                    WorldType::Canvas => ChunkData::generate_canvas(*pos)
                }
            })
        })
        .collect::<Vec<ChunkData>>();

    for chunk in chunks {
        world.insert_chunk(chunk);
    }
}

// Respond to get chunk requests
pub fn get_chunk_requests(
    mut request: EventReader<ReceivePacket>,
    mut system: ResMut<ChunkSystem>,
) {
    for packet in request.read() {
        if let Protocol::RequestChunk(request) = packet.0 {
            let pos = Vector3::new(request.x, request.y, request.z);

            system
                .requesting_chunks
                .entry(packet.1)
                .or_insert_with(|| vec![])
                .push(pos);
        }
    }
}

fn handle_disconnections(
    mut network_disconnection_events: EventReader<NetworkDisconnectionEvent>,
    mut system: ResMut<ChunkSystem>,
) {
    for disconnection in network_disconnection_events.read() {
        debug!(
            "Removed user {:?} from requesting chunks list",
            disconnection.client
        );
        system.requesting_chunks.remove(&disconnection.client);
    }
}
