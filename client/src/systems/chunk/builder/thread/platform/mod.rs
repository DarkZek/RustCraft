use crate::systems::chunk::builder::thread::executor::{ChunkBuilderJob, ChunkBuilderUpdate};

pub mod unthreaded;
pub mod threaded;

pub trait ChunkBuilderSchedulerTrait {
    fn new(executor: Self) -> Self;
    fn schedule(&mut self, job: ChunkBuilderJob);
    fn poll(&mut self) -> Option<ChunkBuilderUpdate>;
}