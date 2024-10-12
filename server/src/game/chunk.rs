use bevy::ecs::prelude::Component;
use rc_shared::chunk::{ChunkBlockMetadata, ChunkDataStorage, ChunkMetadata, ChunkPosition};
use serde::{Deserialize, Serialize};

#[derive(Debug, Component, Serialize, Deserialize, PartialEq, Clone)]
pub struct ChunkData {
    pub position: ChunkPosition,
    pub world: ChunkDataStorage,
    pub block_metadata: ChunkBlockMetadata,
    pub metadata: ChunkMetadata,
}

impl ChunkData {
    pub fn new(
        position: ChunkPosition,
        world: ChunkDataStorage,
        metadata: ChunkMetadata,
        block_metadata: ChunkBlockMetadata,
    ) -> ChunkData {
        ChunkData {
            position,
            world,
            block_metadata,
            metadata,
        }
    }

    pub fn blank(position: ChunkPosition) -> ChunkData {
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