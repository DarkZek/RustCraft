use crate::game::generation::noise::SimplexNoise;
use nalgebra::Vector3;
use rc_client::systems::chunk::biome::{
    ChunkEnvironment, EnvironmentEntry,
};

pub fn generate_environment_map(seed: u32, pos: Vector3<i32>) -> ChunkEnvironment {
    let mut map = [[[EnvironmentEntry {
        climate: 0.0,
        terrain: 0.0,
        vegetation: 0.0,
    }; 16]; 16]; 16];

    for x in 0..16 {
        for y in 0..16 {
            for z in 0..16 {
                map[x][y][z] = generate_biome(seed, pos + Vector3::new(x, y, z).cast::<i32>());
            }
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
