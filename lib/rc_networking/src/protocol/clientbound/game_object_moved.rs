use rc_shared::constants::GameObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
#[repr(C)]
pub struct GameObjectMoved {
    pub entity: GameObjectId,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
