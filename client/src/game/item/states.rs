use crate::game::item::ItemType;
use bevy::prelude::Resource;

#[derive(Resource)]
pub struct ItemStates {
    pub states: Vec<ItemType>,
}

impl ItemStates {
    pub fn new() -> ItemStates {
        ItemStates {
            states: vec![
                ItemType {
                    name: "Grass Block".to_string(),
                    icon: "grass".to_string(),
                    block_state: Some(2),
                },
                ItemType {
                    name: "Dirt Block".to_string(),
                    icon: "dirt".to_string(),
                    block_state: Some(1),
                },
                ItemType {
                    name: "Long Grass".to_string(),
                    icon: "long_grass".to_string(),
                    block_state: Some(3),
                },
            ],
        }
    }
}
