use bevy::prelude::Component;
use rc_shared::game_objects::GameObjectData;
use serde::{Deserialize, Serialize};

#[derive(Component, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct GameObject {
    pub data: GameObjectData,
}
