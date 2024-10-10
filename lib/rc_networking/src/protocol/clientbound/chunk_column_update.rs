use nalgebra::Vector2;
use serde::{Serialize, Deserialize};
use rc_shared::chunk_column::ChunkColumnData;

#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
#[repr(C)]
pub struct ChunkColumnUpdate {
    pub position: Vector2<i32>,
    pub data: ChunkColumnData
}

impl ChunkColumnUpdate {
    pub fn new(position: Vector2<i32>, data: ChunkColumnData) -> ChunkColumnUpdate {
        ChunkColumnUpdate {
            position,
            data
        }
    }
}