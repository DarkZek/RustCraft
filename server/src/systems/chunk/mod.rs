use crate::game::chunk::ChunkData;
use crate::{App, WorldData};
use bevy::prelude::*;
use nalgebra::Vector3;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelRefIterator;
use rc_client::helpers::TextureSubdivisionMethod::Full;
use rc_networking::constants::UserId;
use rc_networking::protocol::clientbound::chunk_update::FullChunkUpdate;
use rc_networking::protocol::Protocol;
use rc_networking::types::{ReceivePacket, SendPacket};
use std::collections::HashMap;

pub struct ChunkPlugin;

impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChunkSystem {
            generating_chunks: Default::default(),
        })
        .add_system(get_chunk_requests)
        .add_system(generate_chunks);
    }
}

#[derive(Resource)]
pub struct ChunkSystem {
    generating_chunks: HashMap<Vector3<i32>, Vec<UserId>>,
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
        let packet = FullChunkUpdate::new(
            chunk.world,
            chunk.position.x,
            chunk.position.y,
            chunk.position.z,
        );

        // Send to users
        for user in system.generating_chunks.remove(&chunk.position).unwrap() {
            send_packets.send(SendPacket(Protocol::PartialChunkUpdate(packet), user));
        }

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
            if system.generating_chunks.contains_key(&pos) {
                system
                    .generating_chunks
                    .get_mut(&pos)
                    .unwrap()
                    .push(packet.1);
            } else {
                system.generating_chunks.insert(pos, vec![packet.1]);
            }
        }
    }
}
