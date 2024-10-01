use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};

#[derive(Resource, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PlayerGameMode {
    Play,
    Minigame,
    Sandbox
}