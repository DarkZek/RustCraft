use crate::game::generation::noise::SimplexNoise;
use nalgebra::{Vector2, Vector3};
use rc_shared::biome::{EnvironmentEntry, EnvironmentMap};
use rc_shared::CHUNK_SIZE;

pub struct EnvironmentMapConfig {
    pub terrain_scale: f32,
    pub vegetation_scale: f32,
    pub climate_scale: f32
}

impl Default for EnvironmentMapConfig {
    fn default() -> Self {
        Self {
            terrain_scale: 8.0*16.0,
            vegetation_scale: 16.0,
            climate_scale: 180.0 * 16.0
        }
    }
}

pub fn generate_environment_map(seed: u32, pos: Vector3<i32>, config: &EnvironmentMapConfig) -> EnvironmentMap {

    let world_pos = Vector2::new(pos.x * CHUNK_SIZE as i32, pos.z * CHUNK_SIZE as i32);

    let mut map = EnvironmentMap::new_empty(
        world_pos,
        CHUNK_SIZE
    );

    for x in (world_pos.x - CHUNK_SIZE as i32)..(world_pos.x + (CHUNK_SIZE as i32 * 2)) {
        for z in (world_pos.y - CHUNK_SIZE as i32)..(world_pos.y + (CHUNK_SIZE as i32 * 2)) {
            map.set([x, z], generate_biome(seed, Vector3::new(x, 0, z), config));
        }
    }

    return map;
}

pub fn generate_biome(seed: u32, pos: Vector3<i32>, config: &EnvironmentMapConfig) -> EnvironmentEntry {
    let climate_noise = SimplexNoise::new(seed).with_scale(config.climate_scale);

    let climate = climate_noise.sample_2d(pos.x, pos.z);

    let terrain = SimplexNoise::new(seed + 1)
        .with_scale(config.terrain_scale)
        .sample_2d(pos.x, pos.z);

    let vegetation = SimplexNoise::new(seed + 2)
        .with_scale(config.vegetation_scale)
        .sample_2d(pos.x, pos.z);

    return EnvironmentEntry {
        climate,
        terrain,
        vegetation,
    };
}
