use serde::{Deserialize, Serialize};
use crate::constants::UserId;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum GameObjectData {
    Debug,
    ItemDrop(String),
    Player(UserId)
}