use nalgebra::Vector3;
use noise::{NoiseFn, Perlin};
use rc_client::systems::chunk::biome::{ChunkEnvironment, Climate, Terrain, Vegetation};

pub fn generate_biome(seed: u32, pos: Vector3<i32>) -> ChunkEnvironment {
    let climate_perlin = Perlin::new(seed);
    let mut climate = Climate::Temperate;

    if climate_perlin.get([pos.x as f64 / 360.0, pos.z as f64 / 360.0]) > 0.7 {
        climate = Climate::Tropic;
    }

    let terrain_perlin = Perlin::new(seed + 1);
    let mut terrain = Terrain::Plain;

    if terrain_perlin.get([pos.x as f64 / 360.0, pos.z as f64 / 360.0]) > 0.7 {
        terrain = Terrain::Hills;
    }

    let terrain_perlin = Perlin::new(seed + 2);
    let mut vegetation = Vegetation::Grass;

    if terrain_perlin.get([pos.x as f64 / 360.0, pos.z as f64 / 360.0]) > 0.5 {
        vegetation = Vegetation::Trees;
    }

    return ChunkEnvironment {
        climate,
        terrain,
        vegetation,
    };
}
