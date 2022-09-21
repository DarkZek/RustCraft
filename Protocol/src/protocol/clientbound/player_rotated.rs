use bevy_ecs::prelude::Component;
use naia_shared::Replicate;
use naia_shared::{EntityProperty, Property};

#[derive(Replicate, Component)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct PlayerRotated {
    pub player: EntityProperty,
    pub x: Property<f32>,
    pub y: Property<f32>,
    pub z: Property<f32>,
    pub w: Property<f32>,
}

impl PlayerRotated {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        PlayerRotated::new_complete(x, y, z, w)
    }
}
