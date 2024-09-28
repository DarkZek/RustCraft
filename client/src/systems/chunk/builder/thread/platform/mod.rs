use crate::systems::chunk::builder::thread::executor::{ChunkBuilderExecutor, ChunkBuilderJob, ChunkBuilderUpdate};

pub mod unthreaded;
#[cfg(not(target_arch = "wasm32"))]
pub mod threaded;
pub mod wasm;

pub trait ChunkBuilderSchedulerTrait {
    fn new(executor: ChunkBuilderExecutor) -> Self;
    fn schedule(&mut self, job: ChunkBuilderJob);
    fn poll(&mut self) -> Option<ChunkBuilderUpdate>;
}