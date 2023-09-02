use crate::game::chunk::ChunkData;
use crate::{App, TransportSystem, WorldData};
use bevy::ecs::system;
use bevy::prelude::KeyCode::Sysrq;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy::utils::petgraph::visit::Walker;
use nalgebra::Vector3;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelRefIterator;
use rc_client::helpers::from_bevy_vec3;
use rc_client::helpers::TextureSubdivisionMethod::Full;
use rc_networking::constants::{UserId, CHUNK_SIZE};
use rc_networking::protocol::clientbound::chunk_update::FullChunkUpdate;
use rc_networking::protocol::Protocol;
use rc_networking::types::{ReceivePacket, SendPacket};
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

const MAX_OUTSTANDING_CHUNK_REQUESTS: usize = 20;

pub struct ChunkPlugin;

impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChunkSystem {
            generating_chunks: Default::default(),
            requesting_chunks: Default::default(),
            chunk_outstanding_requests: Default::default(),
        })
        .add_systems(Update, get_chunk_requests)
        .add_systems(Update, request_chunks)
        .add_systems(Update, generate_chunks);
    }
}

#[derive(Resource)]
pub struct ChunkSystem {
    pub generating_chunks: Vec<Vector3<i32>>,
    pub requesting_chunks: HashMap<UserId, Vec<Vector3<i32>>>,
    // How many chunk requests have been send and are waiting acknowledgement
    pub chunk_outstanding_requests: HashMap<UserId, usize>,
}

pub fn request_chunks(
    mut system: ResMut<ChunkSystem>,
    mut world: ResMut<WorldData>,
    mut send_packets: EventWriter<SendPacket>,
    mut receive_packets: EventReader<ReceivePacket>,
    transport: Res<TransportSystem>,
    mut transforms: Query<&mut Transform>,
) {
    // Manually split because ResMut removes default borrow splitting
    let ChunkSystem {
        requesting_chunks,
        chunk_outstanding_requests,
        generating_chunks,
    } = &mut *system;

    // Remove packets received
    for packet in receive_packets.iter() {
        if let Protocol::AcknowledgeChunk(data) = packet.0 {
            // Remove one
            *chunk_outstanding_requests.get_mut(&packet.1).unwrap() =
                (chunk_outstanding_requests.get(&packet.1).unwrap() - 1).max(0);
        }
    }

    let mut to_generate = Vec::new();
    for (user, chunks) in requesting_chunks {
        if let Some(Some(entity)) = transport.clients.get(&user).map(|v| v.entity) {
            if let Ok(transform) = transforms.get(entity) {
                let pos = from_bevy_vec3(transform.translation);

                chunks.sort_by(|a, b| {
                    let a_dist: i32 = (&pos - (a.cast::<f32>() * CHUNK_SIZE as f32)).norm() as i32;
                    let b_dist: i32 = (&pos - (b.cast::<f32>() * CHUNK_SIZE as f32)).norm() as i32;
                    a_dist.cmp(&b_dist)
                });
            }
        }

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
                        chunk.world,
                        chunk.position.x,
                        chunk.position.y,
                        chunk.position.z,
                    )),
                    *user,
                ));

                println!("Sending chunk");

                // Increase outstanding requests
                *chunk_outstanding_requests.get_mut(user).unwrap() += 1;
            } else {
                if !to_generate.contains(&chunk_pos) {
                    to_generate.push(chunk_pos);
                }
                chunks.push(chunk_pos);
                break;
            }
        }
    }
    system.generating_chunks.append(&mut to_generate);
}

pub fn generate_chunks(
    mut system: ResMut<ChunkSystem>,
    mut world: ResMut<WorldData>,
    mut send_packets: EventWriter<SendPacket>,
) {
    // Generate X chunks per loop
    let chunks_per_loop = system.generating_chunks.len().min(5 as usize);

    let build_chunks = system
        .generating_chunks
        .drain(0..chunks_per_loop)
        .collect::<Vec<(Vector3<i32>)>>();

    #[cfg(not(target_arch = "wasm32"))]
    let iterator = build_chunks.par_iter();
    #[cfg(target_arch = "wasm32")]
    let iterator = build_chunks.iter();

    let chunks = iterator
        .map(|pos| ChunkData::generate(*pos))
        .collect::<Vec<ChunkData>>();

    for chunk in chunks {
        world.chunks.insert(chunk.position, chunk);
    }
}

// Respond to get chunk requests
pub fn get_chunk_requests(
    mut request: EventReader<ReceivePacket>,
    mut system: ResMut<ChunkSystem>,
) {
    for packet in request.iter() {
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
