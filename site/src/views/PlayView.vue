<script setup lang="ts">
import { useRouter } from 'vue-router'
import { onMounted, ref } from 'vue'
import { isActive } from '../services/apiService'
import { webgpuSupported, webtransportSupported } from '../utils/compatibility'
import LoadingBar from '../components/LoadingBar.vue'
import NavigationBar from '../components/NavigationBar.vue'
import { nextTick } from 'vue'

let router = useRouter()

const loading = ref(true)
const hideNav = ref(false)

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

  const { loadGame } = await import('../utils/game')

  nextTick(() => {
    loading.value = false
    hideNav.value = true
  })
  await loadGame()  
}
onMounted(start)

</script>

<template>
  <main>
    <navigation-bar :hide="hideNav" :overlay="true" />
    <loading-bar v-if="loading" />
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

.loading-bar {
  position: fixed;
  inset: 0px;
  margin: auto;
}
</style>
