use crate::constants::UserId;

use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
#[repr(C)]
pub struct PlayerMove {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl PlayerMove {
    pub fn new(x: f32, y: f32, z: f32) -> PlayerMove {
        PlayerMove {
            x,
            y,
            z
        }
    }
}