use bevy::ecs::prelude::Component;
use nalgebra::Vector3;
use noise::{NoiseFn, Perlin};
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

    pub fn generate(position: Vector3<i32>) -> ChunkData {
        let mut world = [[[0; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];

        let ground_perlin = Perlin::new(0);
        let grass_perlin = Perlin::new(1);

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let absolute = Vector3::new(
                        (position.x * 16) + x as i32,
                        (position.y * 16) + y as i32,
                        (position.z * 16) + z as i32,
                    );

                    let ground_level = 35
                        + ground_perlin
                            .get([absolute.x as f64 / 20.0, absolute.z as f64 / 20.0])
                            .mul(3.0)
                            .floor() as i32;

                    println!("{:?}", ground_level);

                    if absolute.y < ground_level - 3 {
                        world[x][y][z] = 6;
                    } else if absolute.y < ground_level {
                        world[x][y][z] = 1;
                    } else if absolute.y == ground_level {
                        world[x][y][z] = 2;
                    } else if absolute.y == ground_level + 1 {
                        if grass_perlin.get([absolute.x as f64 / 2.0, absolute.z as f64 / 2.0])
                            > 0.7
                        {
                            world[x][y][z] = 3;
                        }
                    }
                }
            }
        }

        ChunkData { position, world }
    }
}
