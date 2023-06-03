use crate::constants::{RawChunkData, CHUNK_SIZE};
use std::mem;
use std::sync::atomic::{AtomicU64, Ordering};

/// How many blocks are sent per partial update packet
pub const CHUNK_UPDATE_BLOCKS_PER_PACKET: usize = 256;

/// How many partial chunks it takes to make up a full chunk
pub const CHUNK_UPDATE_PARTIAL_CHUNKS: usize =
    (CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE) / CHUNK_UPDATE_BLOCKS_PER_PACKET;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
#[repr(C)]
pub struct FullChunkUpdate {
    pub data: RawChunkData,
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

pub const PARTIAL_CHUNK_UPDATE_SIZE: usize = 100;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[repr(C)]
pub struct PartialChunkUpdate {
    pub data: Vec<u32>,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub id: u64,
    pub number: u8,
}

const PARTIAL_CHUNK_UPDATE_ID: AtomicU64 = AtomicU64::new(0);

impl FullChunkUpdate {
    pub fn new(data: RawChunkData, x: i32, y: i32, z: i32) -> Self {
        FullChunkUpdate { data, x, y, z }
    }

    pub fn to_partial(&self) -> Vec<PartialChunkUpdate> {
        let mut updates = Vec::new();

        let mut current_update = vec![0; PARTIAL_CHUNK_UPDATE_SIZE];

        let id = PARTIAL_CHUNK_UPDATE_ID.fetch_add(1, Ordering::Acquire);
        let mut number = 0;

        let mut i = 0;
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    if i % PARTIAL_CHUNK_UPDATE_SIZE == 0 && i != 0 {
                        updates.push(PartialChunkUpdate {
                            data: current_update,
                            x: self.x,
                            y: self.y,
                            z: self.z,
                            id,
                            number,
                        });
                        current_update = vec![0; PARTIAL_CHUNK_UPDATE_SIZE];
                        number += 1;
                    }

                    current_update[i % PARTIAL_CHUNK_UPDATE_SIZE] = self.data[x][y][z];

                    i += 1;
                }
            }
        }

        updates.push(PartialChunkUpdate {
            data: current_update,
            x: self.x,
            y: self.y,
            z: self.z,
            id,
            number,
        });

        updates
    }

    pub fn from_partial(mut partials: Vec<PartialChunkUpdate>) -> Option<FullChunkUpdate> {
        let needed_chunks = ((CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE) as f32
            / PARTIAL_CHUNK_UPDATE_SIZE as f32)
            .ceil() as usize;

        assert_eq!(needed_chunks, partials.len());

        partials.sort_by(|p1, p2| p1.number.cmp(&p2.number));

        let mut data = [[[0; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];

        let mut partial_chunk = partials.remove(0);

        let mut i = 0;
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    if i != 0 && i % PARTIAL_CHUNK_UPDATE_SIZE == 0 {
                        partial_chunk = partials.remove(0);
                    }

                    data[x][y][z] = partial_chunk.data[i % PARTIAL_CHUNK_UPDATE_SIZE];
                    i += 1;
                }
            }
        }

        Some(FullChunkUpdate::new(
            data,
            partial_chunk.x,
            partial_chunk.y,
            partial_chunk.z,
        ))
    }
}

#[test]
pub fn reconstructions() {
    let mut data = [[[0 as u32; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                data[x][y][z] = (x as u32 + y as u32 * 2 + z as u32) % 70;
            }
        }
    }

    let partial_chunks = FullChunkUpdate::new(data, 0, 0, 0).to_partial();

    //assert_eq!(format!("{:?}", data), format!("{:?}", partial_chunks));

    assert_eq!(
        data,
        FullChunkUpdate::from_partial(partial_chunks).unwrap().data
    );
}
