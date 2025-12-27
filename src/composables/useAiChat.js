import { ref, computed, watch, nextTick, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { marked } from 'marked'
import DOMPurify from 'dompurify'
import { useAppStore } from '@/stores/app'
import { useChatStore } from '@/stores/chat'
import { useStoresStore } from '@/stores/stores'
import { useLocaleStore } from '@/stores/locale'

// Configure marked options
marked.setOptions({
  breaks: true,
  gfm: true
})

/**
 * AI Chat composable
 * Handles chat messaging, suggest questions, source documents, and scrolling
 *
 * @param {Object} options - Configuration options
 * @param {import('vue').Ref} options.layoutRef - Reference to the layout component
 * @param {import('vue').Ref} options.scrollAnchorRef - Reference to the scroll anchor element
 * @returns {Object} Chat state and methods
 */
export function useAiChat(options = {}) {
  const { layoutRef, scrollAnchorRef } = options

  const appStore = useAppStore()
  const chatStore = useChatStore()
  const storesStore = useStoresStore()
  const localeStore = useLocaleStore()

  // State
  const sourceDocuments = ref([])
  const selectedSuggestIndex = ref(0)

  // Computed
  const renderedContent = computed(() => {
    if (!chatStore.response) return ''
    return DOMPurify.sanitize(marked.parse(chatStore.response))
  })

  // Check if we should show suggest questions (no conversation started yet)
  const showSuggestQuestions = computed(() => {
    return (
      !chatStore.currentQuestion &&
      !chatStore.response &&
      !chatStore.isLoading &&
      !chatStore.error &&
      chatStore.suggestQuestions.length > 0
    )
  })

  // Scroll to bottom of chat
  const scrollToBottom = () => {
    nextTick(() => {
      scrollAnchorRef.value?.scrollIntoView({ behavior: 'smooth', block: 'end' })
    })
  }

  // Fetch source documents by display names (from API groundingMetadata)
  const fetchSourceDocuments = async (documentNames) => {
    if (!documentNames?.length) {
      sourceDocuments.value = []
      return
    }
    sourceDocuments.value = await invoke('get_documents_by_uids', { documentUids: documentNames }).catch(() => [])
  }

  // Open document with default application
  const openDocument = async (doc) => {
    try {
      // Use doc.name (display name) since API returns file names in groundingMetadata
      if (doc.name) {
        await invoke('open_document_file', { documentUid: doc.name })
      }
    } catch (error) {
      console.error('Failed to open document:', error)
    }
  }

  // Send chat message
  const sendMessage = async (content) => {
    appStore.clearSearchQuery()
    await chatStore.sendMessage(content)
  }

  // Select a suggest question and send it
  const selectSuggestQuestion = (question) => {
    sendMessage(question)
  }

  // Reset chat and exit
  const exitChat = () => {
    chatStore.reset()
    chatStore.clearSuggestQuestions()
    sourceDocuments.value = []
    appStore.exitAiChat()
  }

  // Suggest questions navigation
  const navigateSuggestUp = () => {
    if (!showSuggestQuestions.value) return
    const total = chatStore.suggestQuestions.length
    if (selectedSuggestIndex.value > 0) {
      selectedSuggestIndex.value--
    } else {
      selectedSuggestIndex.value = total - 1
    }
  }

  const navigateSuggestDown = () => {
    if (!showSuggestQuestions.value) return
    const total = chatStore.suggestQuestions.length
    if (selectedSuggestIndex.value < total - 1) {
      selectedSuggestIndex.value++
    } else {
      selectedSuggestIndex.value = 0
    }
  }

  // Handle enter on suggest questions
  const handleSuggestEnter = () => {
    if (!showSuggestQuestions.value) return
    const questions = chatStore.suggestQuestions
    if (questions.length > 0 && selectedSuggestIndex.value >= 0 && selectedSuggestIndex.value < questions.length) {
      selectSuggestQuestion(questions[selectedSuggestIndex.value])
    }
  }

  // Load suggest questions on mount
  onMounted(async () => {
    chatStore.reset()
    chatStore.clearSuggestQuestions()

    // Ensure stores are loaded
    if (storesStore.stores.length === 0) {
      await storesStore.loadStores()
    }

    // Get all store gemini names from stores
    const storeGeminiNames = storesStore.stores.filter((s) => s.geminiName).map((s) => s.geminiName)

    if (storeGeminiNames.length > 0) {
      await chatStore.fetchSuggestQuestions(storeGeminiNames, localeStore.currentLocale)
    }
  })

  // Watchers for scrolling behavior
  watch(() => chatStore.response, scrollToBottom)

  watch(
    () => chatStore.isLoading,
    (v) => v && scrollToBottom()
  )

  watch(
    () => chatStore.isStreaming,
    (isStreaming) => {
      if (isStreaming) {
        scrollToBottom()
      } else {
        // When streaming ends, scroll to bottom and focus input
        scrollToBottom()
        nextTick(() => {
          layoutRef.value?.focusInput()
        })
      }
    }
  )

  watch(
    () => chatStore.sources,
    (sources) => {
      if (sources?.length) fetchSourceDocuments(sources)
    },
    { deep: true }
  )

  // Scroll when source documents are loaded to ensure they are visible
  watch(
    sourceDocuments,
    () => {
      nextTick(scrollToBottom)
    },
    { deep: true }
  )

  return {
    // State
    sourceDocuments,
    selectedSuggestIndex,
    // Computed
    renderedContent,
    showSuggestQuestions,
    // Store references (for template bindings)
    chatStore,
    // Methods
    sendMessage,
    selectSuggestQuestion,
    exitChat,
    openDocument,
    navigateSuggestUp,
    navigateSuggestDown,
    handleSuggestEnter
  }
}
