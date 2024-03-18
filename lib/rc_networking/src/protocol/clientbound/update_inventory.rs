use rc_shared::item::types::ItemStack;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[repr(C)]
pub struct UpdateInventory {
    pub slots: [Option<ItemStack>; 10],
    pub hotbar_slot: Option<u8>,
}

impl UpdateInventory {
    pub fn new(
        slots: [Option<ItemStack>; 10],
        hotbar_slot: Option<u8>) -> UpdateInventory {
        UpdateInventory {
            slots,
            hotbar_slot
        }
    }
}