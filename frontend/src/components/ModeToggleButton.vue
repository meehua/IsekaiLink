<template>
  <button
    @click="toggleTheme"
    class="fixed top-5 right-5 z-20 px-4 py-2 rounded-lg transition-colors duration-300"
    :class="themeClasses"
  >
    切换模式
  </button>
</template>

<script setup lang="ts">
import { ref, watchEffect } from 'vue'

const isDark = ref(false)

// 初始化主题
function initTheme() {
  const savedTheme = localStorage.getItem('theme') || 
    (window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light')
  isDark.value = savedTheme === 'dark'
  updateTheme()
}

// 切换主题
function toggleTheme() {
  isDark.value = !isDark.value
  updateTheme()
}

// 更新主题状态
function updateTheme() {
  document.documentElement.classList.toggle('dark', isDark.value)
  localStorage.setItem('theme', isDark.value ? 'dark' : 'light')
  updateBackgroundBrightness()
}

// 更新背景亮度
function updateBackgroundBrightness() {
  document.documentElement.style.setProperty(
    '--brightness',
    isDark.value ? '0.3' : '1'
  )
}

// 样式类
const themeClasses = ref([
  'bg-blue-600 hover:bg-blue-700',
  'dark:bg-blue-500 dark:hover:bg-blue-600',
  'text-white'
])

// 初始化
initTheme()
</script>