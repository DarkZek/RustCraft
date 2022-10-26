use crate::constants::UserId;

use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct PlayerMoved {
    pub player: UserId,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}