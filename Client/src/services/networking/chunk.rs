use crate::helpers::global_to_local_position;
use crate::services::asset::AssetService;
use crate::services::chunk::ChunkService;
use crate::{
    default, info, shape, Assets, Color, Commands, EventReader, Mesh, MouseButton, PartialChunks,
    PbrBundle, Query, RerenderChunkFlag, Res, ResMut, StandardMaterial, Transform, Vec3,
};
use bevy::render::primitives::Aabb;
use bevy_testing_protocol::channels::Channels;
use bevy_testing_protocol::constants::CHUNK_SIZE;
use bevy_testing_protocol::protocol::Protocol;
use naia_bevy_client::events::MessageEvent;
use naia_bevy_client::Client;
use nalgebra::Vector3;

pub fn network_chunk_sync(
    mut event_reader: EventReader<MessageEvent<Protocol, Channels>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut partial_chunks: ResMut<PartialChunks>,
    mut asset_service: Res<AssetService>,
    mut chunk_service: ResMut<ChunkService>,
) {
    for event in event_reader.iter() {
        match event {
            MessageEvent(Channels::ChunkUpdates, Protocol::PartialChunkUpdate(update)) => {
                let location = Vector3::new(*update.x, *update.y, *update.z);

                partial_chunks.add(update.clone());

                if partial_chunks.is_complete(location) {
                    chunk_service.load_chunk(
                        location,
                        &mut partial_chunks,
                        &mut commands,
                        &asset_service,
                        &mut materials,
                        &mut meshes,
                    );
                }
            }
            MessageEvent(Channels::ChunkUpdates, Protocol::BlockUpdate(update)) => {
                let location = Vector3::new(*update.x, *update.y, *update.z);

                // Locate chunk
                let (chunk_loc, inner_loc) = global_to_local_position(location);

                // Try find chunk
                if let Some(mut chunk) = chunk_service.chunks.get_mut(&chunk_loc) {
                    // Found chunk! Update block
                    chunk.world[inner_loc.x][inner_loc.y][inner_loc.z] = *update.id;

                    // Rerender
                    commands
                        .entity(chunk.entity)
                        .insert(RerenderChunkFlag { chunk: chunk_loc });
                } else {
                    // Create chunk data
                    let mut chunk = [[[0; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];

                    // Set block
                    chunk[inner_loc.x][inner_loc.y][inner_loc.z] = *update.id;

                    // Create chunk
                    chunk_service.create_chunk(
                        chunk_loc,
                        chunk,
                        &mut commands,
                        &mut asset_service,
                        &mut meshes,
                    );
                }
            }
            _ => {}
        }
    }
}
