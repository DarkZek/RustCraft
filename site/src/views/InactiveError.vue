<script lang="ts" setup>
import { useRouter } from 'vue-router';
import { isActive } from '../services/apiService';

const router = useRouter()

isActive().then((v) => {
  if (!v) {
    return
  }

  // Api is back up
  router.push({ name: 'play' })
})

</script>

<template>
    <div class="center">
        <span>Rustcraft is currently inactive. Check back later</span>
    </div>
</template>

<style lang="scss" scoped>

.center {
    text-align: center;
    margin-top: 20vh;
    font-family: "Londrina Solid";
}

</style>