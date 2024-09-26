<script lang="ts" setup>

const webgpu = navigator.gpu === undefined
const webtransport = window.WebTransport === undefined

const data = []
if (webgpu) { data.push('WebGPU') }
if (webtransport) { data.push('WebTransport') }

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