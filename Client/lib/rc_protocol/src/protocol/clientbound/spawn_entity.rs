use crate::constants::{EntityId};

use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
#[repr(C)]
pub struct SpawnEntity {
    pub id: EntityId,
    pub loc: [f32; 3],
    pub rot: [f32; 4]
}