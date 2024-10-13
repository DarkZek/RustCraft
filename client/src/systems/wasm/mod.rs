use bevy::app::{App, PostUpdate};
use bevy::prelude::{Local, Plugin};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsValue;
use crate::start::WASM_CONTEXT;

pub struct WasmPlugin;

impl Plugin for WasmPlugin {
    #[cfg(not(target_arch = "wasm32"))]
    fn build(&self, app: &mut App) {

    }
    #[cfg(target_arch = "wasm32")]
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, run_started_callback);
    }
}

#[derive(Default)]
struct StartedContext {
    started: bool
}

fn run_started_callback(
    mut context: Local<StartedContext>
) {

    if context.started {
        return
    }

    context.started = true;

    let context = JsValue::null();
    let array = js_sys::Array::new();

    WASM_CONTEXT.get().unwrap().startup_callback.apply(&context, &array);
}