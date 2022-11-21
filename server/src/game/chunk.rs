use bevy::ecs::prelude::Component;
use nalgebra::Vector3;
use rc_client::rc_protocol::constants::CHUNK_SIZE;
use serde::{Deserialize, Serialize};

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

    pub fn generate(position: Vector3<i32>) -> ChunkData {
        let mut world = [[[0; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    if (x + y + z) % 3 == 0 {
                        world[x][y][z] = 1;
                    } else {
                        world[x][y][z] = 2;
                    }
                }
            }
        }

        if position.x == 0 && position.y == 0 && position.z == 0 {
            world[0][15][15] = 3;
        }

        ChunkData { position, world }
    }
}
