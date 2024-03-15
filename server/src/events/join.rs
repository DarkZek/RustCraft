use bevy::ecs::event::Event;
use rc_shared::constants::UserId;

/// After a player is authorized and spawned in the world
#[derive(Event)]
pub struct PlayerSpawnEvent {
    pub id: UserId,
}
