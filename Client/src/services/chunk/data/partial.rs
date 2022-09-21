use crate::{info, ChunkData, Component, Entity};
use bevy_testing_protocol::constants::CHUNK_SIZE;
use bevy_testing_protocol::protocol::clientbound::chunk_update::{
    PartialChunkUpdate, CHUNK_UPDATE_BLOCKS_PER_PACKET, CHUNK_UPDATE_PARTIAL_CHUNKS,
};
use nalgebra::{Vector, Vector3};
use std::collections::HashMap;

#[derive(Default)]
pub struct PartialChunks {
    data: HashMap<Vector3<i32>, Vec<PartialChunkUpdate>>,
}

impl PartialChunks {
    pub fn new() -> PartialChunks {
        PartialChunks {
            data: HashMap::new(),
        }
    }

    /// Checks if there is enough data to reconstruct a chunk
    pub fn is_complete(&self, location: Vector3<i32>) -> bool {
        let mut indices = [true; CHUNK_UPDATE_PARTIAL_CHUNKS];

        for part in self.data.get(&location).unwrap() {
            indices[*part.section as usize] = false;
        }

        for index in indices {
            // An index was left untouched, no data so not complete yet
            if index {
                return false;
            }
        }

        return true;
    }

    /// Creates a chunk from the partial pieces
    /// Assumes PartialChunks::is_complete is true
    pub fn create_chunk(&mut self, location: Vector3<i32>, entity: Entity) -> ChunkData {
        let mut new_data = [[[0; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];

        let data = self.data.get(&location).unwrap();

        let mut i = 0;
        let mut packet_number = -1;
        // Default value never used
        let mut active_packet = data.get(0).unwrap();

        // Create chunk from packets
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    if i % CHUNK_UPDATE_BLOCKS_PER_PACKET == 0 {
                        packet_number += 1;
                        // Set new active packet
                        active_packet = data
                            .get(self.get_part(location, packet_number as u8))
                            .unwrap();
                    }

                    new_data[x][y][z] = *active_packet
                        .data
                        .get(i % CHUNK_UPDATE_BLOCKS_PER_PACKET)
                        .unwrap() as u32;

                    i += 1;
                }
            }
        }

        // Clear old data
        self.data.remove(&location);

        ChunkData::new(new_data, entity, location)
    }

    fn get_part<'a>(&self, location: Vector3<i32>, id: u8) -> usize {
        for (i, part) in self.data.get(&location).unwrap().iter().enumerate() {
            if *part.section == id {
                return i;
            }
        }

        panic!("Partial Chunk did not contain all indexes. Missing {}", id);
    }

    pub fn add(&mut self, update: PartialChunkUpdate) {
        let location = Vector3::new(*update.x, *update.y, *update.z);

        if !self.data.contains_key(&location) {
            self.data.insert(location, Vec::new());
        }

        self.data.get_mut(&location).unwrap().push(update);
    }
}
