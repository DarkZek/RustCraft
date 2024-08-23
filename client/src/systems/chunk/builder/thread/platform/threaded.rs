use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tokio::task::JoinHandle;
use crate::systems::chunk::builder::thread::executor::{ChunkBuilderExecutor, ChunkBuilderJob, ChunkBuilderUpdate};
use crate::systems::chunk::builder::thread::platform::ChunkBuilderSchedulerTrait;

pub struct ChunkBuilderScheduler {
    jobs_in: UnboundedSender<ChunkBuilderJob>,
    updates_out: UnboundedReceiver<ChunkBuilderUpdate>,
    handle: JoinHandle<usize>
}

impl ChunkBuilderSchedulerTrait for ChunkBuilderScheduler {
    fn new(mut executor: ChunkBuilderExecutor) -> ChunkBuilderScheduler {

        let (in_send, mut in_recv): (UnboundedSender<ChunkBuilderJob>, UnboundedReceiver<ChunkBuilderJob>) =
            unbounded_channel();

        let (out_send, mut out_recv): (UnboundedSender<ChunkBuilderUpdate>, UnboundedReceiver<ChunkBuilderUpdate>) =
            unbounded_channel();

        let executor_handle = tokio::spawn(async move || {
            while let Ok(job) = in_recv.try_recv() {
                // 1:1 since the channel buffers inputs for us
                executor.requests.push(job);
                let update = executor.build().unwrap();
                out_send.send(update).unwrap();
            }
            0_usize
        });

        ChunkBuilderScheduler {
            jobs_in: in_send,
            updates_out: out_recv,
            handle: executor_handle
        }
    }

    fn schedule(&mut self, job: ChunkBuilderJob) {
        self.jobs_in.send(job).unwrap()
    }

    fn poll(&mut self) -> Option<ChunkBuilderUpdate> {
        self.updates_out.recv()
    }
}