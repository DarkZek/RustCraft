use crate::game::generation::noise::SimplexNoise;
use crate::helpers::global_to_local_position;

use nalgebra::Vector3;
use rc_shared::biome::ChunkEnvironment;
use rc_shared::chunk::RawChunkData;
use rc_shared::CHUNK_SIZE;

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
                    spawn_tree(seed, world_pos, absolute_generation, world);
                }
            }
        }
    }
}

fn spawn_tree(seed: u32, chunk_pos: Vector3<i32>, pos: Vector3<i32>, world: &mut RawChunkData) {
    try_place_block(chunk_pos, world, pos, 4);
    try_place_block(chunk_pos, world, pos + Vector3::new(0, 1, 0), 4);
    try_place_block(chunk_pos, world, pos + Vector3::new(0, 2, 0), 4);
    try_place_block(chunk_pos, world, pos + Vector3::new(0, 3, 0), 4);

    try_place_block(chunk_pos, world, pos + Vector3::new(1, 3, 0), 5);
    try_place_block(chunk_pos, world, pos + Vector3::new(1, 3, -1), 5);
    try_place_block(chunk_pos, world, pos + Vector3::new(1, 3, 1), 5);
    try_place_block(chunk_pos, world, pos + Vector3::new(0, 3, 1), 5);
    try_place_block(chunk_pos, world, pos + Vector3::new(0, 3, -1), 5);
    try_place_block(chunk_pos, world, pos + Vector3::new(-1, 3, 0), 5);
    try_place_block(chunk_pos, world, pos + Vector3::new(-1, 3, -1), 5);
    try_place_block(chunk_pos, world, pos + Vector3::new(-1, 3, 1), 5);

    try_place_block(chunk_pos, world, pos + Vector3::new(1, 4, 0), 5);
    try_place_block(chunk_pos, world, pos + Vector3::new(1, 4, -1), 5);
    try_place_block(chunk_pos, world, pos + Vector3::new(1, 4, 1), 5);
    try_place_block(chunk_pos, world, pos + Vector3::new(0, 4, 1), 5);
    try_place_block(chunk_pos, world, pos + Vector3::new(0, 4, -1), 5);
    try_place_block(chunk_pos, world, pos + Vector3::new(0, 4, 0), 5);
    try_place_block(chunk_pos, world, pos + Vector3::new(-1, 4, 0), 5);
    try_place_block(chunk_pos, world, pos + Vector3::new(-1, 4, -1), 5);
    try_place_block(chunk_pos, world, pos + Vector3::new(-1, 4, 1), 5);

    try_place_block(chunk_pos, world, pos + Vector3::new(0, 5, 0), 5);
}

fn try_place_block(
    chunk_pos: Vector3<i32>,
    world: &mut RawChunkData,
    pos: Vector3<i32>,
    block: u32,
) {
    let (block_chunk_pos, block_local_pos) = global_to_local_position(pos);

    // Not same chunk
    if chunk_pos != block_chunk_pos {
        return;
    }

    world[block_local_pos.x][block_local_pos.y][block_local_pos.z] = block;
}
