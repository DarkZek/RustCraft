use crate::systems::game_object::propagation::propagate_game_objects_to_new_clients;
use crate::systems::game_object::spawn::{
    spawn_entities, SpawnGameObjectEvent, SpawnGameObjectRequest,
};
use crate::App;
use bevy::app::Plugin;
use bevy::prelude::Update;
use crate::systems::game_object::despawn::despawn_game_objects;
use self::collect_item::collect_items;

mod propagation;
pub mod spawn;
mod collect_item;
mod despawn;

pub struct GameObjectPlugin;

impl Plugin for GameObjectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_entities)
            .add_systems(Update, (
                propagate_game_objects_to_new_clients,
                collect_items,
                despawn_game_objects
            ))
            .add_event::<SpawnGameObjectRequest>()
            .add_event::<SpawnGameObjectEvent>();
    }
}

pub struct GameObjectSystem;
