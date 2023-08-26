use crate::game::world::sun::{setup_sun, update_sun};
use crate::state::AppState;
use bevy::app::{App, IntoSystemAppConfig};
use bevy::prelude::{IntoSystemConfig, OnEnter, OnUpdate, Plugin, SystemSet};

pub mod sun;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_sun.in_schedule(OnEnter(AppState::InGame)))
            .add_system(update_sun.in_set(OnUpdate(AppState::InGame)));
    }
}
