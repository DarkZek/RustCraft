use bevy::prelude::*;
use rc_shared::game_objects::GameObjectData;
use super::entity::GameObject;

pub struct GameObjectPlugin;

impl Plugin for GameObjectPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, item_spin);
    }
}

fn item_spin(mut query: Query<(&mut Transform, &GameObject)>, time: Res<Time>) {
    for (mut transform, game_object) in query.iter_mut() {
        if let GameObjectData::ItemDrop(_) = game_object.data {
            transform.rotate_local_y(time.delta_seconds() * 1.0);
        }
    }
}