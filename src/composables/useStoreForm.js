import { ref, watch } from 'vue'
import { useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { VALIDATION } from '@/utils/constants.js'
import { validateStoreName } from '@/utils/helpers.js'
import { useAppStore } from '@/stores/app'
import { useStoresStore } from '@/stores/stores'

/**
 * Store form composable
 * Handles form validation and submission for creating new stores
 *
 * @param {Object} options - Configuration options
 * @param {import('vue').Ref<Array>} options.selectedFiles - Selected files ref
 * @param {import('vue').Ref<string>} options.selectedDirectoryFullPath - Full path of selected directory
 * @param {Function} options.clearSelection - Function to clear file selection
 * @returns {Object} Form state and methods
 */
export function useStoreForm(options = {}) {
  const {
    selectedFiles,
    selectedDirectoryFullPath,
    clearSelection
  } = options

  const { t } = useI18n()
  const router = useRouter()
  const appStore = useAppStore()
  const storesStore = useStoresStore()

  // Validation errors
  const nameError = ref('')
  const filesError = ref('')

  // Watch for name input changes to clear error
  watch(() => appStore.searchQuery, (newVal) => {
    if (newVal.trim().length > 0 && nameError.value) {
      nameError.value = ''
    }
  })

  // Watch for file selection changes to clear error
  watch(selectedFiles, (newVal) => {
    if (newVal.length > 0 && filesError.value) {
      filesError.value = ''
    }
  }, { deep: true })

  // Validate form
  const validate = () => {
    const name = appStore.searchQuery.trim()

    // Reset errors
    nameError.value = ''
    filesError.value = ''

    let isValid = true

    // Validate store name
    if (!validateStoreName(name, VALIDATION.MIN_NAME_LENGTH)) {
      nameError.value = t('validation.storeNameRequired')
      isValid = false
    }

    // Validate files selection
    if (selectedFiles.value.length === 0) {
      filesError.value = t('validation.filesRequired')
      isValid = false
    }

    return isValid
  }

  // Handle store creation
  const createStore = async () => {
    if (!validate()) {
      return false
    }

    const name = appStore.searchQuery.trim()

    // Prepare store data with files
    const storeData = {
      name,
      directoryPath: selectedDirectoryFullPath.value,
      files: selectedFiles.value.map(file => ({
        filename: file.name.split('/').pop(), // Get actual filename without subdirectory path
        displayName: file.name, // Keep full relative path for display
        contentType: file.contentType,
        size: file.size,
        path: file.path,
        hash: file.hash
      }))
    }

    try {
      await storesStore.addStore(storeData)
      return true
    } catch (e) {
      console.error('Failed to create store:', e)
      return false
    }
  }

  // Handle form submission and navigation
  const submitAndNavigate = async () => {
    const success = await createStore()

    // Clear and navigate back regardless of success
    clearSelection()
    appStore.clearSearchQuery()
    router.push('/stores')

    return success
  }

  // Cancel and go back
  const cancel = () => {
    clearSelection()
    appStore.clearSearchQuery()
    router.push('/stores')
  }

  return {
    // State
    nameError,
    filesError,
    // Methods
    validate,
    createStore,
    submitAndNavigate,
    cancel
  }
}
