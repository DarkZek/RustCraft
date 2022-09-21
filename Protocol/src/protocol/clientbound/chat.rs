use bevy_ecs::prelude::Component;
use naia_shared::Property;
use naia_shared::Replicate;

#[derive(Replicate, Component)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct ChatSent {
    pub message: Property<String>,
}

impl ChatSent {
    pub fn new(message: &str) -> Self {
        ChatSent::new_complete(message.to_string())
    }
}
