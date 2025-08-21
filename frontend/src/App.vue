<script lang="ts" setup name="App">
import { RouterView, RouterLink } from 'vue-router'
import { ref, onMounted } from 'vue'

const imageLoaded = ref(false)
const bgImage = ref<HTMLImageElement | null>(null)

const handleImageLoad = () => {
  imageLoaded.value = true
}

onMounted(() => {
  // 处理可能缓存的图片,如果图片已经在缓存里了且已经秒开,直接调用回调函数。
  // 这只是个以防万一的东东，不过感觉有点多事了。就当学习新知识吧。
  if (bgImage.value?.complete) {
    handleImageLoad()
  }
})
</script>
<template>
  <div class="h-dvh w-dvw fixed dark:bg-gray-900 bg-gray-300">
    <!-- 背景图片 -->
    <img ref="bgImage" class="z-0 absolute w-full h-full object-cover dark:brightness-50
    transition-opacity duration-3000 opacity-0" :class="{ 'opacity-100': imageLoaded }"
      src="https://www.loliapi.com/acg/" @load="handleImageLoad" />

    <!-- 内容容器 -->
    <div class="z-10 absolute inset-0 flex items-center justify-center">
      <!-- 窄屏背景样式+垂直居中包裹层 -->
      <div class="grid place-items-center h-dvh w-dvw md:min-w-xs md:max-w-md md:max-h-fit
      bg-white/30 dark:bg-black/30 md:bg-transparent md:dark:bg-transparent
      backdrop-blur-sm md:backdrop-blur-none">

        <!-- 宽屏背景（盒子）样式+垂直居中内容层 -->
        <div class=" text-gray-900 dark:text-white whitespace-break-spaces
        md:backdrop-blur-sm md:bg-white/30 md:dark:bg-black/30 overflow-y-auto md:shadow-xl/30
        md:dark:shadow-white md:p-8 md:rounded-3xl md:border-1 md:border-white md:dark:border-black
        md:shadow-lg">
          <RouterView />
        </div>
      </div>
    </div>
  </div>
  <!-- <ModeToggleButton /> -->
</template>

<style>
@import "tailwindcss";
</style>