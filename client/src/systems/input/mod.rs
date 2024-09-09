mod look;
mod movement;
mod sprint;

use crate::state::AppState;

use crate::systems::input::look::update_input_look;
use crate::systems::input::movement::update_input_movement;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PresentMode, PrimaryWindow};
use crate::systems::input::sprint::detect_sprinting;
use crate::systems::ui::console::ConsoleData;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(InputSystem { captured: false })
            .add_systems(
                Update,
                (update_input_look, update_input_movement, grab_mouse, detect_sprinting)
                    .chain()
                    .run_if(in_state(AppState::InGame)),
            )
            .add_systems(OnEnter(AppState::InGame), grab_mouse_on_play);
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
    mouse: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
    mut service: ResMut<InputSystem>,
    console_data: Res<ConsoleData>
) {
    let Ok(mut window) = primary_query.get_single_mut() else {
        return;
    };

    // vsync
    window.present_mode = PresentMode::AutoVsync;

    if mouse.just_pressed(MouseButton::Left) {
        window.cursor.visible = false;
        window.cursor.grab_mode = CursorGrabMode::Confined;
        service.captured = true;
    }
    if key.just_pressed(KeyCode::Escape) && !console_data.capturing {
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
