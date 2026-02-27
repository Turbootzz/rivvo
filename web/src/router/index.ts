import { createRouter, createWebHistory } from 'vue-router'
import { useAuthStore } from '@/stores/auth'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'landing',
      component: () => import('@/views/LandingView.vue'),
    },
    {
      path: '/login',
      name: 'login',
      component: () => import('@/views/LoginView.vue'),
    },
    // Authenticated admin routes
    {
      path: '/boards',
      name: 'boards',
      component: () => import('@/views/BoardView.vue'),
      meta: { requiresAuth: true, showChrome: true },
    },
    {
      path: '/boards/:slug',
      name: 'board-detail',
      component: () => import('@/views/BoardDetailView.vue'),
      meta: { requiresAuth: true, showChrome: true },
    },
    {
      path: '/posts/:id',
      name: 'post-detail',
      component: () => import('@/views/PostDetailView.vue'),
      meta: { requiresAuth: true, showChrome: true },
    },
    {
      path: '/roadmap',
      name: 'roadmap',
      component: () => import('@/views/RoadmapView.vue'),
      meta: { requiresAuth: true, showChrome: true },
    },
    {
      path: '/changelog/manage',
      name: 'changelog-manage',
      component: () => import('@/views/ChangelogManageView.vue'),
      meta: { requiresAuth: true, showChrome: true },
    },
    // Public routes (no auth required)
    {
      path: '/:orgSlug/board/:boardSlug',
      name: 'public-board',
      component: () => import('@/views/PublicBoardView.vue'),
      meta: { publicView: true },
    },
    {
      path: '/:orgSlug/board/:boardSlug/post/:postId',
      name: 'public-post',
      component: () => import('@/views/PublicPostDetailView.vue'),
      meta: { publicView: true },
    },
    {
      path: '/:orgSlug/roadmap/:boardSlug',
      name: 'public-roadmap',
      component: () => import('@/views/RoadmapView.vue'),
      meta: { publicView: true },
    },
    {
      path: '/:orgSlug/changelog',
      name: 'public-changelog',
      component: () => import('@/views/ChangelogView.vue'),
      meta: { publicView: true },
    },
  ],
})

router.beforeEach((to) => {
  if (to.meta.requiresAuth) {
    const authStore = useAuthStore()
    if (!authStore.isAuthenticated) {
      return { name: 'login' }
    }
  }
  return true
})

export default router
