use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
#[repr(C)]
pub struct DestroyBlock {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl DestroyBlock {
    pub fn new(x: i32, y: i32, z: i32) -> DestroyBlock {
        DestroyBlock {
            x,
            y,
            z
        }
    }
}