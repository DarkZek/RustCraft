<script lang="ts" setup>
import { useRouter } from 'vue-router';
import { webgpuSupported, webtransportSupported } from '../utils/compatibility';

const router = useRouter()

const data: string[] = []
if (webgpuSupported) { data.push('WebGPU') }
if (webtransportSupported) { data.push('WebTransport') }

if (!webgpuSupported && !webtransportSupported) {
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