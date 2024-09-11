use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
#[repr(C)]
pub struct PlaceBlock {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl PlaceBlock {
    pub fn new(x: i32, y: i32, z: i32) -> PlaceBlock {
        PlaceBlock {
            x,
            y,
            z
        }
    }
}