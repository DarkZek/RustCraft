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
use std::mem;
use std::sync::atomic::Ordering;
use bevy::tasks::{AsyncComputeTaskPool, Task};
use bevy::tasks::futures_lite::future;
use bevy::tasks::block_on;

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

    processing_chunk_handles: Vec<Task<(UpdateChunkMesh)>>
}

impl MeshBuilderCache {
    pub fn update_requested_chunks(
        &mut self,
        mut flags: EventReader<RerenderChunkFlag>,
        mut chunks: &mut ChunkSystem
    ) {
        let mut rerender_chunks = Vec::new();

        // Add all new flags to rerender list
        for flag in flags.read() {
            rerender_chunks.push(flag.chunk);

            match flag.context {
                RerenderChunkFlagContext::None => {}
                RerenderChunkFlagContext::Adjacent => {
                    // If rerendering adjacent chunks add them too
                    rerender_chunks.push(flag.chunk + Vector3::new(0, 0, 1));
                    rerender_chunks.push(flag.chunk + Vector3::new(0, 0, -1));
                    rerender_chunks.push(flag.chunk + Vector3::new(0, 1, 0));
                    rerender_chunks.push(flag.chunk + Vector3::new(0, -1, 0));
                    rerender_chunks.push(flag.chunk + Vector3::new(1, 0, 0));
                    rerender_chunks.push(flag.chunk + Vector3::new(-1, 0, 0));
                }
                RerenderChunkFlagContext::Surrounding => {
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
        }

        if self.chunks.len() >= MAX_PROCESSING_CHUNKS {
            info!("Chunk Builder queue ({}) at maximum capacity for this frame. Just increased by {}", self.chunks.len(), rerender_chunks.len());
        }

        // Loop over all new chunks to render and add them to the list if the chunk exists and if its not already being rerendered
        for pos in rerender_chunks {
            // The chunk data exists
            if let Some(data) = chunks.chunks.get(&pos) {
                // And if this chunk isn't already scheduled for rebuild
                if !self.chunks.iter().any(|v| v.chunk == pos) {
                    // Put entry into rebuild table
                    self.chunks.push(MeshBuildEntry {
                        entity: data.entity.clone(),
                        chunk: pos,
                    });
                }
            } else {
                // Requested for chunk that has been unloaded or was never loaded
            }
        }

    }
}

const MAX_PROCESSING_CHUNKS: usize = 10;

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

    builder_data.update_requested_chunks(flags, &mut chunks);

    let mut existing_tasks = Vec::new();
    mem::swap(&mut existing_tasks, &mut builder_data.processing_chunk_handles);

    // Filter out any still processing handles
    for mut task in existing_tasks {
        if let Some(mut update) = block_on(future::poll_once(&mut task)) {
            // TODO: Ensure no blocks have changed since cloned
            if let Some(chunk) = chunks.chunks.get_mut(&update.chunk) {
                update.opaque
                    .apply_mesh(meshes.get_mut(&chunk.opaque_mesh).unwrap());
                update.translucent
                    .apply_mesh(meshes.get_mut(&chunk.translucent_mesh).unwrap());
            }
        } else {
            builder_data.processing_chunk_handles.push(task);
        }
    }

    let thread_pool = AsyncComputeTaskPool::get();

    // Collect all available chunks
    while !builder_data.chunks.is_empty() {

        if builder_data.processing_chunk_handles.len() >= MAX_PROCESSING_CHUNKS {
            break
        }

        let chunk_entry = builder_data.chunks.pop().unwrap();

        if let Some(chunk) = chunks.chunks.get(&chunk_entry.chunk) {
            let chunk = chunk.clone();

            // TODO: Make blockstates static
            let block_states = block_states.clone();

            let task = thread_pool.spawn(async move {
                //let cache = NearbyChunkCache::from_service(&chunks, chunk.position);
                // TODO: Transfer data
                let cache = NearbyChunkCache::empty(chunk.position);

                // Generate mesh & gpu buffers
                (
                    chunk.build_mesh(&block_states, true, &cache)
                )
            });

            builder_data.processing_chunk_handles.push(task);

        } else {
            warn!("Chunk data for {:?} doesn't exist when trying to build chunk", chunk_entry.chunk);
            continue
        }
    }
}
