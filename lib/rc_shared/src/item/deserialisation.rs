use bevy::asset::Asset;
use bevy::reflect::TypePath;
use serde::{Deserialize, Serialize};

#[derive(Asset, Debug, Clone, Deserialize, Serialize, TypePath)]
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
