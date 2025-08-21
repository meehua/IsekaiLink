<script setup lang="ts">
import { onMounted, ref } from 'vue'

var countDown = ref(5);
onMounted(() => {
    const timer = setInterval(() => {
        if (countDown.value > 0) {
            countDown.value--;
        } else {
            clearInterval(timer);
            window.location.href = 'https://www.google.com';
        }
    }, 1000);
});
</script>

<template>
    <h1>DaLink</h1>
    <p>Jump in <span>{{ countDown }}</span seconds></p>
</template>

<style scoped>
</style>