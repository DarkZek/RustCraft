use bevy_ecs::prelude::Component;
use naia_shared::Property;
use naia_shared::Replicate;

#[derive(Replicate, Component)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct PlayerLeave {
    pub id: Property<u64>,
}

impl PlayerLeave {
    pub fn new(id: u64) -> Self {
        PlayerLeave::new_complete(id)
    }
}
