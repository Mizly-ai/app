import { computed, markRaw, ref } from 'vue'
import { useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { ThemeIcon, GlobeIcon, KeyIcon, DownloadIcon } from '@/utils/icons.js'
import { highlightSearchMatch } from '@/utils/helpers.js'
import { useAppStore } from '@/stores/app'
import { useUpdater } from '@/composables/useUpdater'

/**
 * Home settings composable
 * Handles settings items and search filtering
 *
 * @returns {Object} Settings state and methods
 */
export function useHomeSettings() {
  const { t } = useI18n()
  const router = useRouter()
  const appStore = useAppStore()
  const { checkForUpdate, downloadAndInstall, isChecking, updateInfo } = useUpdater()

  // Update status message
  const updateStatus = ref('')

  // Settings items
  const settingsItems = computed(() => [
    { id: 1, title: t('settings.changeTheme'), icon: markRaw(ThemeIcon), action: 'changeTheme' },
    { id: 2, title: t('settings.language'), icon: markRaw(GlobeIcon), action: 'changeLanguage' },
    { id: 3, title: t('settings.apiKey'), icon: markRaw(KeyIcon), action: 'configureApiKey' },
    { id: 4, title: t('settings.checkUpdate'), icon: markRaw(DownloadIcon), action: 'checkUpdate' }
  ])

  // All items (no user profile items anymore)
  const allItems = computed(() => [...settingsItems.value])

  // Filtered results based on search query
  const filteredResults = computed(() => {
    if (!appStore.searchQuery) return []

    const query = appStore.searchQuery.toLowerCase()
    return allItems.value.filter(item =>
      item.title?.toLowerCase().includes(query) ||
      item.name?.toLowerCase().includes(query)
    )
  })

  // Change theme (toggle dark mode)
  const changeTheme = () => {
    document.documentElement.classList.toggle('dark')
  }

  // Check for updates
  const handleCheckUpdate = async () => {
    updateStatus.value = t('updater.checking')

    const result = await checkForUpdate()

    if (result) {
      updateStatus.value = t('updater.updateAvailable', { version: result.info.version })
      // Auto download and install
      await downloadAndInstall(result.update)
    } else {
      updateStatus.value = t('updater.noUpdate')
      // Clear status after 3 seconds
      setTimeout(() => {
        updateStatus.value = ''
      }, 3000)
    }
  }

  // Select a result item
  const selectResult = async (result) => {
    if (result.action === 'changeTheme') {
      changeTheme()
      return
    }
    if (result.action === 'changeLanguage') {
      router.push('/settings/locales')
      return
    }
    if (result.action === 'configureApiKey') {
      router.push('/settings/api-key')
      return
    }
    if (result.action === 'checkUpdate') {
      handleCheckUpdate()
      return
    }
    appStore.clearSearchQuery()
  }

  // Highlight search match in text (XSS-safe)
  const highlightMatch = (text) => highlightSearchMatch(text, appStore.searchQuery)

  // Tab navigation helpers
  const switchToPrevTab = () => appStore.switchTab('prev')
  const switchToNextTab = () => appStore.switchTab('next')
  const enterAiChat = () => {
    appStore.enterAiChat()
    appStore.clearSearchQuery()
  }

  return {
    // State
    settingsItems,
    allItems,
    filteredResults,
    searchQuery: computed(() => appStore.searchQuery),
    updateStatus,
    isChecking,
    // Methods
    selectResult,
    changeTheme,
    highlightMatch,
    switchToPrevTab,
    switchToNextTab,
    enterAiChat
  }
}
