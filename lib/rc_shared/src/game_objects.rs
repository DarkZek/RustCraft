use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum GameObjectData {
    Debug,
    ItemDrop(String),
    Player
}