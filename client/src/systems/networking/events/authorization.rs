use bevy::ecs::event::Event;
use rc_networking::constants::UserId;

#[derive(Event)]
pub struct AuthorizationEvent {
    pub client: UserId,
}
