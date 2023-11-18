use bevy::asset::Asset;
use bevy::reflect::TypePath;
use bevy::reflect::TypeUuid;
use nalgebra::Vector3;
use serde::{Deserialize, Serialize};

#[derive(Asset, Debug, Clone, Deserialize, Serialize, TypeUuid, TypePath)]
#[uuid = "7b14806a-672b-423b-8d16-4f18abefa463"]
pub struct BlockStatesFile {
    pub states: Vec<DeserialisedBlock>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeserialisedBlock {
    pub identifier: String,
    pub translucent: bool,
    pub full: bool,
    pub draw_betweens: bool,
    pub faces: Vec<DeserialisedFace>,
    pub colliders: Vec<DeserialisedAabb>,
    pub emission: [u8; 4],
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeserialisedFace {
    pub top_left: Vector3<f32>,
    pub top_right: Vector3<f32>,
    pub bottom_left: Vector3<f32>,
    pub texture: String,
    // If face is at the edge of a face, and its direction is against a block where it could be fulled, then cull the face
    pub edge: bool,
    pub direction: u8,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeserialisedAabb {
    pub bottom_left: Vector3<f32>,
    pub size: Vector3<f32>,
    pub collidable: bool,
}
