use bevy::ecs::prelude::Component;
use nalgebra::Vector3;
use rc_networking::constants::CHUNK_SIZE;
use serde::{Deserialize, Serialize};
use std::ops::Mul;

#[derive(Debug, Component, Serialize, Deserialize)]
pub struct ChunkData {
    pub position: Vector3<i32>,

    pub world: RawChunkData,
}

pub type RawChunkData = [[[u32; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];

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
