import { getCurrentWindow } from '@tauri-apps/api/window'

// Interactive elements that should not trigger window drag
const INTERACTIVE_TAGS = ['INPUT', 'BUTTON', 'A', 'SELECT', 'TEXTAREA', 'LABEL']
const INTERACTIVE_SELECTORS = 'button, a, input, select, textarea, label, [role="button"], [tabindex], .interactive, [data-no-drag]'

/**
 * Composable for handling window drag functionality
 * @returns {Object} Window drag utilities
 */
export function useWindowDrag() {
  /**
   * Check if an element is interactive (should not trigger drag)
   * @param {HTMLElement} element - The element to check
   * @returns {boolean} True if the element is interactive
   */
  const isInteractiveElement = (element) => {
    const tagName = element.tagName?.toUpperCase()
    if (INTERACTIVE_TAGS.includes(tagName)) return true
    if (element.closest(INTERACTIVE_SELECTORS)) return true
    return false
  }

  /**
   * Handle mousedown event to start window dragging
   * @param {MouseEvent} event - The mousedown event
   */
  const handleDragStart = async (event) => {
    // Only left mouse button
    if (event.button !== 0) return

    const target = event.target

    // Skip if clicking on interactive elements
    if (isInteractiveElement(target)) return

    try {
      const window = getCurrentWindow()
      await window.startDragging()
    } catch (e) {
      // Ignore errors in browser mode
    }
  }

  return {
    handleDragStart,
    isInteractiveElement,
    INTERACTIVE_TAGS,
    INTERACTIVE_SELECTORS
  }
}
