use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct BlockUpdate {
    pub id: u32,
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl BlockUpdate {
    pub fn new(id: u32, x: i32, y: i32, z: i32) -> BlockUpdate {
        BlockUpdate {
            id,
            x,
            y,
            z
        }
    }
}