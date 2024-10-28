use bevy::prelude::{warn};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver};
use tokio::task::{JoinHandle};
use wasm_bindgen::__rt::VectorIntoJsValue;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::prelude::{Closure, wasm_bindgen};
use crate::systems::chunk::builder::thread::executor::{ChunkBuilderExecutor, ChunkBuilderJob, ChunkBuilderUpdate};
use crate::systems::chunk::builder::thread::platform::ChunkBuilderSchedulerTrait;
use js_sys::Uint8Array;
use serde::{Deserialize, Serialize};
use web_sys::{Event, MessageEvent, Worker};
use rc_shared::block::BlockStates;
use rc_shared::block::types::VisualBlock;
use std::borrow::BorrowMut;
use std::collections::HashMap;
use fnv::FnvBuildHasher;
use rc_shared::atlas::{TEXTURE_ATLAS, TextureAtlas, TextureAtlasIndex};
use crate::start::WASM_CONTEXT;

unsafe impl Sync for ChunkBuilderScheduler {}
unsafe impl Send for ChunkBuilderScheduler {}

pub struct ChunkBuilderScheduler {
    worker_handle: Worker,
    job_receive: Closure<dyn FnMut(Event)>,
    recv_chunks: UnboundedReceiver<ChunkBuilderUpdate>
}

impl ChunkBuilderSchedulerTrait for ChunkBuilderScheduler {
    fn new(executor: ChunkBuilderExecutor) -> ChunkBuilderScheduler {

        let (send_chunks, recv_chunks) = unbounded_channel();

        let worker_handle = WASM_CONTEXT.get().unwrap().chunk_worker.clone();

        // Recieves a completed job from the worker
        let job_receive = Closure::<dyn FnMut(_)>::new(move |event: Event| {

            let Some(message) = event.dyn_ref::<MessageEvent>() else {
                warn!("Non message sent to job_receive");
                return;
            };

            let data = Uint8Array::new(&message.data());
            let worker_data = bincode::deserialize::<ChunkBuilderUpdate>(data.to_vec().as_slice()).unwrap();

            send_chunks.send(worker_data).unwrap();
        });

        worker_handle.set_onmessage(Some(
            job_receive.as_ref().unchecked_ref()
        ));

        // Send init data
        let init = InitWasmChunkExecutor {
            atlas_map: TEXTURE_ATLAS.get().index.clone()
        };

        let worker_data = bincode::serialize(&init).unwrap();

        let array = Uint8Array::from(worker_data.as_slice());

        worker_handle.post_message(&*array).unwrap();

        ChunkBuilderScheduler {
            worker_handle,
            job_receive,
            recv_chunks,
        }
    }

    fn schedule(&mut self, job: ChunkBuilderJob) {
        let worker_data = bincode::serialize(&job).unwrap();

        let array = Uint8Array::from(worker_data.as_slice());

        // TODO: Bundle multiple
        let _ = self.worker_handle.borrow_mut().post_message(&*array);
    }

    fn poll(&mut self) -> Option<ChunkBuilderUpdate> {
        self.recv_chunks.try_recv().ok()
    }
}



#[derive(Serialize, Deserialize)]
struct InitWasmChunkExecutor {
    // TODO: Replace this with Shared memory between workers
    atlas_map: HashMap<String, TextureAtlasIndex, FnvBuildHasher>
}

#[wasm_bindgen]
pub struct WasmChunkExecutor {
    executor: ChunkBuilderExecutor
}

#[wasm_bindgen]
impl WasmChunkExecutor {
    #[wasm_bindgen(constructor)]
    pub fn new(value: &JsValue) -> JsValue {
        let data = Uint8Array::new(value);

        let worker_data = bincode::deserialize::<InitWasmChunkExecutor>(data.to_vec().as_slice()).unwrap();

        log("[Web Worker] Successfully created WasmChunkExecutor instance on worker");

        WasmChunkExecutor::from(worker_data).into()
    }

    #[wasm_bindgen]
    pub fn job(&mut self, value: &JsValue) -> JsValue {
        let data = Uint8Array::new(value);

        let worker_data = bincode::deserialize::<ChunkBuilderJob>(data.to_vec().as_slice()).unwrap();

        self.executor.requests.push(worker_data);
        let output = self.executor.build().unwrap();

        let worker_data = bincode::serialize(&output).unwrap();

        let array = Uint8Array::from(worker_data.as_slice());

        array.into()
    }
}

// TODO: Use shared memory instead
// https://blog.scottlogic.com/2019/07/15/multithreaded-webassembly.html
impl From<InitWasmChunkExecutor> for WasmChunkExecutor {
    fn from(value: InitWasmChunkExecutor) -> Self {

        let mut atlas = TextureAtlas::blank();
        atlas.index = value.atlas_map;
        TEXTURE_ATLAS.set(atlas);

        let mut block_states = BlockStates::new();

        block_states.calculate_states();

        let executor = ChunkBuilderExecutor::new(block_states);

        WasmChunkExecutor {
            executor,
        }
    }
}


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}