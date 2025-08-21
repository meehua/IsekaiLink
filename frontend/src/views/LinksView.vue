<template>
  <main class="flex justify-center items-center min-h-fill p-4">
    <div class="main-container">
      <slot>
        <h2 class="text-center text-2xl mb-4">{{ content }}</h2>
        <a href="#" class="text-link">示例链接</a>
      </slot>
    </div>
  </main>
</template>

<script setup lang="ts">
const content = '页面内容'
</script>

<style>

</style>