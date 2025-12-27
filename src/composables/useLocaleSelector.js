import { computed } from 'vue'
import { useRouter } from 'vue-router'
import { highlightSearchMatch } from '@/utils/helpers.js'
import { useAppStore } from '@/stores/app'
import { useLocaleStore } from '@/stores/locale'

/**
 * Locale selector composable
 * Handles locale filtering, selection, and navigation
 *
 * @returns {Object} Locale selector state and methods
 */
export function useLocaleSelector() {
  const router = useRouter()
  const appStore = useAppStore()
  const localeStore = useLocaleStore()

  // Filtered locales based on search query
  const filteredLocales = computed(() => {
    if (!appStore.searchQuery) return localeStore.supportedLocales

    const query = appStore.searchQuery.toLowerCase()
    return localeStore.supportedLocales.filter(item =>
      item.name.toLowerCase().includes(query) ||
      item.nativeName.toLowerCase().includes(query) ||
      item.code.toLowerCase().includes(query)
    )
  })

  // Display items (alias for filteredLocales)
  const displayItems = computed(() => filteredLocales.value)

  // Current locale code
  const currentLocale = computed(() => localeStore.currentLocale)

  // Select a locale and navigate back
  const selectLocale = async (item) => {
    await localeStore.setLocale(item.code)
    appStore.clearSearchQuery()
    router.push('/')
  }

  // Go back to home
  const goBack = () => {
    appStore.clearSearchQuery()
    router.push('/')
  }

  // Highlight search match in text (XSS-safe)
  const highlightMatch = (text) => highlightSearchMatch(text, appStore.searchQuery)

  return {
    // State
    displayItems,
    currentLocale,
    searchQuery: computed(() => appStore.searchQuery),
    // Methods
    selectLocale,
    goBack,
    highlightMatch
  }
}
