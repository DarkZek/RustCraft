use std::collections::BinaryHeap;
use bevy::prelude::{Commands, EventReader, Res, Resource};
use bevy::tasks::Task;
use nalgebra::Vector3;
use rc_shared::block::BlockStates;
use rc_shared::chunk::ChunkDataStorage;
use crate::systems::chunk::builder::entry::MeshBuildEntry;
use crate::systems::chunk::builder::generate_mesh::UpdateChunkMesh;
use crate::systems::chunk::builder::lighting::LightingUpdateData;
use crate::systems::chunk::builder::{RerenderChunkRequest, RerenderChunkFlagContext};
use crate::systems::chunk::builder::thread::{ChunkBuilderScheduler, ChunkBuilderSchedulerTrait};
use crate::systems::chunk::builder::thread::executor::ChunkBuilderExecutor;
use crate::systems::chunk::ChunkSystem;
use crate::systems::chunk::flags::ChunkFlagsBitMap;

#[derive(Resource)]
pub struct MeshBuilderContext {
    // A priority list of chunks to build
    pub chunks: BinaryHeap<MeshBuildEntry>,

    pub processing_chunk_handles: Vec<Task<(UpdateChunkMesh, LightingUpdateData)>>,

    pub scheduler: ChunkBuilderScheduler
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
        mut flags: EventReader<RerenderChunkRequest>,
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
            if let Some(chunk) = chunks.chunks.get_mut(&pos) {
                if chunk.flags.has_flag(ChunkFlagsBitMap::AtEdge) {
                    // At edge of loaded world. We can't know its visible direction yet, so don't build it
                    continue
                }

                if chunk.world == ChunkDataStorage::Empty {
                    // The chunk has no blocks, so there's nothing to draw
                    chunk.flags.add_flag(ChunkFlagsBitMap::Ready);
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