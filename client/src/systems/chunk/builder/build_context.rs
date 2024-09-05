use std::collections::HashMap;
use fnv::{FnvBuildHasher, FnvHashMap};
use nalgebra::Vector3;
use rc_shared::block::BlockStates;
use rc_shared::chunk::LightingColor;
use crate::systems::chunk::nearby_cache::NearbyChunkCache;
use crate::systems::chunk::nearby_chunk_map::NearbyChunkMap;

pub struct ChunkBuildContext {
    // Location of all lights in the surrounding chunks
    pub lights: Vec<(Vector3<i32>, [u8; 4])>,
    // Translucency of all blocks in the surrounding chunks.
    // TODO: Convert this into a more compressed format?
    pub translucency_map: NearbyChunkMap<bool>,
    // Lighting & visibility data for a 1 block shell around the chunk
    pub surrounding_data: HashMap<Vector3<i32>, ChunkBuildContextNeighborBlockData, FnvBuildHasher>
}

#[derive(Debug, Clone)]
pub struct ChunkBuildContextNeighborBlockData {
    pub light: LightingColor,
    pub is_transparent: bool
}

// Stores any context a chunk may need to build lighting. Used so that chunk can be chucked at another thread
impl ChunkBuildContext {
    pub fn new(
        states: &BlockStates,
        cache: &NearbyChunkCache,
    ) -> ChunkBuildContext {
        let chunk_pos = cache.position();

        let mut lights = Vec::new();
        let mut translucency_map: NearbyChunkMap<bool> = NearbyChunkMap::new_empty(chunk_pos);
        let mut surrounding_data = FnvHashMap::default();

        // Loop over every nearby chunk
        translucency_map.for_each_mut_with_chunks(cache, |entry| {
            let Some(chunk) = entry.chunk else {
                return;
            };

            let block = states.get_block(
                chunk.world.get(entry.block_position) as usize
            );

            *entry.data = block.translucent;

            // Store all lights in surrounding chunks
            if block.emission[3] != 0 {
                lights.push((
                    entry.world_position,
                    block.emission,
                ));
            }

            // Store blocks that touch this chunk, for lighting and culling
            if is_neighbor_block(entry.chunk_position - chunk_pos, entry.block_position) {
                surrounding_data.insert(entry.world_position,
                    ChunkBuildContextNeighborBlockData {
                        light: [0; 4],
                        is_transparent: block.translucent,
                    }
                );
            }
        });

        ChunkBuildContext {
            lights,
            translucency_map,
            surrounding_data,
        }
    }
}

fn is_neighbor_block(relative_chunk_pos: Vector3<i32>, block_pos: Vector3<usize>) -> bool {

    // Rule out the chunk itself, and its diagonal neighbors
    if relative_chunk_pos.x.abs() + relative_chunk_pos.y.abs() + relative_chunk_pos.z.abs() != 1 {
        return false;
    }

    // Now relative_chunk_pos is a unit vector in the direction of the chunk

    // Figure out what axis is touching the parent chunk
    let axis_index = if relative_chunk_pos.x.abs() != 0 {
        0
    } else if relative_chunk_pos.y.abs() != 0 {
        1
    } else {
        2
    };

    // If the chunk is positive direction, the neighbor block will have 0 has its value
    // If the chunk is negative direction, the neighbor block will be at 15
    let expected_block_pos = if relative_chunk_pos.data.0[0][axis_index] > 0 {
        0
    } else {
        15
    };

    block_pos.data.0[0][axis_index] == expected_block_pos
}

#[cfg(test)]
mod tests {
    use fnv::FnvHashMap;
    use nalgebra::Vector3;
    use rc_shared::block::BlockStates;
    use rc_shared::block::types::Block;
    use rc_shared::viewable_direction::ViewableDirectionBitMap;
    use crate::systems::chunk::builder::build_context::{ChunkBuildContext, is_neighbor_block};
    use crate::systems::chunk::data::ChunkData;
    use crate::systems::chunk::nearby_cache::NearbyChunkCache;

    #[test]
    fn neighbor_test_cases() {
        assert!(is_neighbor_block(Vector3::new(0, 0, -1), Vector3::new(0, 0, 15)));
        assert!(is_neighbor_block(Vector3::new(0, 0, 1), Vector3::new(0, 0, 0)));
        assert!(is_neighbor_block(Vector3::new(1, 0, 0), Vector3::new(0, 5, 15)));
        assert!(is_neighbor_block(Vector3::new(0, -1, 0), Vector3::new(0, 15, 2)));
        assert!(!is_neighbor_block(Vector3::new(0, -1, 0), Vector3::new(0, 14, 0)));
        assert!(!is_neighbor_block(Vector3::new(0, -1, 0), Vector3::new(0, 0, 0)));
        assert!(!is_neighbor_block(Vector3::new(1, 0, 1), Vector3::new(0, 15, 0)));
        assert!(!is_neighbor_block(Vector3::new(1, 0, 1), Vector3::new(0, 0, 0)));
        assert!(!is_neighbor_block(Vector3::new(1, 0, 1), Vector3::new(0, 0, 10)));
    }

