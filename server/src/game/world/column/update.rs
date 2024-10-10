use std::iter::{Iterator, Rev};
use std::ops::Range;
use bevy::prelude::info;
use nalgebra::{Vector2, Vector3};
use rc_shared::chunk_column::ChunkColumnData;
use rc_shared::CHUNK_SIZE;
use rc_shared::helpers::global_to_local_position;
use crate::game::world::data::WorldData;


const CHUNK_CHECKING_RANGE: [i32; 21] = [10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, -1, -2, -3, -4, -5, -6, -7, -8, -9, -10]; // (-10..10).rev()

impl WorldData {
    // Loops over chunks in a column and populates the data for its ChunkColumnData
    // Currently this is skylight levels
    pub fn update_column(&mut self, position: Vector2<i32>) {
        let mut new_skylight_level = [[None; CHUNK_SIZE]; CHUNK_SIZE];

        'vertical_checking: for chunk_y in CHUNK_CHECKING_RANGE {
            if let Some(chunk) = self.chunks.get(&Vector3::new(position.x, chunk_y, position.y)) {
                for x in 0..CHUNK_SIZE {
                    for z in 0..CHUNK_SIZE {
                        // If skylight already set
                        if new_skylight_level[x][z].is_some() {
                            continue
                        }

                        'vertical: for y in (0..CHUNK_SIZE).rev() {
                            if chunk.world.get(Vector3::new(x, y, z)) != 0 {
                                new_skylight_level[x][z] = Some(y as i32 + (chunk_y as i32 * CHUNK_SIZE as i32) + 1);
                                break 'vertical;
                            }
                        }
                    }
                }
            }

            // Can we cancel early
            for x in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    if new_skylight_level[x][z].is_none() {
                        // Try next chunk
                        continue 'vertical_checking;
                    }
                }
            }

            // All skylight have been set, break early
            break
        }


        // Update
        if self.chunks_columns.get(&position).is_none() {
            self.chunks_columns.insert(position, ChunkColumnData::default());
        }

        let column_data = self.chunks_columns.get_mut(&position).unwrap();

        column_data.skylight_level = new_skylight_level;
        column_data.dirty = true;
    }

    // Block position updated at `pos`, update the column data
    pub fn update_column_pos(&mut self, position: Vector3<i32>, block_id: usize) {

        let (chunk_pos, block_pos) = global_to_local_position(position);

        let column_data = self.chunks_columns.get_mut(&Vector2::new(chunk_pos.x, chunk_pos.z)).unwrap();

        if block_id == 0 {
            // Block destroyed, find new highest new block
            column_data.skylight_level[block_pos.x][block_pos.z] = None;
            column_data.dirty = true;

            'outer: for chunk_y in CHUNK_CHECKING_RANGE {
                if let Some(chunk) = self.chunks.get(&Vector3::new(chunk_pos.x, chunk_y, chunk_pos.z)) {
                    for y in CHUNK_SIZE..0 {
                        let block_id = chunk.world.get(Vector3::new(block_pos.x, y, block_pos.z));

                        if block_id != 0 {
                            column_data.skylight_level[block_pos.x][block_pos.z] = Some(position.y + 1);
                            break 'outer;
                        }
                    }
                }
            }
        } else {
            // Block placed
            let current_height_pos = column_data.skylight_level[block_pos.x][block_pos.z];

            if current_height_pos.is_none() || current_height_pos.unwrap() > position.y {
                column_data.skylight_level[block_pos.x][block_pos.z] = Some(position.y + 1);
                column_data.dirty = true;
            }
        }
    }
}