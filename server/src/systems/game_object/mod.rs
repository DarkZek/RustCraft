use crate::systems::game_object::spawn::{
    spawn_entities, SpawnGameObjectEvent, SpawnGameObjectRequest,
};
use crate::App;
use bevy::app::Plugin;
use bevy::prelude::Update;

pub mod spawn;

pub struct GameObjectPlugin;

impl Plugin for GameObjectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_entities)
            .add_event::<SpawnGameObjectRequest>()
            .add_event::<SpawnGameObjectEvent>();
    }
}

pub struct GameObjectSystem;
