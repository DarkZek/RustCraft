use bevy::prelude::Component;
use serde::{Deserialize, Serialize};

#[derive(Component, Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
pub struct GameObject {
    pub object_type: u32,
}
