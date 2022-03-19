use crate::block::blocks::BlockType;
use crate::services::chunk_service::chunk::{ChunkData, ChunkEntityLookup};
use crate::services::settings_service::CHUNK_SIZE;
use nalgebra::Vector3;
use specs::{Read, ReadStorage};

pub mod player_selected_block_update;
pub mod raycast;

/// This struct holds references to the chunks of the world and allows transformations and lookups to be made across chunk boundaries
pub struct WorldChunks<'a> {
    chunks: &'a ReadStorage<'a, ChunkData>,
    lookup: &'a Read<'a, ChunkEntityLookup>,
    // Generally multiple chunk lookups are close by each other and are in the same chunk, so cache the previous chunk to prevent hashmap lookup
    last_chunk: Option<&'a ChunkData>,
}

impl<'a> WorldChunks<'a> {
    pub fn new(
        chunks: &'a ReadStorage<ChunkData>,
        lookup: &'a Read<ChunkEntityLookup>,
    ) -> WorldChunks<'a> {
        WorldChunks {
            chunks,
            lookup,
            last_chunk: None,
        }
    }

    pub fn get_block(&mut self, pos: Vector3<i64>) -> Option<&'a BlockType> {
        let chunk_pos = absolute_pos_to_chunk(pos);

        // Try load previous chunk
        let chunk_data =
            if self.last_chunk.is_some() && self.last_chunk.unwrap().position == chunk_pos {
                self.last_chunk.unwrap()
            } else {
                // Lookup chunk
                if let Some(chunk_entity) = self.lookup.map.get(&chunk_pos) {
                    if let Some(chunk_data) = self.chunks.get(*chunk_entity) {
                        self.last_chunk = Some(chunk_data);
                        chunk_data
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            };

        let x = ((pos.x % CHUNK_SIZE as i64) + CHUNK_SIZE as i64) as usize % CHUNK_SIZE;
        let y = ((pos.y % CHUNK_SIZE as i64) + CHUNK_SIZE as i64) as usize % CHUNK_SIZE;
        let z = ((pos.z % CHUNK_SIZE as i64) + CHUNK_SIZE as i64) as usize % CHUNK_SIZE;

        if let Some(block) = chunk_data.get_block(Vector3::new(x as usize, y as usize, z as usize))
        {
            if block.block_type == &(BlockType::Air {}) {
                return None;
            }

            return Some(block.block_type);
        } else {
            return None;
        }
    }

    pub fn get_block_uncached(&self, pos: Vector3<i64>) -> Option<&'a BlockType> {
        let chunk_pos = absolute_pos_to_chunk(pos);

        if let Some(chunk_entity) = self.lookup.map.get(&chunk_pos) {
            if let Some(chunk_data) = self.chunks.get(*chunk_entity) {
                let x = ((pos.x % CHUNK_SIZE as i64) + CHUNK_SIZE as i64) as usize % CHUNK_SIZE;
                let y = ((pos.y % CHUNK_SIZE as i64) + CHUNK_SIZE as i64) as usize % CHUNK_SIZE;
                let z = ((pos.z % CHUNK_SIZE as i64) + CHUNK_SIZE as i64) as usize % CHUNK_SIZE;

                if let Some(block) =
                    chunk_data.get_block(Vector3::new(x as usize, y as usize, z as usize))
                {
                    if block.block_type == &(BlockType::Air {}) {
                        return None;
                    }

                    return Some(block.block_type);
                } else {
                    return None;
                }
            }
        }

        return None;
    }
}

fn absolute_pos_to_chunk(pos: Vector3<i64>) -> Vector3<i32> {
    Vector3::new(
        (pos.x as f64 / CHUNK_SIZE as f64).floor() as i32,
        (pos.y as f64 / CHUNK_SIZE as f64).floor() as i32,
        (pos.z as f64 / CHUNK_SIZE as f64).floor() as i32,
    )
}
