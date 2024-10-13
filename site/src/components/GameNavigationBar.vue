<script lang="ts" setup>
import { useRouter } from 'vue-router'
import { ref } from 'vue'

const props = defineProps<{
    hide?: boolean
}>()

const router = useRouter()

</script>

<template>
    <!-- Recieves hover events to trigger the navigation bar -->
    <div
        class="navigation-trigger"
        :hide="props.hide"
    >
        <div
            class="navigation-bar"
        >
            <div
                class="constraint"
            >
                <div
                    class="logo"
                    @click="router.push({ name: 'home' })"
                >
                    <img
                        src="../assets/logo.png"
                        class="icon"
                    >
                    <span class="title">RustCraft</span>
                </div>
                <span class="links">
                    <router-link :to="{ name: 'home' }">Home</router-link>
                    <span class="dot">•</span>
                    <router-link :to="{ name: 'download' }">Download</router-link>
                    <span class="dot">•</span>
                    <router-link :to="{ name: 'faq' }">FAQ</router-link>
                </span>
            </div>
        </div>
    </div>
</template>

<style lang="scss" scoped>

.navigation-trigger {
    padding-bottom: 70px;
    position: absolute;
    transition: translate 0.5s ease-in-out;
    translate: 0px -70px !important;

    &:hover, &[hide=false] {
        translate: 0px 0px !important;
    }
}

.navigation-bar {
    background: #6100FF;
    min-height: 70px;
    max-height: 70px;
    width: 100vw;
    padding: 14px 0px 8px 0px;
}

.constraint {
    max-width: 802px;
    margin: auto;
    font-family: "Londrina Solid";
    color: white;
    display: flex;
}

.dot {
    padding: 0px 12px 0px 12px;
}

.logo {
    cursor: pointer;
    display: flex;

    .title {
        font-size: 34px;
        padding-left: 18px;
        line-height: 1.25;
    }

    .icon {
        height: 46px;
    }
}

.links {
    padding-left: 36px;
    font-size: 22px;
    padding-top: 4px;

    a {
        font-weight: 100;
        color: white;
        text-decoration: none;
        transition: color 0.3s ease-in-out;
    }

    a:hover {
        color: #d2b6ff;
    }
}

</style>