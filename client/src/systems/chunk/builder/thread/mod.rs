mod platform;
pub mod executor;

#[cfg(target_arch = "wasm32")]
pub use platform::wasm::ChunkBuilderScheduler;

#[cfg(not(target_arch = "wasm32"))]
pub use platform::threaded::ChunkBuilderScheduler;

pub use platform::ChunkBuilderSchedulerTrait;