use serde::{Deserialize, Serialize};
use crate::{constants::UserId, item::types::ItemStack};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum GameObjectData {
    Debug,
    ItemDrop(ItemStack),
    Player(UserId)
}