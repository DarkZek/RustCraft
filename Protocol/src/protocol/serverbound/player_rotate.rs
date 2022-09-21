use bevy_ecs::prelude::Component;
use naia_shared::Property;
use naia_shared::Replicate;

#[derive(Replicate, Component)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct PlayerRotate {
    pub x: Property<f32>,
    pub y: Property<f32>,
    pub z: Property<f32>,
    pub w: Property<f32>,
}

impl PlayerRotate {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        PlayerRotate::new_complete(x, y, z, w)
    }
}
