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
                    identifier: "mcv3::GrassBlockItem".to_string(),
                    name: "Grass Block".to_string(),
                    icon: "grass".to_string(),
                    block_state: Some(2),
                },
                ItemType {
                    identifier: "mcv3::DirtBlockItem".to_string(),
                    name: "Dirt Block".to_string(),
                    icon: "dirt".to_string(),
                    block_state: Some(1),
                },
                ItemType {
                    identifier: "mcv3::LongGrassItem".to_string(),
                    name: "Long Grass".to_string(),
                    icon: "long_grass".to_string(),
                    block_state: Some(3),
                },
                ItemType {
                    identifier: "mcv3::LeavesItem".to_string(),
                    name: "Leaves".to_string(),
                    icon: "tree_leaves".to_string(),
                    block_state: Some(5),
                },
                ItemType {
                    identifier: "mcv3::WoodLogItem".to_string(),
                    name: "Wood Log".to_string(),
                    icon: "wood_log".to_string(),
                    block_state: Some(4),
                },
                ItemType {
                    identifier: "mcv3::LampItem".to_string(),
                    name: "Lamp".to_string(),
                    icon: "lamp".to_string(),
                    block_state: Some(7),
                },
                ItemType {
                    identifier: "mcv3::ItemSpawnerItem".to_string(),
                    name: "Item Spawner".to_string(),
                    icon: "itemspawner".to_string(),
                    block_state: Some(9),
                },
                ItemType {
                    identifier: "mcv3::PipeItem".to_string(),
                    name: "Pipe".to_string(),
                    icon: "tree_leaves".to_string(),
                    block_state: Some(11),
                },
            ],
        }
    }
}
