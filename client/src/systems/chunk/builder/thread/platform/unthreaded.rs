use std::time::Duration;
use tokio::time::Instant;
use crate::systems::chunk::builder::thread::executor::{ChunkBuilderExecutor, ChunkBuilderJob, ChunkBuilderUpdate};
use crate::systems::chunk::builder::thread::platform::ChunkBuilderSchedulerTrait;

/// Period between chunk builds
const CHUNK_BUILD_PERIOD: Duration = Duration::from_millis(10);

pub struct ChunkBuilderScheduler {
    executor: ChunkBuilderExecutor,
    last_built: Instant
}


impl ChunkBuilderSchedulerTrait for ChunkBuilderScheduler {
    fn new(executor: ChunkBuilderExecutor) -> Self {
        Self {
            executor,
            last_built: Instant::now(),
        }
    }

    fn schedule(&mut self, job: ChunkBuilderJob) {
        self.executor.requests.push(job);
    }

    fn poll(&mut self) -> Option<ChunkBuilderUpdate> {
        if self.last_built.elapsed() < CHUNK_BUILD_PERIOD {
            return None
        }

        // TODO: Better figure out how many chunks to render per frame. For now it's just one
        self.last_built = Instant::now();
        self.executor.build()
    }
}