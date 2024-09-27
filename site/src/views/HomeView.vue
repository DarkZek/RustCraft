<script setup lang="ts">
import { useRouter } from 'vue-router'
import { onMounted, ref } from 'vue'
import { isActive } from '../services/apiService';
import { loadGame } from '../utils/game';
import { webgpuSupported, webtransportSupported } from '../utils/compatibility';

let router = useRouter()

async function start() {
  if (!localStorage.getItem("token")) {
    router.push({ name: 'login' })
    return
  }

  if (!webgpuSupported || !webtransportSupported) {
    router.push({ name: 'unsupported' })
    return
  }
  
  if (!await isActive()) {
    // Api is down
    router.push({ name: 'inactive' })
    return
  }

  loadGame()
}
onMounted(start)

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
