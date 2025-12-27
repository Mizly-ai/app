import { computed, markRaw, onMounted, onUnmounted } from 'vue'
import { useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { FolderPlusIcon } from '@/utils/icons.js'
import { highlightSearchMatch } from '@/utils/helpers.js'
import { useAppStore } from '@/stores/app'
import { useStoresStore } from '@/stores/stores'

/**
 * Store list composable
 * Handles loading, filtering, and operations for the stores list view
 *
 * @returns {Object} Store list state and methods
 */
export function useStoreList() {
  const { t } = useI18n()
  const router = useRouter()
  const appStore = useAppStore()
  const storesStore = useStoresStore()

  // "New Store" button item
  const newStoreButton = computed(() => ({
    id: 'new-store',
    title: t('stores.newStore'),
    icon: markRaw(FolderPlusIcon),
    action: 'create'
  }))

  // Filtered stores based on search query
  const filteredStores = computed(() => {
    if (!appStore.searchQuery) return storesStore.stores || []

    const query = appStore.searchQuery.toLowerCase()
    return (storesStore.stores || []).filter(item =>
      item.title.toLowerCase().includes(query)
    )
  })

  // Display items = filtered stores + new store button
  const displayItems = computed(() => {
    return [...filteredStores.value, newStoreButton.value]
  })

  // Navigate to store or create new
  const selectItem = (item) => {
    if (item.action === 'create') {
      router.push('/stores/new')
      return
    }
    router.push(`/stores/${item.id}`)
  }

  // Handle enter key on selected item
  const handleSelectAtIndex = (index) => {
    const items = displayItems.value
    if (items.length > 0 && index >= 0 && index < items.length) {
      selectItem(items[index])
    }
  }

  // Delete a store
  const deleteStore = async (item) => {
    if (item.action === 'create') return false
    await storesStore.deleteStore(item)
    return true
  }

  // Handle delete at specific index
  const handleDeleteAtIndex = async (index) => {
    const items = displayItems.value
    if (items.length > 0 && index >= 0 && index < items.length) {
      const item = items[index]
      if (item.action !== 'create') {
        await deleteStore(item)
        return true
      }
    }
    return false
  }

  // Navigate to create new store
  const createNew = () => {
    router.push('/stores/new')
  }

  // Highlight search match in text (XSS-safe)
  const highlightMatch = (text) => highlightSearchMatch(text, appStore.searchQuery)

  // Lifecycle management
  onMounted(async () => {
    await storesStore.setupStatusListener()
    await storesStore.loadStores()
  })

  onUnmounted(() => {
    storesStore.cleanupStatusListener()
  })

  return {
    // State
    displayItems,
    searchQuery: computed(() => appStore.searchQuery),
    // Methods
    selectItem,
    handleSelectAtIndex,
    deleteStore,
    handleDeleteAtIndex,
    createNew,
    highlightMatch,
    // App store methods for tab navigation
    switchTab: appStore.switchTab,
    enterAiChat: () => {
      appStore.enterAiChat()
      appStore.clearSearchQuery()
    }
  }
}
