use crate::game::chunk::ChunkData;
use crate::{App, WorldData};
use bevy::prelude::*;
use bevy::time::FixedTimestep;
use nalgebra::Vector3;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelRefIterator;
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
        .add_system(get_chunk_requests)
        .add_system_set(
            SystemSet::new()
                // This prints out "hello world" once every second
                .with_run_criteria(FixedTimestep::step(CHUNK_REQUEST_TIMESTEP))
                .with_system(request_chunks),
        )
        .add_system(generate_chunks);
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
) {
    for (user, chunks) in &mut system.requesting_chunks {
        if chunks.len() == 0 {
            continue;
        }

        chunks.sort_by(|a, b| {
            let a_dist: i32 = (&player_pos - (a.cast::<f32>() * CHUNK_SIZE as f32)).norm() as i32;
            let b_dist: i32 = (&player_pos - (b.cast::<f32>() * CHUNK_SIZE as f32)).norm() as i32;
            a_dist.cmp(&b_dist)
        });

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
            system.generating_chunks.push(chunk_pos);
            chunks.push(chunk_users);
        }
    }
}

pub fn generate_chunks(
    mut system: ResMut<ChunkSystem>,
    mut world: ResMut<WorldData>,
    mut send_packets: EventWriter<SendPacket>,
) {
    // Generate X chunks per loop
    let chunks_per_loop = 5;

    let build_chunks = system
        .generating_chunks
        .keys()
        .take(chunks_per_loop)
        .collect::<Vec<(&Vector3<i32>)>>();

    #[cfg(not(target_arch = "wasm32"))]
    let iterator = build_chunks.par_iter();
    #[cfg(target_arch = "wasm32")]
    let iterator = build_chunks.iter();

    let chunks = iterator
        .map(|pos| ChunkData::generate(**pos))
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
