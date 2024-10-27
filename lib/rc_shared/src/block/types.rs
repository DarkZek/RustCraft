use serde::{Deserialize, Serialize};
use crate::aabb::Aabb;
use crate::block::face::Face;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualBlock {
    pub translucent: bool,
    pub full: bool,
    pub draw_betweens: bool,
    pub faces: Vec<Face>,
    pub collision_boxes: Vec<Aabb>,
    pub bounding_boxes: Vec<Aabb>,
    pub emission: [u8; 4],
}

#[derive(Clone, Debug)]
pub struct LootTableEntry {
    pub chance: f32,
    pub item_identifier: String,
}
