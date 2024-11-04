use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PresentMode, PrimaryWindow};
use crate::systems::input::InputSystem;
use crate::systems::ui::console::ConsoleData;

pub fn setup_listeners() {

}

// This system grabs the mouse when the left mouse button is pressed
// and releases it when the escape key is pressed
pub fn grab_mouse(
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

pub fn grab_mouse_on_play(
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
