use crate::constants::{PartialChunkData, RawChunkData, CHUNK_SIZE};
use bevy_ecs::prelude::Component;
use naia_shared::{Property, Replicate};

/// How many blocks are sent per partial update packet
pub const CHUNK_UPDATE_BLOCKS_PER_PACKET: usize = 256;

/// How many partial chunks it takes to make up a full chunk
pub const CHUNK_UPDATE_PARTIAL_CHUNKS: usize =
    (CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE) / CHUNK_UPDATE_BLOCKS_PER_PACKET;

#[derive(Replicate, Component)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct PartialChunkUpdate {
    pub data: Property<PartialChunkData>,
    pub x: Property<i32>,
    pub y: Property<i32>,
    pub z: Property<i32>,
    pub section: Property<u8>,
}

impl PartialChunkUpdate {
    pub fn new(data: PartialChunkData, x: i32, y: i32, z: i32, section: u8) -> Self {
        PartialChunkUpdate::new_complete(data, x, y, z, section)
    }

    pub fn generate(data: &RawChunkData, position: [i32; 3]) -> Vec<PartialChunkUpdate> {
        let mut updates = Vec::with_capacity(CHUNK_UPDATE_PARTIAL_CHUNKS);

        let mut split_data = [[0; CHUNK_UPDATE_BLOCKS_PER_PACKET]; CHUNK_UPDATE_PARTIAL_CHUNKS];

        println!("{}", CHUNK_UPDATE_PARTIAL_CHUNKS);

        let mut i = 0;
        let mut packet_number = -1;

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    if i % CHUNK_UPDATE_BLOCKS_PER_PACKET == 0 {
                        packet_number += 1;
                    }
                    split_data[packet_number as usize][i % CHUNK_UPDATE_BLOCKS_PER_PACKET] =
                        data[x][y][z] as u8;
                    i += 1;
                }
            }
        }

        // Loop over every quarter
        for portion in split_data {
            // Loop over that quarter of the blocks

            updates.push(PartialChunkUpdate::new(
                portion,
                position[0],
                position[1],
                position[2],
                i as u8,
            ));

            i += 1;
        }

        updates
    }
}
