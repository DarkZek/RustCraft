console.log('Starting worker.js')

function waitForMessage(): Promise<MessageEvent> {
    return new Promise((resolve) => {
        self.onmessage = (event) => {
            resolve(event)
        }
    })
}

async function init_wasm_in_worker() {
    // Load the Wasm file by awaiting the Promise returned by `wasm_bindgen`.
    console.log('Initializing web worker')

    const { WasmChunkExecutor } = await import("../../wasm/rc_client.js")

    self.postMessage('started');

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

export default {}