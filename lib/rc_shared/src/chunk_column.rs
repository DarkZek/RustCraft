use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::chunk::{ChunkPosition, RawChunkData};
use crate::CHUNK_SIZE;

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChunkColumnData {
    pub skylight_level: [[Option<i32>; CHUNK_SIZE]; CHUNK_SIZE],
    pub dirty: bool
}

impl Default for ChunkColumnData {
    fn default() -> Self {
        ChunkColumnData {
            skylight_level: [[None; CHUNK_SIZE]; CHUNK_SIZE],
            dirty: false,
        }
    }
}