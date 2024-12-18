use bevy::prelude::*;
use rc_shared::game_objects::GameObjectData;
use crate::game::game_object::spawn::messages_update;
use super::entity::GameObject;

pub mod spawn;
pub mod mesh;
pub(crate) mod player;

pub struct GameObjectPlugin;

impl Plugin for GameObjectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, item_spin);
        app.add_systems(Update, messages_update);
    }
}

fn item_spin(mut query: Query<(&mut Transform, &GameObject)>, time: Res<Time>) {
    for (mut transform, game_object) in query.iter_mut() {
        if let GameObjectData::ItemDrop(_) = game_object.data {
            transform.rotate_local_y(time.delta_seconds() * 1.0);
        }
    }
}

pub trait Rotatable {
    fn rotate(&self, yaw: f32, pitch: f32, transforms: &mut Query<&mut Transform>);
}