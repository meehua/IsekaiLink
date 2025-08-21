<template>
  <h1 class="font-bold mb-8 text-center bg-gradient-to-r from-cyan-400
    to-blue-500 bg-clip-text text-6xl font-extrabold text-transparent
    ">異界転門</h1>

  <form @submit.prevent="handleLogin" class="space-y-6">
    <div class="relative group">
      <input v-model="password" type="password" required placeholder="時空門扉の鍵を入力" class="input-field w-full px-6 py-4 text-lg rounded-2xl bg-white/20 backdrop-blur-sm
                  border-2 border-indigo-300/50 hover:border-indigo-300 focus:border-indigo-400
                  text-black dark:text-white placeholder-black/40 dark:placeholder-indigo-100/80 focus:ring-0 focus:outline-none
                  transition-all duration-300">
      <span class="absolute inset-y-0 right-4 flex items-center pr-2">
        <KeyIcon
          class="w-6 h-6 text-gray-400/80 dark:text-indigo-200/80 group-focus-within:text-gray-500 dark:group-focus-within:text-indigo-300 transition-colors" />
      </span>
    </div>

    <button type="submit" :disabled="isLoading" class="w-full py-4 px-6 flex items-center justify-center gap-2
                text-lg font-semibold rounded-2xl bg-indigo-600 hover:bg-indigo-700 
                focus:outline-none focus:ring-2 focus:ring-indigo-400 focus:ring-offset-2
                text-white shadow-lg hover:shadow-indigo-500/30
                transition-all duration-300
                disabled:opacity-75 disabled:cursor-not-allowed">
      <span v-if="!isLoading">見参！</span>
      <svg v-else class="animate-spin h-6 w-6 text-white" xmlns="http://www.w3.org/2000/svg" fill="none"
        viewBox="0 0 24 24">
        <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
        <path class="opacity-75" fill="currentColor"
          d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z">
        </path>
      </svg>
    </button>
  </form>

</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { KeyIcon } from '@heroicons/vue/24/outline'

const password = ref('')
const isLoading = ref(false)
const router = useRouter()
const handleLogin = () => {
  isLoading.value = true
  // 模拟异步请求
  setTimeout(() => {
    isLoading.value = false
    router.push('/links')
  }, 2000)
}
</script>

<style>
/* 自定义输入框光标颜色 */
.input-field:focus {
  caret-color: #818cf8;
}

/* 加载动画关键帧 */
@keyframes spin {
  from {
    transform: rotate(0deg);
  }

  to {
    transform: rotate(360deg);
  }
}

.animate-spin {
  animation: spin 1s linear infinite;
}
</style>
