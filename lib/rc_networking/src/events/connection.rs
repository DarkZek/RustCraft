use bevy::ecs::event::Event;
use rc_shared::constants::UserId;

#[derive(Event)]
pub struct NetworkConnectionEvent {
    pub client: UserId,
}
