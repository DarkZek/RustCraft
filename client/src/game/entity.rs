use bevy::prelude::Component;
use rc_shared::game_objects::GameObjectData;

#[derive(Component)]
pub struct GameObject {
    pub data: GameObjectData
}
