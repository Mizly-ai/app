import { onMounted, onUnmounted, ref } from 'vue'
import { SHORTCUTS } from '@/utils/constants.js'

/**
 * Global keyboard event handler composable
 * Handles keyboard shortcuts when input is not focused
 *
 * @param {Object} options - Configuration options
 * @param {Ref<HTMLElement>} options.inputRef - Reference to the search input element
 * @param {Ref<boolean>} options.isDisabled - Whether to disable global keyboard handling
 * @returns {Object} Keyboard handler registration functions
 */
export function useGlobalKeyboard(options = {}) {
  const { inputRef = ref(null), isDisabled = ref(false) } = options

  // Callback registrations
  const callbacks = {
    navigateUp: null,
    navigateDown: null,
    navigateLeft: null,
    navigateRight: null,
    enter: null,
    escape: null,
    tab: null,
    createNew: null,
    openFile: null,
    delete: null
  }

  const isInputFocused = () => {
    const input = inputRef.value
    if (!input) return false
    return document.activeElement === input
  }

  const handleKeydown = (e) => {
    // Skip if disabled
    if (isDisabled.value) return

    // Skip if input is focused - let input's own handlers take over
    if (isInputFocused()) return

    // Handle arrow keys for navigation
    if (e.key === 'ArrowUp') {
      e.preventDefault()
      callbacks.navigateUp?.()
      return
    }

    if (e.key === 'ArrowDown') {
      e.preventDefault()
      callbacks.navigateDown?.()
      return
    }

    if (e.key === 'ArrowLeft') {
      e.preventDefault()
      callbacks.navigateLeft?.()
      return
    }

    if (e.key === 'ArrowRight') {
      e.preventDefault()
      callbacks.navigateRight?.()
      return
    }

    // Handle Enter
    if (e.key === 'Enter') {
      e.preventDefault()
      callbacks.enter?.()
      return
    }

    // Handle Escape
    if (e.key === 'Escape') {
      e.preventDefault()
      callbacks.escape?.()
      return
    }

    // Handle Tab
    if (e.key === 'Tab') {
      e.preventDefault()
      callbacks.tab?.()
      return
    }

    // Handle Command shortcuts (Meta key on Mac)
    if (e.metaKey) {
      if (e.key === SHORTCUTS.CREATE_NEW) {
        e.preventDefault()
        callbacks.createNew?.()
        return
      }

      if (e.key === SHORTCUTS.OPEN_FILE) {
        e.preventDefault()
        callbacks.openFile?.()
        return
      }

      if (e.key === SHORTCUTS.DELETE) {
        e.preventDefault()
        callbacks.delete?.()
        return
      }
    }
  }

  // Register callback functions
  const onNavigateUp = (callback) => {
    callbacks.navigateUp = callback
  }

  const onNavigateDown = (callback) => {
    callbacks.navigateDown = callback
  }

  const onNavigateLeft = (callback) => {
    callbacks.navigateLeft = callback
  }

  const onNavigateRight = (callback) => {
    callbacks.navigateRight = callback
  }

  const onEnter = (callback) => {
    callbacks.enter = callback
  }

  const onEscape = (callback) => {
    callbacks.escape = callback
  }

  const onTab = (callback) => {
    callbacks.tab = callback
  }

  const onCreateNew = (callback) => {
    callbacks.createNew = callback
  }

  const onOpenFile = (callback) => {
    callbacks.openFile = callback
  }

  const onDelete = (callback) => {
    callbacks.delete = callback
  }

  // Focus the input element
  const focusInput = () => {
    inputRef.value?.focus()
  }

  // Lifecycle hooks
  onMounted(() => {
    window.addEventListener('keydown', handleKeydown)
  })

  onUnmounted(() => {
    window.removeEventListener('keydown', handleKeydown)
  })

  return {
    onNavigateUp,
    onNavigateDown,
    onNavigateLeft,
    onNavigateRight,
    onEnter,
    onEscape,
    onTab,
    onCreateNew,
    onOpenFile,
    onDelete,
    focusInput
  }
}
