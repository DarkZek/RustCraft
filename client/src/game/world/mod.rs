use crate::game::world::sun::{setup_sun, update_sun};
use crate::state::AppState;
use bevy::app::App;
use bevy::prelude::{in_state, IntoSystemConfigs, OnEnter, Plugin, Update};

pub mod sun;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup_sun)
            .add_systems(Update, update_sun.run_if(in_state(AppState::InGame)));
    }
}
