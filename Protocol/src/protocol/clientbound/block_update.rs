use bevy_ecs::prelude::Component;
use naia_shared::{Property, Replicate};

#[derive(Replicate, Component)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct BlockUpdate {
    pub id: Property<u32>,
    pub x: Property<i32>,
    pub y: Property<i32>,
    pub z: Property<i32>,
}

impl BlockUpdate {
    pub fn new(id: u32, x: i32, y: i32, z: i32) -> Self {
        BlockUpdate::new_complete(id, x, y, z)
    }
}
