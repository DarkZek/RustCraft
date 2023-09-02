use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
#[repr(C)]
pub struct AcknowledgeChunk {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl AcknowledgeChunk {
    pub fn new(x: i32, y: i32, z: i32) -> AcknowledgeChunk {
        AcknowledgeChunk { x, y, z }
    }
}
