mod entry;
mod generate_mesh;
mod lighting;
pub mod build_context;
pub mod thread;
pub mod builder;

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
use std::sync::atomic::Ordering;
use bevy::tasks::Task;
use rc_shared::chunk::ChunkDataStorage;
use crate::systems::camera::MainCamera;
use crate::systems::chunk::builder::build_context::ChunkBuildContext;
use crate::systems::chunk::builder::builder::MeshBuilderContext;
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
pub struct RerenderChunkRequest {
    pub chunk: Vector3<i32>,
    /// Whether we should also re-render adjacent chunks
    pub context: RerenderChunkFlagContext,
}

#[derive(Event)]
pub struct ChunkRebuiltEvent {
    pub chunk: Vector3<i32>
}

// The context surrounding the rerender chunk flag to know if we should load other chunks around
#[derive(Eq, PartialEq)]
pub enum RerenderChunkFlagContext {
    None,
    Adjacent,
    Surrounding,
}

const MAX_PROCESSING_CHUNKS: usize = 4;

/// Schedules chunk meshes to be built
pub fn mesh_scheduler(
    flags: EventReader<RerenderChunkRequest>,
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
            let chunk = chunk.clone();

            let cache = NearbyChunkCache::from_service(&chunks, chunk.position);

            let context = ChunkBuildContext::new(&block_states, &cache);

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
    mut builder_data: ResMut<MeshBuilderContext>,
    mut writer: EventWriter<ChunkRebuiltEvent>
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
            chunk.flags.add_flag(ChunkFlagsBitMap::Ready);
        }

        writer.send(ChunkRebuiltEvent {
            chunk: update.position
        });
    }
}