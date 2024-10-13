#![allow(dead_code)]

use std::cell::OnceCell;
use std::sync::OnceLock;
use bevy::prelude::{info, Resource};
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;

pub mod start;
pub mod game;
pub mod state;
pub mod systems;
pub mod utils;
pub mod authentication;

// TODO: Performance - Make event based systems only run on event trigger https://docs.rs/bevy/latest/bevy/ecs/prelude/fn.on_event.html

// Native applications, start game right away
#[cfg(not(target_arch = "wasm32"))]
pub fn main() {
    start::start()
}

// Wasm applications, do nothing by default so we can use this wasm file to spin up workers as well
#[cfg(target_arch = "wasm32")]
pub fn main() {
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn start_game(worker: JsValue, startup_callback: js_sys::Function) {
    use web_sys::Worker;
    use crate::start::{WASM_CONTEXT, WasmContext};

    let Ok(chunk_worker) = Worker::try_from(worker) else {
       panic!("Chunk worker not passed in")
    };

    // Store worker
    WASM_CONTEXT.set(WasmContext {
        chunk_worker,
        startup_callback
    });

    start::start();
}