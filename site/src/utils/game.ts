import ChunkWorker from './worker?worker'

export async function loadGame(onLoaded: () => void) {
    return new Promise((resolve) => {
        setTimeout(
            async () => {
                const chunk_worker = new ChunkWorker()
    
                const { start_game } = await import("../../wasm/rc_client.js")
                start_game(chunk_worker, onLoaded);
                resolve(true)
            },
            5000
        )
    })
}
