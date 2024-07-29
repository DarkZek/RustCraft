use std::fs;
use nalgebra::Vector3;
use rc_shared::block::BlockStates;
use rc_shared::CHUNK_SIZE;
use crate::systems::chunk::nearby_cache::NearbyChunkCache;
use crate::systems::chunk::nearby_chunk_map::NearbyChunkMap;

pub struct ChunkBuildContext {
    // Location of all lights in the surrounding chunks
    pub lights: Vec<(Vector3<i32>, [u8; 4])>,
    // Translucency of all blocks in the surrounding chunks.
    // TODO: Convert this into a more compressed format?
    pub translucency_map: NearbyChunkMap<bool>
}

// Stores any context a chunk may need to build lighting. Used so that chunk can be chucked at another thread
impl ChunkBuildContext {
    pub fn new(
        chunk_pos: Vector3<i32>,
        states: &BlockStates,
        cache: &NearbyChunkCache,
    ) -> ChunkBuildContext {
        let mut lights = Vec::new();

        let mut translucency_map: NearbyChunkMap<bool> = NearbyChunkMap::new_empty(chunk_pos);

        // Loop over every nearby chunk
        translucency_map.for_each_mut(|mut entry| {
            // TODO: Allow NearbyChunkMap to directly use a NearbyChunkCache to speed up chunk fetching
            let Some(chunk) = cache.get_chunk(entry.chunk_position) else {
                return;
            };

            let block = states.get_block(
                chunk.world[entry.block_position.x][entry.block_position.y][entry.block_position.z] as usize
            );

            *entry.data = block.translucent;

            if block.emission[3] == 0 {
                return;
            }

            lights.push((
                entry.world_position,
                block.emission,
            ));
        });

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