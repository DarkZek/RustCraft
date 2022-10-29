use crate::constants::{EntityId, UserId};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
#[repr(C)]
pub struct DespawnEntity {
    pub entity: EntityId,
}

impl DespawnEntity {
    pub fn new(eid: EntityId) -> DespawnEntity {
        DespawnEntity {
            entity: eid
        }
    }
}