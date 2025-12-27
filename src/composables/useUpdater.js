import { ref } from 'vue'
import { check } from '@tauri-apps/plugin-updater'
import { relaunch } from '@tauri-apps/plugin-process'

/**
 * Updater composable for checking and installing app updates
 *
 * @returns {Object} Updater state and methods
 */
export function useUpdater() {
  const isChecking = ref(false)
  const isDownloading = ref(false)
  const downloadProgress = ref(0)
  const updateAvailable = ref(false)
  const updateInfo = ref(null)
  const error = ref(null)

  /**
   * Check for available updates
   * @param {Object} options - Options
   * @param {boolean} options.silent - If true, don't show "no update" message
   * @returns {Promise<Object|null>} Update info or null
   */
  const checkForUpdate = async (options = {}) => {
    const { silent = false } = options

    if (isChecking.value) return null

    isChecking.value = true
    error.value = null

    try {
      const update = await check()

      if (update) {
        updateAvailable.value = true
        updateInfo.value = {
          version: update.version,
          currentVersion: update.currentVersion,
          body: update.body,
          date: update.date
        }
        return { update, info: updateInfo.value }
      }

      updateAvailable.value = false
      updateInfo.value = null
      return null
    } catch (err) {
      console.error('Failed to check for updates:', err)
      error.value = err.message || 'Failed to check for updates'
      return null
    } finally {
      isChecking.value = false
    }
  }

  /**
   * Download and install the update
   * @param {Object} update - Update object from check()
   */
  const downloadAndInstall = async (update) => {
    if (isDownloading.value || !update) return

    isDownloading.value = true
    downloadProgress.value = 0
    error.value = null

    try {
      await update.downloadAndInstall((event) => {
        if (event.event === 'Started') {
          downloadProgress.value = 0
        } else if (event.event === 'Progress') {
          const { chunkLength, contentLength } = event.data
          if (contentLength) {
            downloadProgress.value = Math.round((chunkLength / contentLength) * 100)
          }
        } else if (event.event === 'Finished') {
          downloadProgress.value = 100
        }
      })

      // Relaunch the app after installation
      await relaunch()
    } catch (err) {
      console.error('Failed to download/install update:', err)
      error.value = err.message || 'Failed to install update'
    } finally {
      isDownloading.value = false
    }
  }

  /**
   * Check for updates on app startup (silent mode)
   * Shows notification only if update is available
   */
  const checkOnStartup = async () => {
    // Wait a bit before checking to not block app startup
    await new Promise(resolve => setTimeout(resolve, 3000))
    return await checkForUpdate({ silent: true })
  }

  return {
    // State
    isChecking,
    isDownloading,
    downloadProgress,
    updateAvailable,
    updateInfo,
    error,
    // Methods
    checkForUpdate,
    downloadAndInstall,
    checkOnStartup
  }
}