    #[test]
    fn context_test_cases() {

        let mut states = BlockStates::new();

        states.states.push(Block {
            identifier: "mcv3::Air".to_string(),
            translucent: true,
            full: false,
            draw_betweens: false,
            faces: vec![],
            collision_boxes: vec![],
            bounding_boxes: vec![],
            emission: [0; 4],
        });

        states.states.push(Block {
            identifier: "mcv3::Stone".to_string(),
            translucent: false,
            full: true,
            draw_betweens: false,
            faces: vec![],
            collision_boxes: vec![],
            bounding_boxes: vec![],
            emission: [0; 4],
        });

        let mut chunks = FnvHashMap::default();

        // Three chunks one at 0,0,0 and one at 0,1,0 and one at -1,0,0
        let data = [[[0; 16]; 16]; 16];
        chunks.insert(Vector3::new(0, 0, 0), ChunkData::new_handleless(
            data,
            Vector3::new(0, 0, 0)
        ));

        let mut data = [[[0; 16]; 16]; 16];

        data[0][0][0] = 1;
        data[0][1][0] = 1;

        chunks.insert(Vector3::new(0, 1, 0), ChunkData::new_handleless(
            data,
            Vector3::new(0, 1, 0)
        ));

        let mut data = [[[0; 16]; 16]; 16];

        data[15][0][0] = 1;

        chunks.insert(Vector3::new(-1, 0, 0), ChunkData::new_handleless(
            data,
            Vector3::new(-1, 0, 0)
        ));

        let chunk_cache = NearbyChunkCache::from_map(&chunks, Vector3::new(0, 0, 0));

        let context = ChunkBuildContext::new(
            &states,
            &chunk_cache
        );

        assert!(context.surrounding_data.get(&Vector3::new(0, 16, 0)).is_some());
        assert!(context.surrounding_data.get(&Vector3::new(0, 17, 0)).is_none());
        assert!(context.surrounding_data.get(&Vector3::new(-1, 0, 0)).is_some());
        assert!(context.surrounding_data.get(&Vector3::new(0, 0, 0)).is_none());
    }

    #[test]
    fn context_viewable_cases() {

        let mut states = BlockStates::new();

        states.states.push(Block {
            identifier: "mcv3::Air".to_string(),
            translucent: true,
            full: false,
            draw_betweens: false,
            faces: vec![],
            collision_boxes: vec![],
            bounding_boxes: vec![],
            emission: [0; 4],
        });

        states.states.push(Block {
            identifier: "mcv3::Stone".to_string(),
            translucent: false,
            full: true,
            draw_betweens: false,
            faces: vec![],
            collision_boxes: vec![],
            bounding_boxes: vec![],
            emission: [0; 4],
        });

        let mut chunks = FnvHashMap::default();

        // Three chunks one at 0,0,0 and one at 0,1,0 and one at -1,0,0
        let data = [[[0; 16]; 16]; 16];
        chunks.insert(Vector3::new(0, 0, 4), ChunkData::new_handleless(
            data,
            Vector3::new(0, 0, 4)
        ));

        let mut data = [[[0; 16]; 16]; 16];

        data[0][0][0] = 1;
        data[0][1][0] = 1;

        chunks.insert(Vector3::new(0, 1, 4), ChunkData::new_handleless(
            data,
            Vector3::new(0, 1, 4)
        ));

        let mut data = [[[0; 16]; 16]; 16];

        data[15][0][0] = 1;

        chunks.insert(Vector3::new(-1, 0, 4), ChunkData::new_handleless(
            data,
            Vector3::new(-1, 0, 4)
        ));

        let chunk_cache = NearbyChunkCache::from_map(&chunks, Vector3::new(0, 0, 4));

        let context = ChunkBuildContext::new(
            &states,
            &chunk_cache
        );

        //println!("{:?}", context.surrounding_data);

        let main_chunk = chunks.get(&Vector3::new(0,0,4)).unwrap();

        let visibility_map = main_chunk.generate_viewable_map(
            &states,
            &context,
            false
        );

        assert!(!visibility_map[0][15][0].has_flag(ViewableDirectionBitMap::Top));
        assert!(visibility_map[1][15][0].has_flag(ViewableDirectionBitMap::Top));
    }


}