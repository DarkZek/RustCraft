use crate::WorldGenerator;
use crate::constants::CHUNK_SIZE;
use crate::visualiser::MapVisualiser;
use noise::{NoiseFn, Perlin};
use crate::generator::GeneratorSettings;
use crate::helper::calculate_global_pos;
use noise::Seedable;

pub(crate) struct GenerationMaps {
    pub(crate) noise: Perlin,
}

impl GenerationMaps {

    /// Creates a new generation maps
    pub fn new(seed: u32) -> GenerationMaps {
        let noise = Perlin::new();
        noise.set_seed(seed);

        GenerationMaps {
            noise
        }
    }

    /// Generate's the maps for a given location
    pub fn generate(&self, settings: &GeneratorSettings, location: &[i32; 2]) -> WorldMaps {
        // Erosion map
        let mut erosion = [[0.0; CHUNK_SIZE]; CHUNK_SIZE];

        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let pos = calculate_global_pos(location, [x, z]);

                erosion[x][z] = self.noise.get([(pos[0] as f64 * settings.erosion_size / 100.0), pos[1] as f64 * settings.erosion_size / 100.0]);
            }
        }

        //#[cfg(feature = "debug")
        MapVisualiser::visualise_f64_map11(erosion.clone(), location, "erosion_map");

        WorldMaps {
            erosion
        }
    }
}

/// Stores maps relevant to generation data
pub(crate) struct WorldMaps {
    /// Erosion map determines how much erosion has taken placed and thus how flat the terrain will be
    erosion: [[f64; CHUNK_SIZE]; CHUNK_SIZE]
}