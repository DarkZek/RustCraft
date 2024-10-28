mod native;
#[cfg(target_arch = "wasm32")]
mod wasm;

#[cfg(not(target_arch = "wasm32"))]
pub use native::{grab_mouse, grab_mouse_on_play, setup_listeners};
#[cfg(target_arch = "wasm32")]
pub use wasm::{grab_mouse, grab_mouse_on_play, setup_listeners};