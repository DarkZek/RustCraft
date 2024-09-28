import ChunkWorker from './worker?worker'

export async function loadGame() {
    const chunk_worker = new ChunkWorker()

    const { start_game } = await import("../../wasm/rc_client.js")
    start_game(chunk_worker);
}
