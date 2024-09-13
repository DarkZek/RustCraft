<script setup lang="ts">
import { useRouter } from 'vue-router'
import { onMounted } from 'vue'
import * as wasm from "../../wasm/rc_client_bg.wasm";
import { __wbg_set_wasm } from "../../wasm/rc_client_bg.js";
__wbg_set_wasm(wasm);


onMounted(() => {
  wasm.__wbindgen_start();
})

let router = useRouter()

if (!localStorage.getItem("token")) {
  router.push({ name: 'login' })
}
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
