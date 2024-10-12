mod light_heightmap;
pub mod freecam;
mod game_object_hitboxes;
mod chunk;

use bevy::app::App;
use bevy::input::ButtonState;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use crate::systems::debugging::chunk::draw_chunk_boundary;
use crate::systems::debugging::freecam::{freecam_activation, freecam_movement};
use crate::systems::debugging::game_object_hitboxes::draw_game_object_hitboxes;
use crate::systems::debugging::light_heightmap::draw_skylight;

pub struct DebuggingPlugin;

impl Plugin for DebuggingPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(DebuggingInfo {
                gizmos_enabled: false,
                freecam: false,
            })
            .add_systems(Update, (draw_skylight, draw_game_object_hitboxes, draw_chunk_boundary).run_if(run_gizmos))
            .add_systems(Update, freecam_movement)
            .add_systems(Startup, setup_gizmos)
            .add_systems(Update, (freecam_activation, control_gizmos));
    }
}

#[derive(Resource)]
pub struct DebuggingInfo {
    pub gizmos_enabled: bool,
    pub freecam: bool
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
    mut debugging_info: ResMut<DebuggingInfo>
) {
    for event in evr_kbd.read() {
        if event.state != ButtonState::Pressed {
            continue;
        }

        if event.key_code == KeyCode::F6 {
            // Toggle gizmos
            debugging_info.gizmos_enabled = !debugging_info.gizmos_enabled;
        }
    }
}

fn run_gizmos(debugging_info: Res<DebuggingInfo>) -> bool {
    debugging_info.gizmos_enabled
}