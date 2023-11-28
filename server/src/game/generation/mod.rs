mod noise;
mod phase1;
mod phase2;
mod phase3;
mod phase4;

use crate::game::chunk::ChunkData;
use crate::game::generation::phase1::generate_environment_map;
use crate::game::generation::phase2::generate_greybox_chunk;
use crate::game::generation::phase3::decorate_chunk;
use crate::game::generation::phase4::add_structures;
use nalgebra::Vector3;
use rc_shared::CHUNK_SIZE;

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
    pub fn generate(position: Vector3<i32>) -> ChunkData {
        let seed = 0;
        let environment = generate_environment_map(
            seed,
            Vector3::new(
                position.x * CHUNK_SIZE as i32,
                position.y * CHUNK_SIZE as i32,
                position.z * CHUNK_SIZE as i32,
            ),
        );

        let (mut chunk_data, heightmap) = generate_greybox_chunk(seed, position, &environment);

        decorate_chunk(seed, position, &mut chunk_data, &heightmap, &environment);

        add_structures(seed, position, &mut chunk_data);

        ChunkData {
            position,
            world: chunk_data,
        }
    }
}
