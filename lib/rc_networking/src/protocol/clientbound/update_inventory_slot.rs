use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[repr(C)]
pub struct UpdateInventorySlot {
    pub id: String,
    pub amount: u32,
    pub slot: u32
}

impl UpdateInventorySlot {
    pub fn new(id: String, amount: u32, slot: u32) -> UpdateInventorySlot {
        UpdateInventorySlot {
            id,
            amount,
            slot,
        }
    }
}