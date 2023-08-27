mod look;
mod movement;

use crate::state::AppState;
use crate::systems::chunk::builder::{RerenderChunkFlag, RerenderChunkFlagContext};
use crate::systems::input::look::update_input_look;
use crate::systems::input::movement::update_input_movement;
use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use nalgebra::Vector3;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(InputSystem { captured: false })
            .add_systems(
                Update,
                (update_input_look, update_input_movement, grab_mouse)
                    .chain()
                    .run_if(in_state(AppState::InGame)),
            )
            .add_system(grab_mouse_on_play.in_schedule(OnEnter(AppState::InGame)));
    }
}

#[derive(Resource)]
pub struct InputSystem {
    captured: bool,
}

// This system grabs the mouse when the left mouse button is pressed
// and releases it when the escape key is pressed
fn grab_mouse(
    mut primary_query: Query<&mut Window, With<PrimaryWindow>>,
    mouse: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
    mut service: ResMut<InputSystem>,
) {
    let Ok(mut window) = primary_query.get_single_mut() else {
        return;
    };
    if mouse.just_pressed(MouseButton::Left) {
        window.cursor.visible = false;
        window.cursor.grab_mode = CursorGrabMode::Confined;
        service.captured = true;
    }
    if key.just_pressed(KeyCode::Escape) {
        window.cursor.visible = true;
        window.cursor.grab_mode = CursorGrabMode::None;
        service.captured = false;
    }
}

fn grab_mouse_on_play(
    mut primary_query: Query<&mut Window, With<PrimaryWindow>>,
    mut service: ResMut<InputSystem>,
) {
    let Ok(mut window) = primary_query.get_single_mut() else {
        return;
    };
    window.cursor.visible = false;
    window.cursor.grab_mode = CursorGrabMode::Confined;
    service.captured = true;
}
