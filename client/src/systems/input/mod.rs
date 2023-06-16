mod look;
mod movement;

use crate::state::AppState;
use crate::systems::chunk::builder::{RerenderChunkFlag, RerenderChunkFlagContext};
use crate::systems::input::look::update_input_look;
use crate::systems::input::movement::update_input_movement;
use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, Windows};
use nalgebra::Vector3;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(InputSystem { captured: false })
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(update_input_look)
                    .with_system(update_input_movement)
                    .with_system(grab_mouse),
            )
            .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(grab_mouse_on_play));
    }
}

#[derive(Resource)]
pub struct InputSystem {
    captured: bool,
}

// This system grabs the mouse when the left mouse button is pressed
// and releases it when the escape key is pressed
fn grab_mouse(
    mut windows: ResMut<Windows>,
    mouse: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
    mut service: ResMut<InputSystem>,
) {
    let window = windows.get_primary_mut().unwrap();
    if mouse.just_pressed(MouseButton::Left) {
        window.set_cursor_visibility(false);
        window.set_cursor_grab_mode(CursorGrabMode::Confined);
        service.captured = true;
    }
    if key.just_pressed(KeyCode::Escape) {
        window.set_cursor_visibility(true);
        window.set_cursor_grab_mode(CursorGrabMode::None);
        service.captured = false;
    }
}

fn grab_mouse_on_play(mut windows: ResMut<Windows>, mut service: ResMut<InputSystem>) {
    let window = windows.get_primary_mut().unwrap();
    window.set_cursor_visibility(false);
    window.set_cursor_grab_mode(CursorGrabMode::Confined);
    service.captured = true;
}
