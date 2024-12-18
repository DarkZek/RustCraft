use nalgebra::Vector3;
use serde::{Deserialize, Serialize};
use rc_shared::block::BlockStates;
use crate::systems::chunk::builder::build_context::ChunkBuildContext;
use crate::systems::chunk::builder::generate_mesh::UpdateChunkMesh;
use crate::systems::chunk::builder::lighting::LightingUpdateData;
use crate::systems::chunk::data::ChunkData;

pub struct ChunkBuilderExecutor {
    pub requests: Vec<ChunkBuilderJob>,
    pub block_states: BlockStates
}

impl ChunkBuilderExecutor {
    pub fn new(block_states: BlockStates) -> ChunkBuilderExecutor {
        ChunkBuilderExecutor {
            requests: vec![],
            block_states,
        }
    }

    pub fn build(&mut self) -> Option<ChunkBuilderUpdate> {

        let Some(job) = self.requests.pop() else {
            return None
        };

        let ChunkBuilderJob {
            mut chunk,
            mut context
        } = job;

        let mut lighting = chunk.build_lighting(&mut context);

        chunk.build_skylighting(
            &mut context,
            &mut lighting.data
        );

        chunk.light_levels = lighting.data;

        // Generate mesh & gpu buffers
        let mesh = chunk.build_mesh(&self.block_states, false, &context);

        Some(ChunkBuilderUpdate {
            position: chunk.position,
            mesh,
            lighting
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct ChunkBuilderJob {
    pub chunk: ChunkData,
    pub context: ChunkBuildContext
}

#[derive(Serialize, Deserialize)]
pub struct ChunkBuilderUpdate {
    pub position: Vector3<i32>,
    pub mesh: UpdateChunkMesh,
    pub lighting: LightingUpdateData
}