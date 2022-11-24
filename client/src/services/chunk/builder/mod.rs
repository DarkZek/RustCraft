mod entry;

use crate::game::blocks::states::BlockStates;
use crate::helpers::from_bevy_vec3;
use crate::services::chunk::builder::entry::{MeshBuildEntry, PLAYER_POS};
use crate::services::chunk::data::generate_mesh::UpdateChunkMesh;
use crate::services::chunk::ChunkService;
use bevy::prelude::*;
use bevy::render::mesh::Indices;
use nalgebra::Vector3;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rc_networking::constants::CHUNK_SIZE;
use std::collections::{BinaryHeap, VecDeque};
use std::sync::atomic::{AtomicI32, Ordering};

pub struct RerenderChunkFlag {
    pub chunk: Vector3<i32>,
    /// Whether we should also re-render adjacent chunks
    pub adjacent: bool,
}

#[derive(Default)]
pub struct MeshBuilderCache {
    // A priority list of chunks to build
    chunks: BinaryHeap<MeshBuildEntry>,
}

pub fn mesh_builder(
    mut flags: EventReader<RerenderChunkFlag>,
    chunks: Res<ChunkService>,
    mut meshes: ResMut<Assets<Mesh>>,
    camera: Query<&Transform, With<Camera>>,
    block_states: Res<BlockStates>,
    mut builder_data: Local<MeshBuilderCache>,
    service: Res<ChunkService>,
) {
    // Update player location
    let pos = from_bevy_vec3(camera.single().translation);
    PLAYER_POS[0].store(pos.x as i32, Ordering::Relaxed);
    PLAYER_POS[1].store(pos.y as i32, Ordering::Relaxed);
    PLAYER_POS[2].store(pos.z as i32, Ordering::Relaxed);

    let mut rerender_chunks = Vec::new();

    // Add all new flags to rerender list
    for flag in flags.iter() {
        rerender_chunks.push(flag.chunk);

        // If rerendering adjacent chunks add them too
        if flag.adjacent {
            rerender_chunks.push(flag.chunk + Vector3::new(0, 0, 1));
            rerender_chunks.push(flag.chunk + Vector3::new(0, 0, -1));
            rerender_chunks.push(flag.chunk + Vector3::new(0, 1, 0));
            rerender_chunks.push(flag.chunk + Vector3::new(0, -1, 0));
            rerender_chunks.push(flag.chunk + Vector3::new(1, 0, 0));
            rerender_chunks.push(flag.chunk + Vector3::new(-1, 0, 0));
        }
    }

    // Loop over all new chunks to render and add them to the list if the chunk exists and if its not already being rerendered
    for pos in rerender_chunks {
        // The chunk data exists
        if let Some(data) = service.chunks.get(&pos) {
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

    let updates = iterator
        .map(|entry: &MeshBuildEntry| {
            // If the data exists
            if let Some(chunk) = chunks.chunks.get(&entry.chunk) {
                // Generate mesh & gpu buffers
                Some((
                    chunk.generate_mesh(&chunks, &block_states, true),
                    &chunk.mesh,
                ))
            } else {
                warn!("Chunk data doesn't exist when trying to build chunk");
                None
            }
        })
        .collect::<Vec<Option<(UpdateChunkMesh, &Handle<Mesh>)>>>();

    for update in updates {
        if let Some((val, handle)) = update {
            apply_mesh(val, meshes.get_mut(handle).unwrap());
        }
    }
}

fn apply_mesh(update: UpdateChunkMesh, mesh: &mut Mesh) {
    mesh.set_indices(Some(Indices::U32(update.indices)));
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, update.positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, update.normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, update.uv_coordinates);
}
