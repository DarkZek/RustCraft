use nalgebra::Vector3;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct BlockStatesFile {
    pub states: Vec<DeserialisedBlock>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct DeserialisedBlock {
    pub identifier: String,
    pub translucent: bool,
    pub full: bool,
    pub draw_betweens: bool,
    pub faces: Vec<DeserialisedFace>,
    pub colliders: Vec<DeserialisedAabb>,
    pub emission: [u8; 4],
    pub loot_table: Vec<DeserialisedLootTableEntry>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct DeserialisedFace {
    pub top_left: Vector3<f32>,
    pub top_right: Vector3<f32>,
    pub bottom_left: Vector3<f32>,
    pub texture: String,
    // If face is at the edge of a face, and its direction is against a block where it could be fulled, then cull the face
    pub edge: bool,
    pub direction: u8,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct DeserialisedAabb {
    pub bottom_left: Vector3<f32>,
    pub size: Vector3<f32>,
    pub collidable: bool,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct DeserialisedLootTableEntry {
    pub chance: f32,
    pub item: String,
}
