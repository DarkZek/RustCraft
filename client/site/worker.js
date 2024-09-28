console.log('Starting worker.js')

import wasm_bindgen, { WasmChunkExecutor } from './api/wasm.js';

async function init_wasm_in_worker() {
    // Load the Wasm file by awaiting the Promise returned by `wasm_bindgen`.
    console.log('Initializing web worker')

    await wasm_bindgen("/api/wasm.wasm");

    self.onmessage = async event => {
        console.log('Received web worker configuration')

        // Creation
        let chunk_builder = new WasmChunkExecutor(event.data)

        self.onmessage = async event => {
            let worker_result = chunk_builder.job(event.data);

            // Send response back to be handled by callback in main thread.
            self.postMessage(worker_result);
        };
    }
};

init_wasm_in_worker();