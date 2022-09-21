use bevy_ecs::prelude::Component;
use naia_shared::Replicate;
use naia_shared::{EntityProperty, Property};

#[derive(Replicate, Component)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct PlayerMoved {
    pub player: EntityProperty,
    pub x: Property<f32>,
    pub y: Property<f32>,
    pub z: Property<f32>,
}

impl PlayerMoved {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        PlayerMoved::new_complete(x, y, z)
    }
}
