import { ref, computed, watch, onMounted, onUnmounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'
import { useAppStore } from '@/stores/app'
import { useStoresStore } from '@/stores/stores'

/**
 * Store detail composable
 * Handles loading, filtering, and operations for a single store's documents
 *
 * @returns {Object} Store state and methods
 */
export function useStoreDetail() {
  const route = useRoute()
  const router = useRouter()
  const appStore = useAppStore()
  const storesStore = useStoresStore()

  const documents = ref([])
  const selectedStore = ref(null)

  // Load store and documents
  const loadStoreData = async () => {
    const storeId = route.params.id

    if (!storeId) {
      router.push('/stores')
      return
    }

    // Ensure stores are loaded first
    if (storesStore.stores.length === 0) {
      await storesStore.loadStores()
    }

    // Find store from store by ID (UUID string)
    selectedStore.value = storesStore.stores.find(s => s.id === storeId)

    if (!selectedStore.value) {
      router.push('/stores')
      return
    }

    // Load documents
    documents.value = await storesStore.loadStoreDocuments(storeId)
  }

  // Filtered documents based on search query
  const filteredDocuments = computed(() => {
    if (!appStore.searchQuery) return documents.value

    const query = appStore.searchQuery.toLowerCase()
    return documents.value.filter((doc) =>
      doc.name.toLowerCase().includes(query) || doc.path.toLowerCase().includes(query)
    )
  })

  // Open directory in Finder
  const openDirectory = async () => {
    try {
      const dirPath = selectedStore.value?.directoryPath
      if (dirPath) {
        await invoke('open_directory', { path: dirPath })
      }
    } catch (error) {
      console.error('Failed to open directory:', error)
    }
  }

  // Open file with default application
  const openFile = async (file) => {
    try {
      if (file.path) {
        await invoke('open_file', { path: file.path })
      }
    } catch (error) {
      console.error('Failed to open file:', error)
    }
  }

  // Delete document from store
  const deleteDocument = async (doc) => {
    try {
      await invoke('delete_document', { id: doc.id })
      documents.value = await storesStore.loadStoreDocuments(selectedStore.value.id)
    } catch (e) {
      console.error('Failed to delete document:', e)
    }
  }

  // Navigate back to stores list
  const goBack = () => {
    appStore.clearSearchQuery()
    router.push('/stores')
  }

  // Lifecycle management
  onMounted(async () => {
    await storesStore.setupStatusListener()
    await loadStoreData()
  })

  onUnmounted(() => {
    storesStore.cleanupStatusListener()
  })

  // Reload when route changes
  watch(() => route.params.id, () => {
    loadStoreData()
  })

  return {
    // State
    documents,
    selectedStore,
    filteredDocuments,
    searchQuery: computed(() => appStore.searchQuery),
    // Methods
    loadStoreData,
    openDirectory,
    openFile,
    deleteDocument,
    goBack
  }
}
