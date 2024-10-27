use serde::{Deserialize, Serialize};
use crate::block::BlockDefinitionIndex;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct ItemStack {
    pub item: ItemType,
    pub amount: u32,
}

impl ItemStack {
    pub fn new(item: ItemType, amount: u32) -> ItemStack {
        ItemStack { item, amount }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct ItemType {
    pub identifier: String,
    pub name: String,
    pub icon: String,

    // The block that will be created if placed
    pub block_definition_index: Option<BlockDefinitionIndex>,
}
