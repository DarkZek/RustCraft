mod light_heightmap;
pub mod freecam;
mod game_object_hitboxes;

use bevy::app::App;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use crate::systems::debugging::freecam::{freecam_activation, freecam_movement};
use crate::systems::debugging::game_object_hitboxes::draw_game_object_hitboxes;
use crate::systems::debugging::light_heightmap::draw_skylight;

pub struct DebuggingPlugin;

impl Plugin for DebuggingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (draw_skylight, draw_game_object_hitboxes))
            .add_systems(Update, (freecam_activation, freecam_movement))
            .add_systems(Startup, setup_gizmos);
    }
}

fn setup_gizmos(
    mut config_store: ResMut<GizmoConfigStore>,
) {
    for (_, config, _) in config_store.iter_mut() {
        config.depth_bias = -1.0;
    }
}

fn control_gizmos(
    mut evr_kbd: EventReader<KeyboardInput>,
) {

}