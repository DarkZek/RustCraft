<script lang="ts" setup>

</script>

<template>
    <div
        class="loading-bar"
    >
        <div class="bouncing-ball">
            <div class="rotating">
            </div>
        </div>
        <div
            class="tiles"
        >
            <div class="t1">L</div>
            <div class="t2">O</div>
            <div class="t3">A</div>
            <div class="t4">D</div>
            <div class="t5">I</div>
            <div class="t6">N</div>
            <div class="t7">G</div>
            <div class="t8">.</div>
        </div>
    </div>
</template>

<style lang="scss" scoped>
@use 'sass:math';

$duration: 0.6;

$padding: 20px;
$mainSize: 35px;
$squareSize: 50px;
$totalSize: 70px;

$tiles: 8;

.loading-bar {
    min-width: ($totalSize * 3) + $mainSize;
    max-width: ($totalSize * 3) + $mainSize;
    padding-top: 40px;
    max-height: 140px;
}

@keyframes rotating {
    0% {
        transform: rotate(0deg);
    }
    2% {
        transform: rotate(0deg);
    }

    23% {
        transform: rotate(90deg);
    }
    27% {
        transform: rotate(90deg);
    }

    48% {
        transform: rotate(180deg);
    }
    52% {
        transform: rotate(180deg);
    }

    73% {
        transform: rotate(270deg);
    }
    77% {
        transform: rotate(270deg);
    }

    98% {
        transform: rotate(360deg);
    }
    100% {
        transform: rotate(360deg);
    }
}

.bouncing-ball {
    animation: bounce ($duration * 0.5s) infinite alternate;
    translate: $totalSize * 1.5;

    .rotating {
        width: $mainSize;
        height: $mainSize;
        animation: rotating ($duration * 4s) infinite;
        background: url('../assets/wood_top.png') center center;
        background-size: contain;
        animation-delay: -0.03s;
    }
}

@keyframes bounce {
    0% {
        transform: translateY(0px);
    }
    100% {
        transform: translateY(-30px);
    }
}

.tiles {

    display: flex;
    gap: 20px;
    translate: ($totalSize * 2.5) + 10px - math.div(($squareSize - $mainSize), 2px);

    @for $i from 1 through $tiles {
        .t#{$i} {
            position: absolute;
            width: $squareSize;
            height: $squareSize;
            background: rgba(0, 0, 0, 0.6);
            animation: tile ($duration * $tiles * 1s) linear infinite;
            animation-delay: $duration * ($i - 1) * 1s;
            opacity: 0;
            text-align: center;
            font-size: 30px;
            font-family: "Londrina Solid";
            color: white;
            animation-fill-mode: forwards;
            transition: all 0.3s ease-in-out;
        }
    }

    .t1 {
        rotate: 10deg;
    }
    .t2 {
        rotate: -5deg;
    }
    .t3 {
        rotate: 3deg;
    }
    .t4 {
        rotate: -1deg;
    }
    .t5 {
        rotate: -8deg;
    }
    .t6 {
        rotate: 4deg;
    }
    .t7 {
        rotate: -2deg;
    }
    .t8 {
        rotate: 6deg;
    }
}

$test: ($squareSize + $padding) * 3;

@keyframes tile {
    #{0/$tiles*100%} {
        opacity: 0.0;
        translate: (-$test * 0.0);
    }
    #{1/$tiles*100%} {
        opacity: 1.0;
        translate: (-$test * 0.33);
    }
    #{2/$tiles*100%} {
        opacity: 1.0;
        translate: (-$test * 0.66);
    }
    #{3/$tiles*100%} {
        opacity: 0.0;
        translate: (-$test * 1.0);
    }
}

</style>