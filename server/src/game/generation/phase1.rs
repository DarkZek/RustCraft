use crate::game::generation::noise::SimplexNoise;
use nalgebra::Vector3;
use rc_client::systems::chunk::biome::{ChunkEnvironment, Climate, Terrain, Vegetation};

pub fn generate_biome(seed: u32, pos: Vector3<i32>) -> ChunkEnvironment {
    let climate_noise = SimplexNoise::new(seed).with_scale(180.0);
    let mut climate = Climate::Temperate;

    if climate_noise.sample_2d(pos.x, pos.z) > 0.7 {
        climate = Climate::Tropic;
    }

    let terrain_noise = SimplexNoise::new(seed + 1).with_scale(180.0);
    let mut terrain = Terrain::Plain;

    if terrain_noise.sample_2d(pos.x, pos.z) > 0.7 {
        terrain = Terrain::Hills;
    }

    let vegetation_noise = SimplexNoise::new(seed + 2);
    let mut vegetation = Vegetation::Grass;

    if vegetation_noise.sample_2d(pos.x, pos.z) > 0.5 {
        vegetation = Vegetation::Trees;
    }

    return ChunkEnvironment {
        climate,
        terrain,
        vegetation,
    };
}
