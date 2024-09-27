<script lang="ts" setup>
import { useRouter } from 'vue-router';

const router = useRouter()

const webgpu = (navigator as any).gpu === undefined
const webtransport = window.WebTransport === undefined

const data: string[] = []
if (webgpu) { data.push('WebGPU') }
if (webtransport) { data.push('WebTransport') }

if (!webgpu && !webtransport) {
    // Browser does support
    router.push({ name: 'player' })
}

</script>

<template>
    <div class="center">
        <span>Your browser does not support {{ data.join(', ') }}. Update your browser, or if that doesn't work</span>
        <br>
        <a href="https://www.google.com/chrome/" target="_blank">Try Google Chrome</a>
    </div>
</template>

<style lang="scss" scoped>

.center {
    text-align: center;
    margin-top: 20vh;
}

</style>