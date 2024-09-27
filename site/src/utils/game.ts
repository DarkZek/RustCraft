export async function loadGame() {
    if (import.meta.env.VITE_PLACEHOLDER_GAME) {
        const { create_placeholder_game } = await import("../../wasm/rc_client_bg.js")
        create_placeholder_game()
    } else {
        const { __wbg_set_wasm } = await import("../../wasm/rc_client_bg.js")
        const wasm = await import("../../wasm/rc_client_bg.wasm")
        __wbg_set_wasm(wasm);
        wasm.__wbindgen_start();
    }
}
