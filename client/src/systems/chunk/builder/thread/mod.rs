mod platform;
pub mod executor;

#[cfg(target_arch = "wasm32")]
pub use platform::unthreaded::ChunkBuilderScheduler;

#[cfg(not(target_arch = "wasm32"))]
pub use platform::threaded::ChunkBuilderScheduler;