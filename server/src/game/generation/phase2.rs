use crate::game::chunk::RawChunkData;
use nalgebra::{Vector2, Vector3};
use noise::{NoiseFn, Perlin};
use rc_client::systems::chunk::biome::{ChunkEnvironment, Terrain};
use rc_networking::constants::CHUNK_SIZE;
use std::ops::Mul;

pub fn generate_greybox_chunk(
    seed: u32,
    pos: Vector3<i32>,
    environment: &ChunkEnvironment,
) -> (RawChunkData, [[i32; CHUNK_SIZE]; CHUNK_SIZE]) {
    let ground_perlin = Perlin::new(0);

    let mut heightmap = [[0; CHUNK_SIZE]; CHUNK_SIZE];

    for x in 0..CHUNK_SIZE {
        for z in 0..CHUNK_SIZE {
            let absolute = Vector2::new((pos.x * 16) + x as i32, (pos.z * 16) + z as i32);

            let mut base_height = 35;

            if environment.terrain == Terrain::Hills {
                base_height = 45;
            }

            let ground_level = base_height
                + ground_perlin
                    .get([absolute.x as f64 / 20.0, absolute[1] as f64 / 20.0])
                    .mul(3.0)
                    .floor() as i32;

            heightmap[x][z] = ground_level;
        }
    }

    let mut world = [[[0; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let absolute = Vector3::new(
                    (pos.x * 16) + x as i32,
                    (pos.y * 16) + y as i32,
                    (pos.z * 16) + z as i32,
                );
                if absolute.y < heightmap[x][z] {
                    world[x][y][z] = 6;
                }
            }
        }
    }

    (world, heightmap)
}
