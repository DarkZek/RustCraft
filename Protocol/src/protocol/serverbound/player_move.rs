use crate::constants::UserId;

use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct PlayerMove {
    pub client: UserId,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl PlayerMove {
    pub fn new(client: UserId, x: f32, y: f32, z: f32) -> PlayerMove {
        PlayerMove {
            client,
            x,
            y,
            z
        }
    }
}