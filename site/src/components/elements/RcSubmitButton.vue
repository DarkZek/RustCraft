<script setup lang="ts">

let props = defineProps<{
    label: string
}>()

</script>

<template>
    <input
        type="submit"
        class="rcButton"
        :label="props.label"
    >
</template>

<style lang="scss" scoped>

.rcButton {
    background: white;
    border: none;
    border-radius: 0px;
    color: var(--color-primary);
    font-family: "Londrina Solid";
    padding: 6px 20px;
    font-size: 24px;
    display: flex;
    align-items: center;
    cursor: pointer;

    &:hover {
        background: rgba(white, 0.9);
    }

    svg {
        margin-right: 6px;
    }
}

</style>