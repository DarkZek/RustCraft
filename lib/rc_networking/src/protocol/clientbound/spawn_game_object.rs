use crate::constants::GameObjectId;
use rc_shared::game_objects::GameObjectData;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[repr(C)]
pub struct SpawnGameObject {
    pub id: GameObjectId,
    pub loc: [f32; 3],
    pub rot: [f32; 4],
    pub data: GameObjectData
}
