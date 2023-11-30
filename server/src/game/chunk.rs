use bevy::ecs::prelude::Component;
use nalgebra::Vector3;
use rc_shared::chunk::RawChunkData;
use rc_shared::CHUNK_SIZE;
use serde::{Deserialize, Serialize};

#[derive(Debug, Component, Serialize, Deserialize, PartialEq, Clone)]
pub struct ChunkData {
    pub position: Vector3<i32>,
    pub world: RawChunkData,
}

impl ChunkData {
    pub fn new(position: Vector3<i32>, world: RawChunkData) -> ChunkData {
        ChunkData { position, world }
    }

    pub fn blank(position: Vector3<i32>) -> ChunkData {
        ChunkData {
            position,
            world: [[[0; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
        }
    }
}
