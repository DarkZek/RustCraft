use crate::constants::GameObjectId;

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
#[repr(C)]
pub struct SpawnGameObject {
    pub id: GameObjectId,
    pub loc: [f32; 3],
    pub rot: [f32; 4],
    pub object_type: u32,
}
