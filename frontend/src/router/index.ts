import { createRouter, createWebHistory } from 'vue-router'
import JumpView from '@/views/JumpView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'jump',
      component: JumpView,
    },
    {
      path: '/transcend-gate',
      name: 'login',
      // route level code-splitting
      // this generates a separate chunk (About.[hash].js) for this route
      // which is lazy-loaded when the route is visited.
      component: () => import('@/views/LoginView.vue'),
    },
    {
      path: '/links',
      name: 'links',
      component: () => import('@/views/LinksView.vue'),
    },
  ],
})

export default router
