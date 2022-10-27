use crate::constants::{RawChunkData, CHUNK_SIZE};

/// How many blocks are sent per partial update packet
pub const CHUNK_UPDATE_BLOCKS_PER_PACKET: usize = 256;

/// How many partial chunks it takes to make up a full chunk
pub const CHUNK_UPDATE_PARTIAL_CHUNKS: usize =
    (CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE) / CHUNK_UPDATE_BLOCKS_PER_PACKET;

use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
pub struct PartialChunkUpdate {
    pub data: RawChunkData,
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl PartialChunkUpdate {
    pub fn new(data: RawChunkData, x: i32, y: i32, z: i32) -> Self {
        PartialChunkUpdate {
            data,
            x,
            y,
            z,
        }
    }
}
