use crate::game::chunk::ChunkData;
use crate::{App, TransportSystem, WorldData};
use bevy::ecs::system;
use bevy::prelude::*;
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

const CHUNK_REQUEST_TIMESTEP: f64 = 1.0 / 60.0;

pub struct ChunkPlugin;

impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChunkSystem {
            generating_chunks: Default::default(),
            requesting_chunks: Default::default(),
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
}

pub fn request_chunks(
    mut system: ResMut<ChunkSystem>,
    mut world: ResMut<WorldData>,
    mut send_packets: EventWriter<SendPacket>,
    transport: Res<TransportSystem>,
    mut transforms: Query<&mut Transform>,
) {
    let mut to_generate = Vec::new();
    for (user, chunks) in &mut system.requesting_chunks {
        if chunks.len() == 0 {
            continue;
        }

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

        let chunk_pos = chunks.pop().unwrap();

        // Check if chunk exists
        if world.chunks.contains_key(&chunk_pos) {
            let chunk = world.chunks.get(&chunk_pos).unwrap();
            let partial_updates = FullChunkUpdate::new(
                chunk.world,
                chunk.position.x,
                chunk.position.y,
                chunk.position.z,
            )
            .to_partial();
            // Send to user
            for partial in partial_updates.iter() {
                send_packets.send(SendPacket(
                    Protocol::PartialChunkUpdate(partial.clone()),
                    *user,
                ));
            }
        } else {
            if !to_generate.contains(&chunk_pos) {
                to_generate.push(chunk_pos);
            }
            chunks.push(chunk_pos);
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
