use bevy::prelude::Component;
use rc_shared::constants::GameObjectId;
use serde::{Deserialize, Serialize};

#[derive(Component, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct GameObject {
    pub id: GameObjectId
}
