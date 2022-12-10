use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
#[repr(C)]
pub struct RequestChunk {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl RequestChunk {
    pub fn new(x: i32, y: i32, z: i32) -> RequestChunk {
        RequestChunk { x, y, z }
    }
}
