use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
#[repr(C)]
pub struct ChangeHotbarSlot {
    pub slot: u8
}

impl ChangeHotbarSlot {
    pub fn new(slot: u8) -> ChangeHotbarSlot {
        ChangeHotbarSlot {
            slot
        }
    }
}