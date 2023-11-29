use crate::constants::GameObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
#[repr(C)]
pub struct DespawnGameObject {
    pub entity: GameObjectId,
}

impl DespawnGameObject {
    pub fn new(eid: GameObjectId) -> DespawnGameObject {
        DespawnGameObject { entity: eid }
    }
}
