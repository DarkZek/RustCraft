use bevy::ecs::event::Event;
use rc_networking::constants::UserId;

#[derive(Event)]
pub struct AuthorizationEvent {
    pub client: UserId,
}

impl AuthorizationEvent {
    pub fn new(client: UserId) -> AuthorizationEvent {
        AuthorizationEvent { client }
    }
}
