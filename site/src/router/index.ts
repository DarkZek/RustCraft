import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '../views/HomeView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'play',
      component: HomeView
    },
    {
      path: '/login',
      name: 'login',
      component: () => import('../views/LoginView.vue')
    },
    {
      path: '/unsupported',
      name: 'unsupported',
      component: () => import('../views/UnsupportedBrowser.vue')
    },
    {
      path: '/inactive',
      name: 'inactive',
      component: () => import('../views/InactiveError.vue')
    }
  ]
})

export default router
