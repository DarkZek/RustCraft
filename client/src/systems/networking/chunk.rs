use crate::systems::asset::AssetService;
use crate::systems::chunk::builder::{RerenderChunkRequest, RerenderChunkFlagContext};
use crate::systems::chunk::ChunkSystem;
use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;
use nalgebra::Vector3;
use rc_shared::constants::UserId;
use rc_networking::protocol::clientbound::chunk_update::{
    FullChunkUpdate, PartialChunkUpdate, PARTIAL_CHUNK_UPDATE_SIZE,
};
use rc_networking::protocol::serverbound::acknowledge_chunk::AcknowledgeChunk;
use rc_networking::protocol::Protocol;
use rc_networking::types::{ReceivePacket, SendPacket};
use rc_shared::chunk::ChunkDataStorage;
use rc_shared::helpers::global_to_local_position;
use rc_shared::CHUNK_SIZE;

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
    mut rerender_chunks: EventWriter<RerenderChunkRequest>,
    mut chunk_cache: Local<ChunkSync>,
    mut send_response: EventWriter<SendPacket>,
) {
    for event in event_reader.read() {
        match &event.0 {
            Protocol::FullChunkUpdate(update) => {
                let location = Vector3::new(update.x, update.y, update.z);

                if chunk_service.chunks.contains_key(&location) {
                    chunk_service.unload_chunk(location, &mut commands);
                }

                chunk_service.create_chunk(
                    location,
                    update.data.clone(),
                    &mut commands,
                    &asset_service,
                    &mut meshes,
                );

                rerender_chunks.send(RerenderChunkRequest {
                    chunk: location,
                    context: RerenderChunkFlagContext::Surrounding,
                });

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
                        &mut meshes,
                    );

                    rerender_chunks.send(RerenderChunkRequest {
                        chunk: location,
                        context: RerenderChunkFlagContext::Surrounding,
                    });
                } else {
                    warn!("Partial chunk failed to build {:?}", location);
                }
            }
            Protocol::BlockUpdate(update) => {
                let location = Vector3::new(update.x, update.y, update.z);

                // Locate chunk
                let (chunk_loc, inner_loc) = global_to_local_position(location);

                // Try find chunk
                if let Some(chunk) = chunk_service.chunks.get_mut(&chunk_loc) {
                    // Found chunk! Update block
                    chunk.world.set(inner_loc, update.id);

                    // Rerender
                    rerender_chunks.send(RerenderChunkRequest {
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
                        ChunkDataStorage::Data(Box::new(chunk)),
                        &mut commands,
                        &mut asset_service,
                        &mut meshes,
                    );

                    rerender_chunks.send(RerenderChunkRequest {
                        chunk: chunk_loc,
                        context: RerenderChunkFlagContext::Surrounding,
                    });
                }
            }
            Protocol::UnloadAllChunks(_) => {
                chunk_service.unload_all_chunks(&mut commands);
            }
            _ => {}
        }
    }
}
