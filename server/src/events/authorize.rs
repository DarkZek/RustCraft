use bevy::prelude::Event;
use rc_shared::constants::UserId;

#[derive(Event)]
pub struct AuthorizationEvent {
    pub user_id: UserId,
}
