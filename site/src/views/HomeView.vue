<script setup lang="ts">
import { useRouter } from 'vue-router'
import NavigationBar from '../components/NavigationBar.vue'
import RcButton from '../components/elements/RcButton.vue';

let router = useRouter()

</script>

<template>
  <main>
    <navigation-bar />
    <rc-button @click="router.push({ name: 'play' })">Play</rc-button>
  </main>
</template>

<style lang="scss" scoped>

button {
  font-size: 50px !important;
  max-width: 200px;
  left: 0px;
  right: 0px;
  margin: auto;
  margin-top: 80px;
  position: absolute;
}

</style>
