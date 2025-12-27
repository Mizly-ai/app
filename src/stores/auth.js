import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export const useAuthStore = defineStore('auth', () => {
  // State
  const apiKey = ref(null)
  const error = ref(null)
  const isInitialized = ref(false)

  // Getters
  const hasApiKey = computed(() => !!apiKey.value)

  // Masked API key for display (e.g., "AIza...xyz")
  const maskedApiKey = computed(() => {
    if (!apiKey.value) return ''
    const key = apiKey.value
    if (key.length <= 8) return '••••••••'
    return `${key.slice(0, 4)}${'•'.repeat(Math.min(key.length - 7, 20))}${key.slice(-3)}`
  })

  // Actions
  const loadApiKey = async () => {
    try {
      const key = await invoke('get_api_key')
      apiKey.value = key
    } catch (e) {
      // No API key set yet, that's fine
      apiKey.value = null
    }
  }

  const setApiKey = async (key) => {
    error.value = null
    try {
      await invoke('set_api_key', { apiKey: key })
      apiKey.value = key
    } catch (e) {
      error.value = e.toString()
      throw e
    }
  }

  const clearApiKey = async () => {
    error.value = null
    try {
      await invoke('clear_api_key')
      apiKey.value = null
    } catch (e) {
      error.value = e.toString()
      throw e
    }
  }

  const initialize = async () => {
    if (!isInitialized.value) {
      await loadApiKey()
      isInitialized.value = true
    }
  }

  return {
    // State
    apiKey,
    error,
    isInitialized,
    // Getters
    hasApiKey,
    maskedApiKey,
    // Actions
    loadApiKey,
    setApiKey,
    clearApiKey,
    initialize
  }
})
