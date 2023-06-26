use crate::game::chunk::RawChunkData;
use crate::game::generation::noise::SimplexNoise;
use nalgebra::Vector3;
use rc_client::systems::chunk::biome::{ChunkEnvironment, Terrain};
use rc_networking::constants::CHUNK_SIZE;
use std::ops::Mul;

pub fn decorate_chunk(
    seed: u32,
    pos: Vector3<i32>,
    world: &mut RawChunkData,
    heightmap: &[[i32; CHUNK_SIZE]; CHUNK_SIZE],
    environment: &ChunkEnvironment,
) {
    let ground_noise = SimplexNoise::new(seed);
    let tropic_noise = SimplexNoise::new(seed + 1).with_scale(24.0);
    let grass_noise = SimplexNoise::new(seed + 2).with_scale(1.0);

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let absolute = Vector3::new(
                    (pos.x * 16) + x as i32,
                    (pos.y * 16) + y as i32,
                    (pos.z * 16) + z as i32,
                );

                let ground_level = heightmap[x][z];

                // Dirt
                if absolute.y < ground_level && absolute.y > ground_level - 4 {
                    world[x][y][z] = 1;
                }
                // Grass
                if absolute.y == ground_level {
                    world[x][y][z] = 2;
                }

                // Long grass
                if absolute.y == ground_level + 1
                    && grass_noise.sample_2d(absolute.x, absolute.z) > 0.7
                {
                    world[x][y][z] = 3;
                }

                let tropic_sand = tropic_noise.sample_2d(absolute.x, absolute.z);
                if absolute.y == ground_level && tropic_sand > 0.8 {
                    world[x][y][z] = 8;
                }
            }
        }
    }
}
