import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '../views/HomeView.vue'
import PlayView from '../views/PlayView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: HomeView
    },
    {
      path: '/play',
      name: 'play',
      component: PlayView
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
    },
    {
      path: '/download',
      name: 'download',
      component: () => import('../views/DownloadView.vue')
    },
    {
      path: '/faq',
      name: 'faq',
      component: () => import('../views/FAQView.vue')
    }
  ]
})

export default router
