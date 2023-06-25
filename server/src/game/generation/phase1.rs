use nalgebra::Vector3;
use noise::{NoiseFn, Simplex};
use rc_client::systems::chunk::biome::{ChunkEnvironment, Climate, Terrain, Vegetation};

pub fn generate_biome(seed: u32, pos: Vector3<i32>) -> ChunkEnvironment {
    let climate_noise = Simplex::new(seed);
    let mut climate = Climate::Temperate;

    if climate_noise.get([pos.x as f64 / 180.0, pos.z as f64 / 180.0]) > 0.7 {
        climate = Climate::Tropic;
    }

    let terrain_noise = Simplex::new(seed + 1);
    let mut terrain = Terrain::Plain;

    if terrain_noise.get([pos.x as f64 / 180.0, pos.z as f64 / 180.0]) > 0.7 {
        terrain = Terrain::Hills;
    }

    let vegetation_noise = Simplex::new(seed + 2);
    let mut vegetation = Vegetation::Grass;

    if vegetation_noise.get([pos.x as f64 / 180.0, pos.z as f64 / 180.0]) > 0.5 {
        vegetation = Vegetation::Trees;
    }

    return ChunkEnvironment {
        climate,
        terrain,
        vegetation,
    };
}
