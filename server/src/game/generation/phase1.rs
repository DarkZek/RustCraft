use crate::game::generation::noise::SimplexNoise;
use nalgebra::{Vector2, Vector3};
use rc_shared::biome::{ChunkEnvironment, EnvironmentEntry, EnvironmentMap};
use rc_shared::CHUNK_SIZE;

pub fn generate_environment_map(seed: u32, pos: Vector3<i32>) -> EnvironmentMap {

    let world_pos = Vector2::new(pos.x * CHUNK_SIZE as i32, pos.z * CHUNK_SIZE as i32);

    let mut map = EnvironmentMap::new_empty(
        world_pos
    , CHUNK_SIZE);

    for x in (world_pos.x - CHUNK_SIZE as i32)..(world_pos.x + (CHUNK_SIZE as i32 * 2)) {
        for z in (world_pos.y - CHUNK_SIZE as i32)..(world_pos.y + (CHUNK_SIZE as i32 * 2)) {
            map.set([x, z], generate_biome(seed, Vector3::new(x, 0, z)));
        }
    }

    return map;
}

pub fn generate_biome(seed: u32, pos: Vector3<i32>) -> EnvironmentEntry {
    let climate_noise = SimplexNoise::new(seed).with_scale(180.0 * 16.0);

    let climate = climate_noise.sample_2d(pos.x, pos.z);

    let terrain = SimplexNoise::new(seed + 1)
        .with_scale(180.0 * 16.0)
        .sample_2d(pos.x, pos.z);

    let vegetation = SimplexNoise::new(seed + 2)
        .with_scale(16.0)
        .sample_2d(pos.x, pos.z);

    return EnvironmentEntry {
        climate,
        terrain,
        vegetation,
    };
}
