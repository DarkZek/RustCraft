use bevy::app::{App, Plugin};
use bevy::asset::Assets;
use bevy::pbr::StandardMaterial;
use bevy::prelude::{Input, KeyCode, MouseButton, Mut, Res, ResMut};
use bevy::window::Windows;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(grab_mouse);
    }
}

// This system grabs the mouse when the left mouse button is pressed
// and releases it when the escape key is pressed
fn grab_mouse(
    mut windows: ResMut<Windows>,
    mouse: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
) {
    let window = windows.get_primary_mut().unwrap();
    // if mouse.just_pressed(MouseButton::Left) {
    //     window.set_cursor_visibility(false);
    //     window.set_cursor_lock_mode(true);
    //     println!("Capturing cursor!");
    // }
    // if key.just_pressed(KeyCode::Escape) {
    //     window.set_cursor_lock_mode(false);
    //     window.set_cursor_visibility(true);
    //     println!("uncapturing cursor!");
    // }
}