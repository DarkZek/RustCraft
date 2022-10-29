
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
#[repr(C)]
pub struct PlayerRotate {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}