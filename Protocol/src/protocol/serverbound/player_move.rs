use bevy_ecs::prelude::Component;
use naia_shared::Property;
use naia_shared::Replicate;

#[derive(Replicate, Component)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct PlayerMove {
    pub x: Property<f32>,
    pub y: Property<f32>,
    pub z: Property<f32>,
}

impl PlayerMove {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        PlayerMove::new_complete(x, y, z)
    }
}
