// Overwritten by game
import ImageUrl from './terraria.jpg?url'

let canvas
let image

export function start_game(wasm_worker, on_startup) {
    const el = document.getElementById('game')
    canvas = document.createElement('canvas')
    el.appendChild(canvas)

    image = new Image()
    image.src = ImageUrl

    document.addEventListener('resize', resizeCanvas)
    image.onload = resizeCanvas

    on_startup()
}

function resizeCanvas() {
    canvas.width = screen.width
    canvas.height = screen.height

    const ctx = canvas.getContext("2d");
    ctx.drawImage(image, 0, 0, screen.width, screen.height)
}

export class WasmChunkExecutor {
    constructor() {
        console.log("Dummy WasmChunkExecutor started")
    }
}