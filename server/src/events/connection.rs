use bevy::ecs::event::Event;
use rc_networking::constants::UserId;

#[derive(Event)]
pub struct ConnectionEvent {
    pub user: UserId,
}

impl ConnectionEvent {
    pub fn new(user: UserId) -> ConnectionEvent {
        ConnectionEvent { user }
    }
}
