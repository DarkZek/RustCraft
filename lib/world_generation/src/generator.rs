use noise::Perlin;
use crate::constants::{CHUNK_SIZE, VERTICAL_CHUNKS};
use noise::Seedable;
use crate::steps::maps::GenerationMaps;
use crate::steps::solid::SolidGeneration;

pub struct WorldGenerator {
    pub(crate) seed: u32,

    pub(crate) settings: GeneratorSettings,

    pub(crate) maps: GenerationMaps,

    pub(crate) solids: SolidGeneration,
}

impl WorldGenerator {

    pub fn new(seed: u32) -> WorldGenerator {
        WorldGenerator {
            seed,
            settings: GeneratorSettings::default(),
            maps: GenerationMaps::new(seed),
            solids: SolidGeneration::new(seed)
        }
    }

    pub fn generate_chunk(&self, position: [i32; 2]) -> [[[u32; CHUNK_SIZE]; CHUNK_SIZE*VERTICAL_CHUNKS]; CHUNK_SIZE] {
        // Go through the steps of generation

        // Step 1: Generate maps for erosion & biome
        let maps = self.maps.generate(&self.settings, &position);

        // Step 2: Generate the solid stone blocks of the world using a heightmap
        let data = self.solids.generate(&self.settings, &maps, &position);

        // Step 3: Add topsoil & details

        // Step 4: Add structures

        data
    }
}

pub struct GeneratorSettings {
    pub erosion_size: f64
}

impl Default for GeneratorSettings {
    fn default() -> Self {
        GeneratorSettings {
            erosion_size: 1.0
        }
    }
}