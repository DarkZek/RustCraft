use crate::game::chunk::RawChunkData;
use crate::game::generation::noise::SimplexNoise;
use crate::helpers::global_to_local_position;
use nalgebra::Vector3;
use rc_client::systems::chunk::biome::{ChunkEnvironment, EnvironmentEntry, Vegetation};
use rc_networking::constants::CHUNK_SIZE;

pub fn add_structures_to_chunk(
    seed: u32,
    pos: Vector3<i32>,
    world: &mut RawChunkData,
    heightmap: &[[i32; CHUNK_SIZE]; CHUNK_SIZE],
    environment_map: &ChunkEnvironment,
) {
    let tree_noise = SimplexNoise::new(seed + 10).with_scale(2.0);

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let absolute = Vector3::new(
                    (pos.x * 16) + x as i32,
                    (pos.y * 16) + y as i32,
                    (pos.z * 16) + z as i32,
                );

                let ground_level = heightmap[x][z];

                let environment = &environment_map[x][y][z];

                // Trees
                if absolute.y == ground_level + 1
                    && environment.vegetation > 0.5
                    && tree_noise.sample_2d(absolute.x, absolute.z)
                        > 1.0 - ((environment.vegetation - 0.5) * 0.8)
                {
                    spawn_tree(seed, pos, absolute, world, heightmap, environment);
                }
            }
        }
    }
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

fn spawn_tree(
    seed: u32,
    chunk_pos: Vector3<i32>,
    pos: Vector3<i32>,
    world: &mut RawChunkData,
    heightmap: &[[i32; CHUNK_SIZE]; CHUNK_SIZE],
    environment: &EnvironmentEntry,
) {
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
