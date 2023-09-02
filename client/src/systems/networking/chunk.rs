use crate::helpers::global_to_local_position;

use crate::systems::asset::AssetService;
use crate::systems::chunk::ChunkSystem;
use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;

use crate::systems::chunk::builder::{RerenderChunkFlag, RerenderChunkFlagContext};
use nalgebra::Vector3;
use rc_networking::constants::{UserId, CHUNK_SIZE};
use rc_networking::protocol::clientbound::chunk_update::{
    FullChunkUpdate, PartialChunkUpdate, PARTIAL_CHUNK_UPDATE_SIZE,
};
use rc_networking::protocol::serverbound::acknowledge_chunk::AcknowledgeChunk;
use rc_networking::protocol::Protocol;
use rc_networking::types::{ReceivePacket, SendPacket};

#[derive(Default)]
pub struct ChunkSync {
    partial_chunks: HashMap<u64, Vec<PartialChunkUpdate>>,
}

pub fn network_chunk_sync(
    mut event_reader: EventReader<ReceivePacket>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut asset_service: Res<AssetService>,
    mut chunk_service: ResMut<ChunkSystem>,
    mut rerender_chunks: EventWriter<RerenderChunkFlag>,
    mut chunk_cache: Local<ChunkSync>,
    mut send_response: EventWriter<SendPacket>,
) {
    for event in event_reader.iter() {
        match &event.0 {
            Protocol::FullChunkUpdate(update) => {
                let location = Vector3::new(update.x, update.y, update.z);

                chunk_service.create_chunk(
                    location,
                    update.data,
                    &mut commands,
                    &asset_service,
                    &mut rerender_chunks,
                    &mut meshes,
                );

                // Acknowledge
                send_response.send(SendPacket(
                    Protocol::AcknowledgeChunk(AcknowledgeChunk::new(update.x, update.y, update.z)),
                    UserId(0),
                ));
            }
            Protocol::PartialChunkUpdate(update) => {
                if !chunk_cache.partial_chunks.contains_key(&update.id) {
                    let partial_chunks = vec![update.clone()];
                    chunk_cache.partial_chunks.insert(update.id, partial_chunks);
                    continue;
                }

                chunk_cache
                    .partial_chunks
                    .get_mut(&update.id)
                    .unwrap()
                    .push(update.clone());
                // Check if its full
                let needed_chunks = ((CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE) as f32
                    / PARTIAL_CHUNK_UPDATE_SIZE as f32)
                    .ceil() as usize;

                if chunk_cache.partial_chunks.get(&update.id).unwrap().len() != needed_chunks {
                    continue;
                }

                let location = Vector3::new(update.x, update.y, update.z);

                if let Some(chunk) = FullChunkUpdate::from_partial(
                    chunk_cache.partial_chunks.remove(&update.id).unwrap(),
                ) {
                    chunk_service.create_chunk(
                        location,
                        chunk.data,
                        &mut commands,
                        &asset_service,
                        &mut rerender_chunks,
                        &mut meshes,
                    );
                } else {
                    warn!("Partial chunk failed to build {:?}", location);
                }
            }
            Protocol::BlockUpdate(update) => {
                let location = Vector3::new(update.x, update.y, update.z);

                // Locate chunk
                let (chunk_loc, inner_loc) = global_to_local_position(location);

                // Try find chunk
                if let Some(mut chunk) = chunk_service.chunks.get_mut(&chunk_loc) {
                    // Found chunk! Update block
                    chunk.world[inner_loc.x][inner_loc.y][inner_loc.z] = update.id;

                    // Rerender
                    rerender_chunks.send(RerenderChunkFlag {
                        chunk: chunk_loc,
                        context: RerenderChunkFlagContext::Surrounding,
                    });
                } else {
                    // Create chunk data
                    let mut chunk = [[[0; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];

                    // Set block
                    chunk[inner_loc.x][inner_loc.y][inner_loc.z] = update.id;

                    // Create chunk
                    chunk_service.create_chunk(
                        chunk_loc,
                        chunk,
                        &mut commands,
                        &mut asset_service,
                        &mut rerender_chunks,
                        &mut meshes,
                    );
                }
            }
            _ => {}
        }
    }
}
