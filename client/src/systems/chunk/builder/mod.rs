mod entry;
mod generate_mesh;
mod lighting;

use crate::systems::chunk::builder::entry::{MeshBuildEntry, PLAYER_POS};
use crate::systems::chunk::builder::generate_mesh::UpdateChunkMesh;
use crate::systems::chunk::builder::lighting::LightingUpdateData;
use crate::systems::chunk::data::ChunkData;
use crate::systems::chunk::nearby_cache::NearbyChunkCache;
use crate::systems::chunk::ChunkSystem;
use bevy::prelude::*;
use bevy::render::mesh::MeshVertexAttribute;
use bevy::render::render_resource::VertexFormat;
use nalgebra::Vector3;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rc_shared::block::BlockStates;
use rc_shared::helpers::from_bevy_vec3;
use std::collections::BinaryHeap;
use std::sync::atomic::Ordering;

pub const ATTRIBUTE_LIGHTING_COLOR: MeshVertexAttribute =
    MeshVertexAttribute::new("Lighting", 988540917, VertexFormat::Float32x4);

#[derive(Event)]
pub struct RerenderChunkFlag {
    pub chunk: Vector3<i32>,
    /// Whether we should also re-render adjacent chunks
    pub context: RerenderChunkFlagContext,
}

// The context surrounding the renrender chunk flag to know if we should load other chunks around
#[derive(Eq, PartialEq)]
pub enum RerenderChunkFlagContext {
    None,
    Adjacent,
    Surrounding,
}

#[derive(Default)]
pub struct MeshBuilderCache {
    // A priority list of chunks to build
    chunks: BinaryHeap<MeshBuildEntry>,
}

pub fn mesh_builder(
    mut flags: EventReader<RerenderChunkFlag>,
    mut chunks: ResMut<ChunkSystem>,
    mut meshes: ResMut<Assets<Mesh>>,
    camera: Query<&Transform, With<Camera>>,
    block_states: Res<BlockStates>,
    mut builder_data: Local<MeshBuilderCache>,
) {
    // Update player location
    let pos = from_bevy_vec3(camera.single().translation);
    PLAYER_POS[0].store(pos.x as i32, Ordering::SeqCst);
    PLAYER_POS[1].store(pos.y as i32, Ordering::SeqCst);
    PLAYER_POS[2].store(pos.z as i32, Ordering::SeqCst);

    let mut rerender_chunks = Vec::new();

    // Add all new flags to rerender list
    for flag in flags.read() {
        rerender_chunks.push(flag.chunk);

        // If rerendering adjacent chunks add them too
        if flag.context == RerenderChunkFlagContext::Adjacent {
            rerender_chunks.push(flag.chunk + Vector3::new(0, 0, 1));
            rerender_chunks.push(flag.chunk + Vector3::new(0, 0, -1));
            rerender_chunks.push(flag.chunk + Vector3::new(0, 1, 0));
            rerender_chunks.push(flag.chunk + Vector3::new(0, -1, 0));
            rerender_chunks.push(flag.chunk + Vector3::new(1, 0, 0));
            rerender_chunks.push(flag.chunk + Vector3::new(-1, 0, 0));
        }

        if flag.context == RerenderChunkFlagContext::Surrounding {
            for x in -1..=1 {
                for y in -1..=1 {
                    for z in -1..=1 {
                        if x == 0 && y == 0 && z == 0 {
                            continue;
                        }
                        rerender_chunks.push(flag.chunk + Vector3::new(x, y, z));
                    }
                }
            }
        }
    }

    // Loop over all new chunks to render and add them to the list if the chunk exists and if its not already being rerendered
    for pos in rerender_chunks {
        // The chunk data exists
        if let Some(data) = chunks.chunks.get(&pos) {
            // And if this chunk isn't already scheduled for rebuild
            if !builder_data.chunks.iter().any(|v| v.chunk == pos) {
                // Put entry into rebuild table
                builder_data.chunks.push(MeshBuildEntry {
                    entity: data.entity.clone(),
                    chunk: pos,
                });
            }
        }
    }

    // How many chunks to render per frame
    let max_chunks_per_frame = 40;

    let build_chunks_count = builder_data.chunks.len().min(max_chunks_per_frame);

    let mut build_chunks = Vec::with_capacity(build_chunks_count);

    // Collect
    while build_chunks.len() < build_chunks_count {
        build_chunks.push(builder_data.chunks.pop().unwrap());
    }

    #[cfg(not(target_arch = "wasm32"))]
    let iterator = build_chunks.par_iter();
    #[cfg(target_arch = "wasm32")]
    let iterator = build_chunks.iter();

    // Direct lighting pass
    let updates = iterator
        .map(|entry| {
            if let Some(chunk) = chunks.chunks.get(&entry.chunk) {
                let nearby = NearbyChunkCache::from_service(&chunks, chunk.position);
                Some((chunk.position, chunk.build_lighting(&block_states, &nearby)))
            } else {
                None
            }
        })
        .collect::<Vec<Option<(Vector3<i32>, LightingUpdateData)>>>();

    // Update chunks
    for update in updates {
        if let Some((chunk_pos, update)) = update {
            chunks.chunks.get_mut(&chunk_pos).unwrap().light_levels = update.data;
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    let iterator = build_chunks.par_iter();
    #[cfg(target_arch = "wasm32")]
    let iterator = build_chunks.iter();

    let updates = iterator
        .map(|entry: &MeshBuildEntry| {
            // If the data exists
            if let Some(chunk) = chunks.chunks.get(&entry.chunk) {
                let cache = NearbyChunkCache::from_service(&chunks, chunk.position);
                // Generate mesh & gpu buffers
                Some((
                    chunk.build_mesh(&chunks, &block_states, true, &cache),
                    chunk,
                ))
            } else {
                warn!("Chunk data doesn't exist when trying to build chunk");
                None
            }
        })
        .collect::<Vec<Option<(UpdateChunkMesh, &ChunkData)>>>();

    for update in updates {
        if let Some((val, chunk)) = update {
            val.opaque
                .apply_mesh(meshes.get_mut(&chunk.opaque_mesh).unwrap());
            val.translucent
                .apply_mesh(meshes.get_mut(&chunk.translucent_mesh).unwrap());
        }
    }
}
