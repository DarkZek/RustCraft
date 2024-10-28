mod look;
mod movement;
mod sprint;
mod platform;

use crate::state::AppState;

use crate::systems::input::look::update_input_look;
use crate::systems::input::movement::update_input_movement;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PresentMode, PrimaryWindow};
use crate::systems::input::platform::setup_listeners;
use crate::systems::input::sprint::detect_sprinting;
use crate::systems::ui::console::ConsoleData;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(InputSystem { captured: false })
            .add_systems(Startup, setup_listeners)
            .add_systems(
                Update,
                (update_input_look, update_input_movement, platform::grab_mouse, detect_sprinting)
                    .chain()
                    .run_if(in_state(AppState::InGame)),
            )
            .add_systems(OnEnter(AppState::InGame), platform::grab_mouse_on_play);
    }
}

#[derive(Resource)]
pub struct InputSystem {
    captured: bool,
}