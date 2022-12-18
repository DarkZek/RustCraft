use crate::constants::CHUNK_SIZE;
use crate::steps::maps::{GenerationMaps, WorldMaps};
use crate::steps::solid::SolidGeneration;
use crate::{MapVisualiser, WorldGenerator};
use crate::helper::calculate_global_pos;
use noise::NoiseFn;
use crate::generator::GeneratorSettings;

impl SolidGeneration {

    /// Generates a heightmap of the terrain at each x-z point
    pub fn generate_heightmap(&self, settings: &GeneratorSettings, maps: &WorldMaps, location: &[i32; 2]) -> [[u32; CHUNK_SIZE]; CHUNK_SIZE] {
        let mut heightmap = [[0; CHUNK_SIZE]; CHUNK_SIZE];

        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let pos = calculate_global_pos(location, [x, z]);

                let large_details = (self.noise3.get([pos[0] as f64/ 100.0, pos[1] as f64 / 100.0]) * 40.0) as u32 + 110;
                let medium_details = (self.noise1.get([pos[0] as f64/ 25.0, pos[1] as f64 / 25.0]) * 10.0) as u32;
                let small_details = (self.noise2.get([pos[0] as f64/ 10.0, pos[1] as f64 / 10.0]) * 1.0) as u32;

                heightmap[x][z] = large_details + medium_details + small_details;
                println!("{}", heightmap[x][z]);
            }
        }

        //#[cfg(feature = "debug")
        MapVisualiser::visualise_f64_map_heightmap(heightmap.clone(), location, "heightmap");

        heightmap
    }

}