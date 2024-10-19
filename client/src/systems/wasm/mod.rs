use bevy::app::{App, PostUpdate};
use bevy::prelude::{EventWriter, info, Local, NextState, OnEnter, Plugin, ResMut};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsValue;
use crate::start::WASM_CONTEXT;
use crate::state::AppState;
use crate::systems::connection::connect::ConnectToServerIntent;

pub struct WasmPlugin;

impl Plugin for WasmPlugin {
    #[cfg(not(target_arch = "wasm32"))]
    fn build(&self, app: &mut App) {

    }
    #[cfg(target_arch = "wasm32")]
    fn build(&self, app: &mut App) {
        app
            .add_systems(PostUpdate, run_started_callback)
            .add_systems(OnEnter(AppState::MainMenu), temp_skip_loading);
    }
}

#[derive(Default)]
struct StartedContext {
    started: bool
}

fn temp_skip_loading(
    mut connection_intent: EventWriter<ConnectToServerIntent>
) {
    info!("Skipped main menu");
    connection_intent.send(ConnectToServerIntent {
        address: env!("SERVER_URL").parse().unwrap()
    });
}

fn run_started_callback(
    mut context: Local<StartedContext>
) {

    if context.started {
        return
    }

    info!("Successfully started");

    context.started = true;

    let context = JsValue::null();
    let array = js_sys::Array::new();

    WASM_CONTEXT.get().unwrap().startup_callback.apply(&context, &array).unwrap();
}