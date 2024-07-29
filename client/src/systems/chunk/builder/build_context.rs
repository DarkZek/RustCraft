use nalgebra::Vector3;
use rc_shared::block::BlockStates;
use rc_shared::CHUNK_SIZE;
use crate::systems::chunk::nearby_cache::NearbyChunkCache;

pub struct ChunkBuildContext {
    // Location of all lights in the surrounding chunks
    pub lights: Vec<(Vector3<i32>, [u8; 4])>,
    // Translucency of all blocks in the surrounding chunks.
    // TODO: Convert this into a more compressed format?
    pub translucency_map: [[[bool; CHUNK_SIZE * 3]; CHUNK_SIZE * 3]; CHUNK_SIZE * 3]
}

// Stores any context a chunk may need to build lighting. Used so that chunk can be chucked at another thread
impl ChunkBuildContext {
    pub fn new(
        chunk_pos: Vector3<i32>,
        states: &BlockStates,
        cache: &NearbyChunkCache,
    ) -> ChunkBuildContext {
        let mut lights = Vec::new();

        let mut translucency_map = [[[false; 48]; 48]; 48];

        for chunk_x in (chunk_pos.x - 1)..=(chunk_pos.x + 1) {
            for chunk_y in (chunk_pos.y - 1)..=(chunk_pos.y + 1) {
                for chunk_z in (chunk_pos.z - 1)..=(chunk_pos.z + 1) {
                    // Get chunk
                    if let Some(chunk) = cache.get_chunk(Vector3::new(chunk_x, chunk_y, chunk_z)) {
                        for x in 0..CHUNK_SIZE {
                            for y in 0..CHUNK_SIZE {
                                for z in 0..CHUNK_SIZE {
                                    let block = states.get_block(chunk.world[x][y][z] as usize);

                                    let block_position = Vector3::new(x, y, z).cast::<i32>()
                                        + (CHUNK_SIZE as i32 * chunk.position);

                                    // Position in 3x3 array of chunk data
                                    let local_relative_position =
                                        block_position - (CHUNK_SIZE as i32 * chunk_pos)
                                            + Vector3::new(CHUNK_SIZE as i32, CHUNK_SIZE as i32, CHUNK_SIZE as i32);

                                    translucency_map
                                        [local_relative_position.x as usize]
                                        [local_relative_position.y as usize]
                                        [local_relative_position.z as usize] = block.translucent;

                                    if block.emission[3] == 0 {
                                        continue;
                                    }

                                    lights.push((
                                        block_position,
                                        block.emission,
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        }

        // if lights.len() > 0 {
        //     println!("Get lights {}", lights.len());
        //
        //     for light in &lights {
        //         println!("Found light at {:?}", light.0);
        //     }
        // }

        ChunkBuildContext {
            lights,
            translucency_map,
        }
    }
}