use crate::game::blocks::states::BlockStates;
use crate::game::viewable_direction::{
    calculate_viewable, ViewableDirection, ViewableDirectionBitMap,
};
use crate::services::chunk::data::ChunkData;
use fnv::FnvBuildHasher;
use nalgebra::Vector3;
use rustcraft_protocol::constants::CHUNK_SIZE;
use std::collections::HashMap;

impl<'a> ChunkData {
    /*
       This is a complex function that needs to take the xyz of the block position and the direction to create a value

       15, 15, 15  - 1, 0, 0   = 0, 15, 15
       15, 15, 15  - 0, 0, 1   = 15, 15, 0
       0, 0, 0     - -1, 0, 0  = 15, 0, 0
       0, 0, 0     - 0, 0, -1  = 0, 0, 15
       0, 3, 8     - -1, 0, 0  = 15, 3, 8
       15, 3, 8    - 1, 0, 0   = 0, 3, 8
       15, 8, 8    - 1, 0, 0   = 0, 8, 8

       Here's some pseudo code I made up
       If number equals zero
           If any digits are negative
               return 0
           else
               return corresponding number
       else if number equals one
           return 0
       else if number equals negative one
           return 15
    */

    pub fn generate_viewable_map(
        &self,
        block_states: &BlockStates,
        adjacent_chunks: HashMap<Vector3<i32>, Option<&ChunkData>, FnvBuildHasher>,
        chunk_edge_faces: bool,
    ) -> [[[ViewableDirection; 16]; 16]; 16] {
        let mut data = [[[ViewableDirection(0); CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];

        let directions: [Vector3<i32>; 6] = [
            Vector3::new(1, 0, 0),
            Vector3::new(-1, 0, 0),
            Vector3::new(0, 1, 0),
            Vector3::new(0, -1, 0),
            Vector3::new(0, 0, 1),
            Vector3::new(0, 0, -1),
        ];

        for x in 0..self.world.len() {
            for z in 0..self.world[0][0].len() {
                for y in 0..self.world[0].len() {
                    let block = block_states.get_block(self.world[x][y][z] as usize);

                    let mut viewable = calculate_viewable(block_states, &self, &block, [x, y, z]);

                    for direction in directions.iter() {
                        // Calculates if block is bordering on this direction
                        if (direction.x == 1 && x == 15)
                            || (direction.x == -1 && x == 0)
                            || (direction.y == 1 && y == 15)
                            || (direction.y == -1 && y == 0)
                            || (direction.z == 1 && z == 15)
                            || (direction.z == -1 && z == 0)
                        {
                            // Make it so we get the block on the other chunk closest to our block
                            let block_pos: Vector3<usize> = Vector3::new(
                                if direction.x == 0 {
                                    x
                                } else if direction.x == 1 {
                                    0
                                } else {
                                    15
                                },
                                if direction.y == 0 {
                                    y
                                } else if direction.y == 1 {
                                    0
                                } else {
                                    15
                                },
                                if direction.z == 0 {
                                    z
                                } else if direction.z == 1 {
                                    0
                                } else {
                                    15
                                },
                            );

                            // Checks if the block in an adjacent chunk is transparent
                            if let Some(Some(chunk)) = adjacent_chunks.get(&direction) {
                                let block = {
                                    let block_id =
                                        chunk.world[block_pos.x][block_pos.y][block_pos.z];

                                    block_states.get_block(block_id as usize)
                                };

                                // Check if face visible
                                if block.translucent {
                                    viewable.add_flag(ViewableDirectionBitMap::from(direction));
                                }
                            } else if chunk_edge_faces {
                                viewable.add_flag(ViewableDirectionBitMap::from(direction));
                            }
                        }
                    }

                    data[x][y][z] = viewable;
                }
            }
        }

        // Check top faces
        data
    }
}
