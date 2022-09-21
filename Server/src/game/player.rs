use bevy_ecs::entity::Entity;
use nalgebra::Vector3;

pub struct Player {
    pub entity: Entity,
    pub name: String
}

impl Player {
    pub fn new(entity: Entity, name: String) -> Self {
        Player {
            entity,
            name
        }
    }
}