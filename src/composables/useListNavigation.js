import { ref, watch } from 'vue'

/**
 * List navigation composable
 * Handles keyboard navigation (up/down) and scroll behavior for list views
 *
 * @param {Object} options - Configuration options
 * @param {import('vue').ComputedRef<number>} options.itemCount - Total number of items in the list
 * @param {import('vue').Ref} options.layoutRef - Reference to the layout component with scrollToSelectedItem method
 * @param {import('vue').Ref|import('vue').ComputedRef} [options.searchQuery] - Search query to reset selection on change
 * @returns {Object} Navigation state and methods
 */
export function useListNavigation(options = {}) {
  const { itemCount, layoutRef, searchQuery = null } = options

  const selectedIndex = ref(0)

  const navigateUp = () => {
    const total = itemCount.value
    if (total === 0) return

    if (selectedIndex.value > 0) {
      selectedIndex.value--
    } else {
      selectedIndex.value = total - 1
    }
    scrollToSelected()
  }

  const navigateDown = () => {
    const total = itemCount.value
    if (total === 0) return

    if (selectedIndex.value < total - 1) {
      selectedIndex.value++
    } else {
      selectedIndex.value = 0
    }
    scrollToSelected()
  }

  const scrollToSelected = () => {
    if (layoutRef.value) {
      layoutRef.value.scrollToSelectedItem(selectedIndex.value)
    }
  }

  const resetSelection = () => {
    selectedIndex.value = 0
  }

  /**
   * Adjust selection when items are removed
   * Call this after deleting an item
   */
  const adjustSelectionAfterRemoval = () => {
    const total = itemCount.value
    if (selectedIndex.value >= total) {
      selectedIndex.value = Math.max(0, total - 1)
    }
  }

  // Reset selection when search query changes
  if (searchQuery) {
    watch(searchQuery, () => {
      resetSelection()
    })
  }

  return {
    selectedIndex,
    navigateUp,
    navigateDown,
    scrollToSelected,
    resetSelection,
    adjustSelectionAfterRemoval
  }
}
