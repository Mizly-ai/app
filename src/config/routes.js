import { createRouter, createMemoryHistory } from 'vue-router'

const routes = [
  {
    path: '/',
    name: 'home',
    component: () => import('@/views/home/index.vue')
  },
  {
    path: '/stores',
    name: 'stores',
    component: () => import('@/views/stores/index.vue')
  },
  {
    path: '/stores/new',
    name: 'stores-new',
    component: () => import('@/views/stores/new.vue')
  },
  {
    path: '/stores/:id',
    name: 'stores-show',
    component: () => import('@/views/stores/show.vue'),
    props: true
  },
  {
    path: '/chats',
    name: 'chats',
    component: () => import('@/views/chats/show.vue')
  },
  {
    path: '/settings/locales',
    name: 'settings-locales',
    component: () => import('@/views/settings/locales/index.vue')
  },
  {
    path: '/settings/api-key',
    name: 'settings-api-key',
    component: () => import('@/views/settings/api-key/index.vue')
  }
]

const router = createRouter({
  history: createMemoryHistory(),
  routes
})

// Navigation guard for initialization
router.beforeEach(async (to, from, next) => {
  // Dynamically import to avoid Pinia initialization issues
  const { useAuthStore } = await import('@/stores/auth')
  const { useLocaleStore } = await import('@/stores/locale')
  const authStore = useAuthStore()
  const localeStore = useLocaleStore()

  // Initialize stores if not already done
  if (!authStore.isInitialized) {
    await authStore.initialize()
  }
  if (!localeStore.isInitialized) {
    await localeStore.initialize()
  }

  next()
})

export default router
