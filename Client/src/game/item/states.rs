use crate::game::item::ItemType;

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
            ],
        }
    }
}
