
<template>
    <button
        class="rcButton"
    >
        <slot></slot>
    </button>
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
    justify-content: center;
    cursor: pointer;

    &:hover {
        background: rgba(white, 0.9);
    }

    svg {
        margin-right: 6px;
    }
}

</style>