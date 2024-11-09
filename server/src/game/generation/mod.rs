mod noise;
mod phase1;
mod phase2;
mod phase3;
mod phase4;
mod structures;

use std::time::Instant;

use crate::game::chunk::ChunkData;
use crate::game::generation::phase1::{EnvironmentMapConfig, generate_environment_map};
use crate::game::generation::phase2::generate_greybox_chunk;
use crate::game::generation::phase3::decorate_chunk;
use crate::game::generation::phase4::add_structures;
use bevy::prelude::{Resource, trace};
use nalgebra::Vector3;
use rc_shared::chunk::ChunkDataStorage;
use rc_shared::CHUNK_SIZE;

#[derive(Resource, Default)]
pub struct ChunkGenerationConfig {
    pub environment_map_config: EnvironmentMapConfig
}

impl ChunkData {
    /// Works in 4 phases
    /// Phase 1: Biome Generation
    ///     This affects all the other phases so it is performed first
    /// Phase 2: Greyboxing
    ///     Stone blocks are placed for the ground, holes are left for caves
    /// Phase 3: Decoration
    ///     More block types are added, such as grass, dirt, sand, water
    /// Phase 4: Structures
    ///     Structures are generated in this step such as trees
    pub fn generate(
        position: Vector3<i32>,
        config: &ChunkGenerationConfig
    ) -> ChunkData {

        let started = Instant::now();

        let seed = 0;
        let environment_map = generate_environment_map(
            seed,
            position,
            &config.environment_map_config
        );

        let (mut chunk_data, heightmap) = generate_greybox_chunk(seed, position, &environment_map);

        decorate_chunk(seed, position, &mut chunk_data, &heightmap, &environment_map);

        add_structures(seed, position, &mut chunk_data, &heightmap, &environment_map);
        
        trace!("Took {}ms to build chunk", started.elapsed().as_millis());

        let mut data = ChunkData::new(
            position,
            ChunkDataStorage::Data(Box::new(chunk_data)),
            Default::default(),
            Default::default()
        );

        data.optimise_data();

        data
    }

    pub fn generate_canvas(position: Vector3<i32>) -> ChunkData {

        let y_plane = 0;
        let block_id = 1;

        let chunk_data = if position.y == 0 {
            let mut data = ChunkDataStorage::Data(Box::new([[[0; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]));

            for x in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    data.set(Vector3::new(x, y_plane, z), block_id);
                }
            }

            data
        } else {
            ChunkDataStorage::Empty
        };

        let data = ChunkData::new(
            position,
            chunk_data,
            Default::default(),
            Default::default()
        );

        data
    }
}
