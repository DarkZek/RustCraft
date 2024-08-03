use crate::game::generation::noise::SimplexNoise;
use nalgebra::Vector3;
use rc_shared::biome::ChunkEnvironment;
use rc_shared::chunk::RawChunkData;
use rc_shared::CHUNK_SIZE;
use crate::game::generation::structures::StructureGenerator;
use crate::game::generation::structures::tree::TreeStructureGenerator;

pub fn add_structures(seed: u32, pos: Vector3<i32>, world: &mut RawChunkData, heightmap: &[[i32; CHUNK_SIZE]; CHUNK_SIZE], environment: ChunkEnvironment) {
    // TODO: Take in surrounding chunks `ChunkEnvironment`

    add_structures_to_chunk(seed, pos, pos, world, &heightmap, &environment);
}

/// Adds structures to chunk
/// `world_pos` is the position of `world`, while `generation_pos` is a surrounding chunk that is having its structures generated also to generate overlap
/// in the chunks structures.
pub fn add_structures_to_chunk(
    seed: u32,
    generation_pos: Vector3<i32>,
    world_pos: Vector3<i32>,
    world: &mut RawChunkData,
    heightmap: &[[i32; CHUNK_SIZE]; CHUNK_SIZE],
    environment: &ChunkEnvironment,
) {
    let tree_noise = SimplexNoise::new(seed + 10).with_scale(2.0);

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let absolute_generation = Vector3::new(
                    (generation_pos.x * 16) + x as i32,
                    (generation_pos.y * 16) + y as i32,
                    (generation_pos.z * 16) + z as i32,
                );

                let ground_level = heightmap[x][z];

                // Trees in an falling off manner
                if absolute_generation.y == ground_level + 1
                    && *environment == ChunkEnvironment::FOREST
                    && tree_noise.sample_2d(absolute_generation.x, absolute_generation.z)
                        > 0.8
                {
                    TreeStructureGenerator::spawn(seed, world_pos, absolute_generation, world);
                }
            }
        }
    }
}
