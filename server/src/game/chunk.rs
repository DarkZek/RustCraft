use bevy::ecs::prelude::Component;
use nalgebra::Vector3;
use rc_shared::chunk::{ChunkBlockMetadata, ChunkDataStorage, ChunkMetadata, RawChunkData};
use rc_shared::CHUNK_SIZE;
use serde::{Deserialize, Serialize};

#[derive(Debug, Component, Serialize, Deserialize, PartialEq, Clone)]
pub struct ChunkData {
    pub position: Vector3<i32>,
    pub world: ChunkDataStorage,
    pub block_metadata: ChunkBlockMetadata,
    pub metadata: ChunkMetadata,
}

impl ChunkData {
    pub fn new(
        position: Vector3<i32>,
        world: RawChunkData,
        metadata: ChunkMetadata,
        block_metadata: ChunkBlockMetadata,
    ) -> ChunkData {
        ChunkData {
            position,
            world: ChunkDataStorage::Data(Box::new(world)),
            block_metadata,
            metadata,
        }
    }

    pub fn blank(position: Vector3<i32>) -> ChunkData {
        ChunkData {
            position,
            world: ChunkDataStorage::Empty,
            block_metadata: Default::default(),
            metadata: Default::default(),
        }
    }

    pub fn optimise_data(&mut self) {
        self.world.optimise();
    }
}