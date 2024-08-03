use bevy::ecs::entity::Entity;

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