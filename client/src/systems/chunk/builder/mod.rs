mod entry;
mod generate_mesh;
mod lighting;
pub mod build_context;
pub mod thread;

use crate::systems::chunk::builder::entry::{MeshBuildEntry, PLAYER_POS};
use crate::systems::chunk::builder::generate_mesh::UpdateChunkMesh;
use crate::systems::chunk::nearby_cache::NearbyChunkCache;
use crate::systems::chunk::ChunkSystem;
use bevy::prelude::*;
use bevy::render::mesh::MeshVertexAttribute;
use bevy::render::render_resource::VertexFormat;
use nalgebra::Vector3;
use rc_shared::block::BlockStates;
use rc_shared::helpers::from_bevy_vec3;
use std::collections::BinaryHeap;
use std::mem;
use std::sync::atomic::Ordering;
use bevy::tasks::{AsyncComputeTaskPool, Task};
use bevy::tasks::futures_lite::future;
use bevy::tasks::block_on;
use rc_shared::chunk::ChunkDataStorage;
use crate::systems::camera::MainCamera;
use crate::systems::chunk::builder::build_context::ChunkBuildContext;
use crate::systems::chunk::builder::lighting::{LightingUpdateData};
use crate::systems::chunk::builder::thread::ChunkBuilderScheduler;
use crate::systems::chunk::builder::thread::executor::{ChunkBuilderExecutor, ChunkBuilderJob};
use crate::systems::chunk::flags::ChunkFlagsBitMap;
use crate::systems::chunk::builder::thread::ChunkBuilderSchedulerTrait;

pub const ATTRIBUTE_LIGHTING_COLOR: MeshVertexAttribute =
    MeshVertexAttribute::new("Lighting", 988540917, VertexFormat::Float32x4);

pub const ATTRIBUTE_WIND_STRENGTH: MeshVertexAttribute =
    MeshVertexAttribute::new("WindStrength", 988520913, VertexFormat::Float32);

#[derive(Event)]
pub struct RerenderChunkFlag {
    pub chunk: Vector3<i32>,
    /// Whether we should also re-render adjacent chunks
    pub context: RerenderChunkFlagContext,
}

// The context surrounding the rerender chunk flag to know if we should load other chunks around
#[derive(Eq, PartialEq)]
pub enum RerenderChunkFlagContext {
    None,
    Adjacent,
    Surrounding,
}

#[derive(Resource)]
pub struct MeshBuilderContext {
    // A priority list of chunks to build
    chunks: BinaryHeap<MeshBuildEntry>,

    processing_chunk_handles: Vec<Task<(UpdateChunkMesh, LightingUpdateData)>>,

    scheduler: ChunkBuilderScheduler
}

pub fn setup_mesh_builder_context(mut commands: Commands, block_states: Res<BlockStates>) {
    commands.insert_resource(MeshBuilderContext {
        chunks: Default::default(),
        processing_chunk_handles: vec![],
        scheduler: ChunkBuilderScheduler::new(ChunkBuilderExecutor::new(block_states.clone())),
    });
}

impl MeshBuilderContext {
    pub fn update_requested_chunks(
        &mut self,
        mut flags: EventReader<RerenderChunkFlag>,
        chunks: &mut ChunkSystem
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

        // Loop over all new chunks to render and add them to the list if the chunk exists and if its not already being rerendered
        for pos in rerender_chunks {
            // The chunk data exists
            if let Some(chunk) = chunks.chunks.get(&pos) {
                if chunk.flags.has_flag(ChunkFlagsBitMap::AtEdge) {
                    // At edge of loaded world. We can't know its visible direction yet, so don't build it
                    continue
                }

                if chunk.world == ChunkDataStorage::Empty {
                    // The chunk has no blocks, so there's nothing to draw
                    continue;
                }

                // And if this chunk isn't already scheduled for rebuild
                if !self.chunks.iter().any(|v| v.chunk == pos) {
                    // Put entry into rebuild table
                    self.chunks.push(MeshBuildEntry {
                        chunk: pos,
                    });
                }
            } else {
                // Requested for chunk that has been unloaded or was never loaded
            }
        }

    }
}

const MAX_PROCESSING_CHUNKS: usize = 4;

/// Schedules chunk meshes to be built
pub fn mesh_scheduler(
    flags: EventReader<RerenderChunkFlag>,
    mut chunks: ResMut<ChunkSystem>,
    camera: Query<&Transform, With<MainCamera>>,
    block_states: Res<BlockStates>,
    mut builder_data: ResMut<MeshBuilderContext>,
) {
    // Update player location
    let pos = from_bevy_vec3(camera.single().translation);
    PLAYER_POS[0].store(pos.x as i32, Ordering::SeqCst);
    PLAYER_POS[1].store(pos.y as i32, Ordering::SeqCst);
    PLAYER_POS[2].store(pos.z as i32, Ordering::SeqCst);

    if builder_data.chunks.len() >= MAX_PROCESSING_CHUNKS {
        debug!("Chunk Builder queue ({}) at maximum capacity for this frame. Just increased by {}", builder_data.chunks.len(), flags.len());
    }

    builder_data.update_requested_chunks(flags, &mut chunks);

    // Collect all available chunks
    while !builder_data.chunks.is_empty() {

        if builder_data.processing_chunk_handles.len() >= MAX_PROCESSING_CHUNKS {
            break
        }

        let chunk_entry = builder_data.chunks.pop().unwrap();

        if let Some(chunk) = chunks.chunks.get(&chunk_entry.chunk) {
            let mut chunk = chunk.clone();

            let cache = NearbyChunkCache::from_service(&chunks, chunk.position);

            let mut context = ChunkBuildContext::new(&block_states, &cache);

            builder_data.scheduler.schedule(ChunkBuilderJob {
                chunk,
                context
            })

        } else {
            warn!("Chunk data for {:?} doesn't exist when trying to build chunk", chunk_entry.chunk);
            continue
        }
    }
}

/// Updates the mesh data with the new built data
pub fn mesh_updater(
    mut chunks: ResMut<ChunkSystem>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut builder_data: ResMut<MeshBuilderContext>
) {
    while let Some(update) = builder_data.scheduler.poll() {
        if let Some(chunk) = chunks.chunks.get_mut(&update.position) {
            if chunk.handles.is_none() {
                continue
            }

            update.mesh.opaque
                .apply_mesh(meshes.get_mut(&chunk.handles.as_ref().unwrap().opaque_mesh).unwrap());
            update.mesh.translucent
                .apply_mesh(meshes.get_mut(&chunk.handles.as_ref().unwrap().translucent_mesh).unwrap());

            chunk.light_levels = update.lighting.data;
        }
    }
}