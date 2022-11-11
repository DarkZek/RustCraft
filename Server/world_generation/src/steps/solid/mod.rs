mod heightmap;

use noise::Perlin;
use crate::blocks::{AIR_BLOCK, STONE_BLOCK};
use crate::constants::{CHUNK_SIZE, VERTICAL_CHUNKS};
use crate::generator::GeneratorSettings;
use crate::steps::maps::{GenerationMaps, WorldMaps};
use crate::WorldGenerator;
use noise::Seedable;

pub(crate) struct SolidGeneration {
    pub(crate) noise1: Perlin,
    pub(crate) noise2: Perlin,
    pub(crate) noise3: Perlin,
}

impl SolidGeneration {

    /// Create's a new solid block generator
    pub fn new(seed: u32) -> SolidGeneration {
        let noise1 = Perlin::new();
        noise1.set_seed(seed);

        let noise2 = Perlin::new();
        noise2.set_seed(seed + 1);

        let noise3 = Perlin::new();
        noise3.set_seed(seed + 2);

        SolidGeneration {
            noise1,
            noise2,
            noise3
        }
    }

    /// Generates a solid foundation for the world
    pub fn generate(&self, settings: &GeneratorSettings, maps: &WorldMaps, location: &[i32; 2]) -> [[[u32; CHUNK_SIZE]; CHUNK_SIZE*VERTICAL_CHUNKS]; CHUNK_SIZE] {

        let heightmap = self.generate_heightmap(settings, maps, location);

        let mut data = [[[0; CHUNK_SIZE]; CHUNK_SIZE*VERTICAL_CHUNKS]; CHUNK_SIZE];

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    data[x][y][z] = self.get_block(x, y, z, &heightmap);
                }
            }
        }

        data
    }

    pub fn get_block(&self, x: usize, y: usize, z: usize, heightmap: &[[u32; CHUNK_SIZE]; CHUNK_SIZE]) -> u32 {
        if y < heightmap[x][z] as usize {
            return STONE_BLOCK;
        } else {
            return AIR_BLOCK;
        }
    }
}