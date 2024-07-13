#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(not(target_arch = "wasm32"))]
pub use native::QuinnClientPlugin;
#[cfg(not(target_arch = "wasm32"))]
pub use native::NetworkingClient;

#[cfg(target_arch = "wasm32")]
mod wasm;
#[cfg(target_arch = "wasm32")]
pub use wasm::QuinnClientPlugin;
#[cfg(target_arch = "wasm32")]
pub use wasm::NetworkingClient;
