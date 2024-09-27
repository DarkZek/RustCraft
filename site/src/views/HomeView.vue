<script setup lang="ts">
import { useRouter } from 'vue-router'
import { onMounted } from 'vue'
import { isActive } from '../services/apiService';
import { loadGame } from '../utils/game';

let router = useRouter()

if (!localStorage.getItem("token")) {
  router.push({ name: 'login' })
} else {
    if (!(navigator as any).gpu) {
      router.push({ name: 'unsupported' })
    } else {
      onMounted(loadGame)
    }
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
