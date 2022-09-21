use bevy_ecs::prelude::Component;
use naia_shared::Replicate;
use naia_shared::{EntityProperty, Property};

#[derive(Replicate, Component)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct PlayerJoin {
    pub username: Property<String>,
    pub entity: EntityProperty,
}

impl PlayerJoin {
    pub fn new(username: &str) -> Self {
        PlayerJoin::new_complete(username.to_string())
    }
}
