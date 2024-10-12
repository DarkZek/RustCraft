use tokio::runtime::{Builder, Runtime};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tokio::sync::mpsc::error::TryRecvError;
use tokio::task::{JoinHandle};
use crate::systems::chunk::builder::thread::executor::{ChunkBuilderExecutor, ChunkBuilderJob, ChunkBuilderUpdate};
use crate::systems::chunk::builder::thread::platform::ChunkBuilderSchedulerTrait;

pub struct ChunkBuilderScheduler {
    jobs_in: UnboundedSender<ChunkBuilderJob>,
    updates_out: UnboundedReceiver<ChunkBuilderUpdate>,
    handle: JoinHandle<usize>,
    runtime: Runtime
}

impl ChunkBuilderSchedulerTrait for ChunkBuilderScheduler {
    fn new(mut executor: ChunkBuilderExecutor) -> ChunkBuilderScheduler {

        let runtime = Builder::new_multi_thread()
            .thread_name("chunk-builder")
            .worker_threads(1)
            .build()
            .unwrap();

        let (in_send, mut in_recv): (UnboundedSender<ChunkBuilderJob>, UnboundedReceiver<ChunkBuilderJob>) =
            unbounded_channel();

        let (out_send, out_recv): (UnboundedSender<ChunkBuilderUpdate>, UnboundedReceiver<ChunkBuilderUpdate>) =
            unbounded_channel();

        let executor_handle = runtime.spawn(async move {
            while let Some(job) = in_recv.recv().await {
                // 1:1 since the channel buffers inputs for us
                executor.requests.push(job);
                let update = executor.build().unwrap();
                let _ = out_send.send(update);
            }
            0_usize
        });

        ChunkBuilderScheduler {
            jobs_in: in_send,
            updates_out: out_recv,
            handle: executor_handle,
            runtime
        }
    }

    fn schedule(&mut self, job: ChunkBuilderJob) {
        self.jobs_in.send(job).unwrap()
    }

    fn poll(&mut self) -> Option<ChunkBuilderUpdate> {
        match self.updates_out.try_recv() {
            Ok(val) => Some(val),
            Err(e) => {
                match e {
                    TryRecvError::Empty => {
                        None
                    }
                    _ => {
                        panic!("Chunk builder thread closed unexpectedly.")
                    }
                }
            }
        }
    }
}