use crate::constants::{EntityId, UserId};

use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
pub struct SpawnEntity {
    pub id: EntityId,
    pub loc: [f32; 3],
    pub rot: [f32; 4]
}