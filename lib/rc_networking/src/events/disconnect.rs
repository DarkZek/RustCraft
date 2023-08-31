use crate::constants::UserId;
use bevy::ecs::event::Event;

#[derive(Event)]
pub struct NetworkDisconnectionEvent {
    pub client: UserId,
}
