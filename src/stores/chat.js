import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useStoresStore } from './stores'

const CHARS_PER_FRAME = 6

export const useChatStore = defineStore('chat', () => {
  // State
  const isLoading = ref(false)
  const response = ref('')
  const error = ref(null)
  const isStreaming = ref(false)
  const sources = ref([])
  const currentQuestion = ref('')

  let typewriterQueue = []
  let isTyping = false
  let isCancelled = false

  // Actions
  const reset = () => {
    response.value = ''
    error.value = null
    isLoading.value = false
    isStreaming.value = false
    sources.value = []
    currentQuestion.value = ''
    typewriterQueue = []
    isTyping = false
    isCancelled = false
  }

  const addToTypewriterQueue = (text) => {
    typewriterQueue.push(...text.split(''))
    if (!isTyping) processTypewriterQueue()
  }

  const processTypewriterQueue = () => {
    if (typewriterQueue.length === 0 || isCancelled) {
      isTyping = false
      if (typewriterQueue.length === 0) {
        isStreaming.value = false
      }
      return
    }

    isTyping = true
    response.value += typewriterQueue.splice(0, CHARS_PER_FRAME).join('')
    requestAnimationFrame(processTypewriterQueue)
  }

  const sendMessage = async (content) => {
    if (!content.trim()) return

    // Reset state but keep current question
    response.value = ''
    error.value = null
    sources.value = []
    typewriterQueue = []
    isTyping = false
    isCancelled = false

    currentQuestion.value = content
    isLoading.value = true

    try {
      // Get all store gemini names
      const storesStore = useStoresStore()
      const storeNames = storesStore.stores
        .filter((s) => s.geminiName)
        .map((s) => s.geminiName)

      if (storeNames.length === 0) {
        throw new Error('No stores available. Please add documents first.')
      }

      // Query stores via Tauri command
      const result = await invoke('query_stores', {
        storeNames,
        query: content
      })

      // Start typewriter effect
      isStreaming.value = true
      addToTypewriterQueue(result.content)

      // Set sources
      if (result.sources?.length) {
        sources.value = result.sources
      }
    } catch (err) {
      if (isCancelled) return
      error.value = err.message || err || 'Failed to send message'
    } finally {
      isLoading.value = false
    }
  }

  const abort = () => {
    isCancelled = true
    typewriterQueue = []
    isTyping = false
    isLoading.value = false
    isStreaming.value = false
  }

  const retrySend = () => {
    if (currentQuestion.value) {
      sendMessage(currentQuestion.value)
    }
  }

  // Suggest questions state and actions
  const suggestQuestions = ref([])
  const isSuggestLoading = ref(false)
  const suggestError = ref(null)
  let suggestCancelled = false

  const fetchSuggestQuestions = async (storeNames, locale) => {
    suggestCancelled = false

    if (!storeNames?.length) {
      suggestQuestions.value = []
      return
    }

    isSuggestLoading.value = true
    suggestError.value = null

    try {
      const result = await invoke('suggest_questions', {
        storeNames,
        locale: locale || 'en'
      })

      if (suggestCancelled) return
      suggestQuestions.value = Array.isArray(result) ? result : []
    } catch (err) {
      if (suggestCancelled) return
      suggestError.value = err.message || err || 'Failed to fetch suggest questions'
      suggestQuestions.value = []
    } finally {
      isSuggestLoading.value = false
    }
  }

  const clearSuggestQuestions = () => {
    suggestCancelled = true
    suggestQuestions.value = []
    suggestError.value = null
  }

  return {
    // State
    isLoading,
    response,
    error,
    isStreaming,
    sources,
    currentQuestion,
    suggestQuestions,
    isSuggestLoading,
    suggestError,
    // Actions
    sendMessage,
    reset,
    abort,
    retrySend,
    fetchSuggestQuestions,
    clearSuggestQuestions
  }
})
