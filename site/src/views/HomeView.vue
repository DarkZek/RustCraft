<script setup lang="ts">
import { useRouter } from 'vue-router'
import NavigationBar from '../components/NavigationBar.vue'

let router = useRouter()

</script>

<template>
  <main>
    <navigation-bar />
    <button label="Play" @click="router.push({ name: 'play' })">Play</button>
  </main>
</template>

<style>

button {
  font-size: 50px;
  max-width: 200px;
  left: 0px;
  right: 0px;
  margin: auto;
  margin-top: 80px;
  background-color: green;
  position: absolute;
}


</style>
