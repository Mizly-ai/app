import { defineStore } from 'pinia'
import { ref, markRaw } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { FolderIcon } from '@/utils/icons.js'

const formatItemCount = (count) => {
  if (count === 0) return 'No items'
  if (count === 1) return '1 item'
  return `${count} items`
}

const validateStoreName = (name) => {
  return name && typeof name === 'string' && name.trim().length > 0
}

export const useStoresStore = defineStore('stores', () => {
  // State
  const stores = ref([])
  const storeDocuments = ref({})
  let unlistenStatusUpdate = null
  let unlistenStoreSync = null
  let unlistenDocumentSync = null

  // Helpers
  const findStore = (storeId) =>
    stores.value.find((s) => s.id === storeId)

  const isTerminalStatus = (status) =>
    status === 'completed' || status === 'active' || status === 'failed'

  const recalculateStoreStatus = (store) => {
    store.status = store.pendingCount > 0 ? 'processing' : 'completed'
  }

  // Actions
  const loadStores = async () => {
    try {
      const savedStores = await invoke('get_stores')

      if (savedStores && Array.isArray(savedStores)) {
        stores.value = savedStores
          .filter((s) => {
            if (!s || typeof s !== 'object') return false
            return validateStoreName(s.title)
          })
          .map((s) => ({
            id: s.id,
            title: s.title,
            geminiName: s.geminiName,
            subtitle: formatItemCount(s.documentCount || 0),
            icon: markRaw(FolderIcon),
            directoryPath: s.directoryPath,
            status: s.status,
            syncStatus: s.syncStatus || 'pending',
            documentCount: s.documentCount,
            pendingCount: s.pendingCount,
            failedCount: s.failedCount || 0
          }))
      } else {
        stores.value = []
      }
    } catch (e) {
      console.error('Failed to load stores:', e)
      stores.value = []
    }
  }

  const addStore = async (storeData) => {
    const name =
      typeof storeData === 'string' ? storeData : storeData.name
    const files =
      typeof storeData === 'object' ? storeData.files : []
    const directoryPath =
      typeof storeData === 'object'
        ? storeData.directoryPath
        : undefined

    try {
      const newStore = await invoke('create_store', {
        title: name,
        directoryPath
      })

      if (files.length > 0 && newStore.id) {
        const fileInfos = files.map((file) => ({
          name: file.displayName || file.filename || file.name,
          path: file.path || '',
          contentType: file.contentType || null,
          size: file.size ? Number(file.size) : null,
          hash: file.hash || null
        }))

        const documents = await invoke('upload_documents', {
          storeId: newStore.id,
          files: fileInfos
        })

        storeDocuments.value[newStore.id] = documents
      }

      const storeItem = {
        id: newStore.id,
        title: newStore.title,
        geminiName: newStore.geminiName,
        subtitle: formatItemCount(files.length),
        icon: markRaw(FolderIcon),
        directoryPath: newStore.directoryPath,
        status: files.length > 0 ? 'processing' : 'completed',
        syncStatus: newStore.syncStatus || 'pending',
        documentCount: files.length,
        pendingCount: files.length,
        failedCount: 0
      }

      stores.value.push(storeItem)
      return storeItem
    } catch (e) {
      console.error('Failed to add store:', e)
      return null
    }
  }

  const deleteStore = async (item) => {
    if (item.action === 'create') return false

    try {
      await invoke('delete_store', { id: item.id })
      const index = stores.value.findIndex((s) => s.id === item.id)
      if (index !== -1) {
        stores.value.splice(index, 1)
        delete storeDocuments.value[item.id]
      }
      return true
    } catch (e) {
      console.error('Failed to delete store:', e)
      return false
    }
  }

  const loadStoreDocuments = async (storeId) => {
    try {
      const documents = await invoke('get_documents', { storeId })
      storeDocuments.value[storeId] = documents || []

      const store = findStore(storeId)
      if (store) {
        store.subtitle = formatItemCount(documents.length)
        store.documentCount = documents.length
        store.pendingCount = documents.filter(
          (d) => !isTerminalStatus(d.status)
        ).length
        store.failedCount = documents.filter(
          (d) => d.status === 'failed'
        ).length
        recalculateStoreStatus(store)
      }

      return documents
    } catch (e) {
      console.error('Failed to load store documents:', e)
      storeDocuments.value[storeId] = []
      return []
    }
  }

  const getStoreDocuments = (storeId) => {
    return storeDocuments.value[storeId] || []
  }

  const updateStoreFromDocuments = (storeId) => {
    const store = findStore(storeId)
    const documents = storeDocuments.value[storeId]
    if (store && documents) {
      store.pendingCount = documents.filter(
        (d) => !isTerminalStatus(d.status)
      ).length
      store.failedCount = documents.filter(
        (d) => d.status === 'failed'
      ).length
      recalculateStoreStatus(store)
    }
  }

  const updateStoreCountsDirectly = (storeId, status) => {
    const store = findStore(storeId)
    if (store) {
      store.pendingCount = Math.max(0, (store.pendingCount || 0) - 1)
      if (status === 'failed') {
        store.failedCount = (store.failedCount || 0) + 1
      }
      recalculateStoreStatus(store)
    }
  }

  const updateDocumentStatus = (storeId, documentId, newStatus, geminiName) => {
    const documents = storeDocuments.value[storeId]
    if (documents) {
      const doc = documents.find((d) => d.id === documentId)
      if (doc) {
        doc.status = newStatus
        if (geminiName) {
          doc.geminiName = geminiName
        }
      }
      updateStoreFromDocuments(storeId)
    } else if (isTerminalStatus(newStatus)) {
      updateStoreCountsDirectly(storeId, newStatus)
    }
  }

  const updateStoreSyncStatus = (storeId, syncStatus, geminiName) => {
    const store = findStore(storeId)
    if (store) {
      store.syncStatus = syncStatus
      if (geminiName) {
        store.geminiName = geminiName
      }
    }
  }

  const updateDocumentSyncStatus = (
    storeId,
    documentId,
    syncStatus,
    geminiName,
    status
  ) => {
    const documents = storeDocuments.value[storeId]
    if (documents) {
      const doc = documents.find((d) => d.id === documentId)
      if (doc) {
        doc.syncStatus = syncStatus
        doc.status = status
        if (geminiName) {
          doc.geminiName = geminiName
        }
      }
      updateStoreFromDocuments(storeId)
    } else if (isTerminalStatus(status)) {
      updateStoreCountsDirectly(storeId, status)
    }
  }

  const setupStatusListener = async () => {
    if (unlistenStatusUpdate) return

    unlistenStatusUpdate = await listen('document-status-updated', (event) => {
      const { documentId, storeId, status, geminiName } = event.payload
      updateDocumentStatus(storeId, documentId, status, geminiName)
    })

    unlistenStoreSync = await listen('store-sync-updated', (event) => {
      const { storeId, syncStatus, geminiName } = event.payload
      updateStoreSyncStatus(storeId, syncStatus, geminiName)
    })

    unlistenDocumentSync = await listen('document-sync-updated', (event) => {
      const { documentId, storeId, syncStatus, geminiName, status } =
        event.payload
      updateDocumentSyncStatus(
        storeId,
        documentId,
        syncStatus,
        geminiName,
        status
      )
    })
  }

  const cleanupStatusListener = () => {
    if (unlistenStatusUpdate) {
      unlistenStatusUpdate()
      unlistenStatusUpdate = null
    }
    if (unlistenStoreSync) {
      unlistenStoreSync()
      unlistenStoreSync = null
    }
    if (unlistenDocumentSync) {
      unlistenDocumentSync()
      unlistenDocumentSync = null
    }
  }

  return {
    // State
    stores,
    storeDocuments,
    // Actions
    loadStores,
    addStore,
    deleteStore,
    loadStoreDocuments,
    getStoreDocuments,
    updateDocumentStatus,
    setupStatusListener,
    cleanupStatusListener
  }
})
