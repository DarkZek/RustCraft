use bevy_ecs::prelude::Component;
use naia_shared::Replicate;
use naia_shared::{EntityProperty, Property};

#[derive(Replicate, Component)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct UserAuthenticate {
    pub username: Property<String>,
    pub entity: EntityProperty,
}

impl UserAuthenticate {
    pub fn new(username: &str) -> Self {
        UserAuthenticate::new_complete(username.to_string())
    }
}
