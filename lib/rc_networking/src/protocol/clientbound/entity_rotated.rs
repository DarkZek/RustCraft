use crate::constants::GameObjectId;

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
#[repr(C)]
pub struct EntityRotated {
    pub entity: GameObjectId,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
