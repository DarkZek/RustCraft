
#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(target_arch = "wasm32")]
mod wasm;

#[cfg(not(target_arch = "wasm32"))]
pub use native::BiStream;
#[cfg(target_arch = "wasm32")]
pub use wasm::BiStream;

pub enum StreamError {
    Error,
}