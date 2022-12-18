use crate::game::world::sun::{setup_sun, update_sun};
use crate::state::AppState;
use bevy::app::App;
use bevy::prelude::{Plugin, SystemSet};

pub mod sun;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::InGame).with_system(setup_sun))
            .add_system_set(SystemSet::on_update(AppState::InGame).with_system(update_sun));
    }
}
