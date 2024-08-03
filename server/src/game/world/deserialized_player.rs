use nalgebra::{Quaternion, Vector3};
use serde::{Deserialize, Serialize};
use crate::game::inventory::Inventory;

#[derive(Serialize, Deserialize)]
pub struct DeserializedPlayerData {
    pub position: Vector3<f32>,
    pub rotation: Quaternion<f32>,
    pub inventory: Inventory
}