use bevy::prelude::Component;

#[derive(Component, Debug, Clone)]
pub struct Player {
    pub pitch: f32,
    pub yaw: f32,
    pub is_sprinting: bool
}

impl Player {
    pub fn new() -> Player {
        Player {
            pitch: 0.0,
            yaw: 0.0,
            is_sprinting: false,
        }
    }
}
