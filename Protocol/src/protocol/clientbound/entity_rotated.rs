use crate::constants::{EntityId, UserId};

use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
pub struct EntityRotated {
    pub entity: EntityId,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}