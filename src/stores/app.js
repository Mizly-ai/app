import { defineStore } from 'pinia'
import { ref, computed, markRaw } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'
import { HomeIcon, StoreIcon } from '@/utils/icons.js'

export const useAppStore = defineStore('app', () => {
  const router = useRouter()
  const route = useRoute()

  // State
  const searchQuery = ref('')

  const tabs = ref([
    { id: 'home', label: 'Main', icon: markRaw(HomeIcon), route: '/' },
    { id: 'stores', label: 'Stores', icon: markRaw(StoreIcon), count: 0, route: '/stores' }
  ])

  // Getters
  const activeTab = computed(() => {
    const path = route.path
    if (path === '/' || route.name === 'home') return 'home'
    if (path.startsWith('/stores')) return 'stores'
    if (path.startsWith('/chats')) return 'chats'
    return 'home'
  })

  // Actions
  const setSearchQuery = (query) => {
    searchQuery.value = query
  }

  const clearSearchQuery = () => {
    searchQuery.value = ''
  }

  const changeTab = (tabId) => {
    const tab = tabs.value.find(t => t.id === tabId)
    if (tab?.route) {
      router.push(tab.route)
    }
  }

  const switchTab = (direction = 'next') => {
    const tabIds = tabs.value.map(t => t.id)
    const currentIndex = tabIds.indexOf(activeTab.value)

    let nextIndex
    if (direction === 'prev') {
      nextIndex = (currentIndex - 1 + tabIds.length) % tabIds.length
    } else {
      nextIndex = (currentIndex + 1) % tabIds.length
    }

    changeTab(tabIds[nextIndex])
  }

  const enterAiChat = () => {
    router.push('/chats')
  }

  const exitAiChat = () => {
    clearSearchQuery()
    router.back()
  }

  const hideWindow = async () => {
    await invoke('hide_window')
  }

  return {
    // State
    searchQuery,
    tabs,
    // Getters
    activeTab,
    // Actions
    setSearchQuery,
    clearSearchQuery,
    changeTab,
    switchTab,
    enterAiChat,
    exitAiChat,
    hideWindow
  }
})
