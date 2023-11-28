use bevy::asset::Asset;
use bevy::reflect::TypePath;
use bevy::reflect::TypeUuid;
use nalgebra::Vector3;
use serde::{Deserialize, Serialize};

#[derive(Asset, Debug, Clone, Deserialize, Serialize, TypeUuid, TypePath)]
#[uuid = "7b14806a-672b-423b-8d16-4f18abefa463"]
pub struct ItemStatesFile {
    pub states: Vec<DeserialisedItem>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeserialisedItem {
    pub identifier: String,
    pub name: String,
    pub icon: String,
    // The block that will be created if placed
    pub block_state: String,
}
