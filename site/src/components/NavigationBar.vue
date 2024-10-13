<script lang="ts" setup>
import { useRouter } from 'vue-router'
import { Icon } from '@iconify/vue'
import RcButton from './elements/RcButton.vue';

const router = useRouter()

</script>

<template>
    <div
        class="navigation-bar"
    >
        <div
            class="constraint">
            <div>
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
                    <router-link :to="{ name: 'play' }">Play</router-link>
                    <span class="dot">•</span>
                    <router-link :to="{ name: 'download' }">Download</router-link>
                    <span class="dot">•</span>
                    <router-link :to="{ name: 'faq' }">FAQ</router-link>
                </span>
                <div class="spacer" />
                <rc-button
                    @click.prevent="router.push({ name: 'login' })"
                >
                    <Icon icon="mingcute:happy-line" />
                    Login
                </rc-button>
            </div>
        </div>
    </div>
</template>

<style lang="scss" scoped>

.navigation-bar {
    background: var(--color-primary);
    min-height: 70px;
    max-height: 70px;
    width: 100vw;
    padding: 14px 0px 8px 0px;
    transition: translate 0.5s ease-in-out;
    display: flex;
    justify-content: center;
}

.constraint {
    max-width: 802px;
    font-family: "Londrina Solid";
    color: white;
    flex-grow: 1;
    padding: 0px 16px;

    > div {
        display: flex;
    }
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

.spacer {
    display: flex;
    flex-grow: 1;
}

</style>