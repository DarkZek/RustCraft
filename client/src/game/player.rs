use bevy::prelude::Component;

#[derive(Component, Debug, Clone)]
pub struct Player {
    pub is_sprinting: bool
}

impl Player {
    pub fn new() -> Player {
        Player {
            is_sprinting: false,
        }
    }
}
