use bevy::prelude::Component;
use serde::{Deserialize, Serialize};
use crate::{constants::UserId, item::types::ItemStack};

// I only really want to store the type on the entity so that I can lookup the components based on type, but
// I still need to store the data somewhere so that it can be networked so thats why we have two

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum GameObjectData {
    Debug,
    ItemDrop(ItemDropGameObjectData),
    Player(PlayerGameObjectData)
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Component)]
pub enum GameObjectType {
    Debug,
    ItemDrop,
    Player
}

impl From<&GameObjectData> for GameObjectType {
    fn from(value: &GameObjectData) -> Self {
        match value {
            GameObjectData::Debug => GameObjectType::Debug,
            GameObjectData::ItemDrop(_) => GameObjectType::ItemDrop,
            GameObjectData::Player(_) => GameObjectType::Player
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Copy, Component)]
pub struct PlayerGameObjectData {
    pub user_id: UserId
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Component)]
pub struct ItemDropGameObjectData {
    pub item_stack: ItemStack
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Copy, Component)]
pub struct DebugGameObjectData;

pub fn get_game_object_data(data: &GameObjectData) -> PlayerGameObjectData {
    if let GameObjectData::Player(data) = data {
        data.clone()
    } else {
        panic!("Didnt handle case")
    }
}