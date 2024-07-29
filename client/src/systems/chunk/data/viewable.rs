use crate::systems::chunk::data::ChunkData;
use nalgebra::Vector3;
use rc_shared::block::BlockStates;
use rc_shared::viewable_direction::{
    calculate_chunk_viewable, ViewableDirection, ViewableDirectionBitMap,
};
use rc_shared::CHUNK_SIZE;
use crate::systems::chunk::builder::build_context::ChunkBuildContext;

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
        build_context: &ChunkBuildContext,
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

                    let mut viewable =
                        calculate_chunk_viewable(block_states, &self.world, &block, [x, y, z]);

                    // Catch any directions not inside the current chunk
                    for direction in directions.iter() {
                        // Calculates if block is bordering on this direction
                        if (direction.x == 1 && x == 15)
                            || (direction.x == -1 && x == 0)
                            || (direction.y == 1 && y == 15)
                            || (direction.y == -1 && y == 0)
                            || (direction.z == 1 && z == 15)
                            || (direction.z == -1 && z == 0)
                        {

                            let position = (self.position * CHUNK_SIZE as i32) +
                                Vector3::new(x as i32, y as i32, z as i32) +
                                direction;

                            // Checks if the block in an adjacent chunk is transparent
                            if let Some(chunk) = build_context.surrounding_data.get(&position) {
                                // Check if face visible
                                if chunk.is_transparent {
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
