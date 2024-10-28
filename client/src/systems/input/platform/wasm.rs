use std::sync::atomic::{AtomicBool, Ordering};
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PresentMode, PrimaryWindow};
use bevy::winit::WinitWindows;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::prelude::wasm_bindgen;
use crate::systems::input::InputSystem;
use crate::systems::ui::console::ConsoleData;

static IS_CAPTURED: AtomicBool = AtomicBool::new(false);

pub fn setup_listeners(

) {
    let document = web_sys::window().unwrap().document().unwrap();

    let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::Event| {
        let document = web_sys::window().unwrap().document().unwrap();
        IS_CAPTURED.store(document.pointer_lock_element().is_some(), Ordering::Relaxed);
    });
    document.add_event_listener_with_callback("pointerlockchange", closure.as_ref().unchecked_ref()).unwrap();

    // Keep it around forever
    Box::new(closure).forget();
}

// This system grabs the mouse when the left mouse button is pressed
// and releases it when the escape key is pressed
pub fn grab_mouse(
    mut primary_query: Query<&mut Window, With<PrimaryWindow>>,
    mouse: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
    mut service: ResMut<InputSystem>,
    console_data: Res<ConsoleData>,
    mut prev_state: Local<bool>
) {
    let Ok(mut window) = primary_query.get_single_mut() else {
        return;
    };
    let document = web_sys::window().unwrap().document().unwrap();
    let game = document.get_element_by_id("game").unwrap();

    let is_captured = IS_CAPTURED.load(Ordering::Relaxed);

    // vsync
    window.present_mode = PresentMode::AutoVsync;

    // Request to capture & uncapture mouse
    if mouse.just_pressed(MouseButton::Left) {
        debug!("Capturing");
        game.request_pointer_lock();
        // window.cursor.grab_mode = CursorGrabMode::Confined;
        window.cursor.visible = false;
    }
    if key.just_pressed(KeyCode::Escape) && !console_data.capturing {
        debug!("Uncapture");
        document.exit_pointer_lock();
        window.cursor.visible = true;
        service.captured = false;
    }

    // Only once we've got a definite lock on the inputs do we accept input
    if is_captured && !*prev_state {
        service.captured = true;
    }

    // If the browser uncaptured us, uncapture
    if !is_captured && *prev_state {
        debug!("Force uncapture");
        document.exit_pointer_lock();
        window.cursor.visible = true;
        service.captured = false;
    }

    *prev_state = is_captured;
}

// Must be user initiated in wasm
pub fn grab_mouse_on_play(
    mut primary_query: Query<&mut Window, With<PrimaryWindow>>
) {

}
