import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import { readFile, stat, readDir } from '@tauri-apps/plugin-fs'
import { FILE_LIMITS } from '@/utils/constants.js'
import { formatFileSize, getContentType, calculateFileHash } from '@/utils/helpers.js'

/**
 * File selection composable
 * Handles local folder file selection
 *
 * @returns {Object} File selection state and methods
 */
export function useFileSelection() {
  // State
  const selectedFiles = ref([])
  const isSelectingFile = ref(false)
  const selectedDirectoryPath = ref('')
  const selectedDirectoryFullPath = ref('')

  // Helper function to get directory name for display
  const getDirectoryDisplayName = (fullPath) => {
    if (!fullPath) return ''
    const parts = fullPath.split('/')
    return parts[parts.length - 1] || fullPath
  }

  // Helper function to process a file entry
  const processFileEntry = async (dirPath, entry, parentDirSelected = true) => {
    const fullPath = `${dirPath}/${entry.name}`
    let fileSize = entry.size

    // Try to get file size with stat if not available from readDir
    if (fileSize === undefined) {
      try {
        const fileInfo = await stat(fullPath)
        fileSize = fileInfo.size
      } catch {
        return null // Skip files we can't access
      }
    }

    // Skip files with no size info
    if (!fileSize || fileSize <= 0) {
      return null
    }

    const contentType = getContentType(entry.name)
    let hash = null

    // Only calculate hash for small files in parent directory
    if (fileSize < FILE_LIMITS.MAX_HASH_SIZE && parentDirSelected) {
      try {
        hash = await calculateFileHash(fullPath, fileSize, readFile)
      } catch {
        // Skip hash calculation if file is not accessible
      }
    }

    return {
      name: entry.name,
      displayName: entry.name,
      subdirectoryName: '',
      path: fullPath,
      size: fileSize,
      sizeText: formatFileSize(fileSize),
      isSubdirectoryFile: !parentDirSelected,
      contentType,
      hash
    }
  }

  // Process files in a subdirectory
  const processSubdirectory = async (parentDir, dirEntry) => {
    const subDirPath = `${parentDir}/${dirEntry.name}`

    try {
      const subEntries = await readDir(subDirPath)

      for (const subEntry of subEntries) {
        // Skip hidden files and nested directories
        if (subEntry.name.startsWith('.') || subEntry.isDirectory) continue

        const file = await processFileEntry(subDirPath, subEntry, false)
        if (file) {
          file.name = `${dirEntry.name}/${subEntry.name}`
          file.subdirectoryName = dirEntry.name
          file.displayName = subEntry.name
          selectedFiles.value.push(file)
        }
      }
    } catch {
      // Silently skip subdirectories we can't read
    }
  }

  // Process the selected directory and its subdirectories
  const processSelectedDirectory = async (selectedDir) => {
    selectedFiles.value = []
    selectedDirectoryPath.value = getDirectoryDisplayName(selectedDir)
    selectedDirectoryFullPath.value = selectedDir

    try {
      const entries = await readDir(selectedDir)

      for (const entry of entries) {
        // Skip hidden files/folders
        if (entry.name.startsWith('.')) continue

        if (entry.isDirectory) {
          // Process subdirectory (1 level deep)
          await processSubdirectory(selectedDir, entry)
        } else {
          // Process file in main directory
          const file = await processFileEntry(selectedDir, entry, true)
          if (file) {
            selectedFiles.value.push(file)
          }
        }
      }

      // Sort files alphabetically
      selectedFiles.value.sort((a, b) => a.name.localeCompare(b.name))
    } catch {
      // Failed to read directory - likely a permissions issue
    }
  }

  // Open folder selector and read all files in the directory
  const openFileSelector = async () => {
    isSelectingFile.value = true

    try {
      // Tell backend to prevent auto-hide
      await invoke('set_prevent_auto_hide', { prevent: true })

      // Select directory
      const selectedDir = await open({
        multiple: false,
        directory: true
      })

      // Reset flag and backend state after dialog closes
      setTimeout(async () => {
        isSelectingFile.value = false
        try {
          await invoke('set_prevent_auto_hide', { prevent: false })
        } catch {
          // Ignore error if window is already closed
        }
      }, FILE_LIMITS.DIALOG_CLOSE_DELAY)

      if (selectedDir) {
        await processSelectedDirectory(selectedDir)
      }
    } catch {
      // User cancelled or error occurred
      isSelectingFile.value = false
      try {
        await invoke('set_prevent_auto_hide', { prevent: false })
      } catch {
        // Ignore error if window is already closed
      }
    }
  }

  // Remove file from list
  const removeFile = (index) => {
    selectedFiles.value.splice(index, 1)
  }

  // Clear all selections
  const clearSelection = () => {
    selectedFiles.value = []
    selectedDirectoryPath.value = ''
    selectedDirectoryFullPath.value = ''
  }

  return {
    // State
    selectedFiles,
    isSelectingFile,
    selectedDirectoryPath,
    selectedDirectoryFullPath,
    // Methods
    openFileSelector,
    removeFile,
    clearSelection
  }
}
