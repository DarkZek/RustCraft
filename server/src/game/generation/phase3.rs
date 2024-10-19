use crate::game::generation::noise::SimplexNoise;
use nalgebra::Vector3;
use rc_shared::biome::{EnvironmentMap};
use rc_shared::chunk::RawChunkData;
use rc_shared::CHUNK_SIZE;
use rc_shared::relative_chunk_flat_map::RelativeChunkFlatMap;

pub fn decorate_chunk(
    seed: u32,
    pos: Vector3<i32>,
    world: &mut RawChunkData,
    heightmap: &RelativeChunkFlatMap<i32>,
    environment: &EnvironmentMap,
) {
    let ground_noise = SimplexNoise::new(seed);
    let tropic_noise = SimplexNoise::new(seed + 1).with_scale(24.0);
    let grass_noise = SimplexNoise::new(seed + 2).with_scale(1.0);
    let ruby_noise = SimplexNoise::new(seed + 3).with_scale(5.0);

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let absolute = Vector3::new(
                    (pos.x * 16) + x as i32,
                    (pos.y * 16) + y as i32,
                    (pos.z * 16) + z as i32,
                );

                let ground_level = *heightmap.get([absolute.x, absolute.z]).unwrap();

                // Mountains without grass or dirt
                if ground_level < 18 {
                    // Dirt
                    if absolute.y < ground_level && absolute.y > ground_level - 4 {
                        world[x][y][z] = 1;
                    }
                    // Grass
                    if absolute.y == ground_level {
                        world[x][y][z] = 2;
                    }

                    let tropic_sand = tropic_noise.sample_2d(absolute.x, absolute.z);
                    if absolute.y == ground_level && tropic_sand > 0.8 {
                        world[x][y][z] = 8;
                    }

                    // Long grass
                    if absolute.y == ground_level + 1
                        && grass_noise.sample_2d(absolute.x, absolute.z) > 0.7
                        && !(absolute.y - 1 == ground_level && tropic_sand > 0.8) // No sand beneath
                    {
                        world[x][y][z] = 3;
                    }
                }

                if world[x][y][z] == 6 &&
                    ruby_noise.sample_3d(absolute.x, absolute.y, absolute.z) > 0.82 {
                    // world[x][y][z] = 76;
                }
            }
        }
    }
}
