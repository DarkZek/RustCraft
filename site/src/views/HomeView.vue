<script setup lang="ts">
import { useRouter } from 'vue-router'
import { onMounted } from 'vue'
import { isActive } from '../services/apiService.js';

async function load() {
  if (false) {
    const { __wbg_set_wasm } = await import("../../wasm/rc_client_bg.js")
    const wasm = await import("../../wasm/rc_client_bg.wasm")
    __wbg_set_wasm(wasm);
    wasm.__wbindgen_start();
  } else {
    const { create_placeholder_game } = await import("../../wasm/rc_client_bg.js")
    create_placeholder_game()
  }
}

let router = useRouter()

if (!localStorage.getItem("token")) {
  router.push({ name: 'login' })
}

if (!navigator.gpu) {
  router.push({ name: 'unsupported' })
} else {
  onMounted(load)
}

isActive().then((v) => {
  if (v) {
    return
  }

  // Api is down
  router.push({ name: 'inactive' })
})

</script>

<template>
  <main>
    <div id="game"></div>
  </main>
</template>

<style>
canvas {
  display: block;
  touch-action: none;
  width: 100vw !important;
  height: 100vh !important;
}

canvas:focus {
  outline: none;
}
</style>
