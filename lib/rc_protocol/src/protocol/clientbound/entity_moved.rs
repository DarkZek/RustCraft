use crate::constants::{EntityId};

use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
#[repr(C)]
pub struct EntityMoved {
    pub entity: EntityId,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}