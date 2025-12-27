<template>
  <SearchLayout :placeholder="$t('stores.enterName')" :prevent-close="true" @enter="handleEnter"
    @navigate-up="navigateUp" @navigate-down="navigateDown" @keydown="handleKeydown" @escape="handleEscape"
    ref="layoutRef">

    <template #search-icon>
      <div
        class="px-2 py-0.5 bg-indigo-100 text-indigo-500 text-xs font-medium rounded-md whitespace-nowrap dark:bg-indigo-900 dark:text-indigo-300">
        /new_store
      </div>
    </template>

    <template #header-actions>
      <div class="flex items-center gap-1.5">
        <ActionButton @click="submitAndNavigate" :label="$t('common.create')" shortcut="⌘N" variant="primary" />
        <ActionButton @click="handleEscape" :label="$t('common.cancel')" shortcut="ESC" />
      </div>
    </template>

    <!-- Error Messages -->
    <div v-if="nameError || filesError" class="flex flex-col gap-1 mb-2">
      <div v-if="nameError"
        class="flex items-center gap-3 px-3 py-2.5 rounded-lg bg-red-50 dark:bg-red-900/20">
        <TriangleAlertIcon class="shrink-0 size-5 text-red-600 dark:text-red-400" />
        <div class="text-sm text-red-600 dark:text-red-400">{{ nameError }}</div>
      </div>
      <div v-if="filesError"
        class="flex items-center gap-3 px-3 py-2.5 rounded-lg bg-red-50 dark:bg-red-900/20">
        <TriangleAlertIcon class="shrink-0 size-5 text-red-600 dark:text-red-400" />
        <div class="text-sm text-red-600 dark:text-red-400">{{ filesError }}</div>
      </div>
    </div>

    <!-- Files List -->
    <div class="mb-4 last:mb-0">
      <div class="flex flex-col gap-0.5">
        <!-- Select Local Folder Button -->
        <div :data-result-index="0" @click.stop.prevent="handleOpenLocalFolder" @mouseenter="selectedIndex = 0" :class="[
          'group flex items-center gap-3 px-3 py-2.5 rounded-lg cursor-pointer transition-all duration-150 relative',
          selectedIndex === 0
            ? 'bg-gray-100 before:absolute before:left-0 before:top-1/2 before:-translate-y-1/2 before:w-0.5 before:h-6 before:bg-indigo-600 before:rounded-r-sm dark:bg-gray-600'
            : 'hover:bg-gray-50 dark:hover:bg-gray-700'
        ]" data-no-drag>
          <FolderOpenIcon class="shrink-0 size-5 text-gray-500 dark:text-gray-400" />
          <div class="flex-1 min-w-0">
            <div class="text-sm font-medium text-gray-900 truncate dark:text-gray-100">
              {{ selectedDirectoryPath ? selectedDirectoryPath : $t('files.selectFolder') }}
            </div>
            <div v-if="selectedDirectoryPath" class="text-xs text-gray-500 dark:text-gray-400 mt-0.5">
              {{ $t('files.clickToChange') }}
            </div>
          </div>
          <kbd
            class="px-2 py-1 text-xs font-sans font-medium text-gray-500 bg-white border border-gray-200 rounded-lg dark:bg-gray-700 dark:border-gray-600 dark:text-gray-400">
            ⌘O
          </kbd>
        </div>

        <!-- Selected Files -->
        <div v-for="(file, index) in selectedFiles" :key="file.path" :data-result-index="index + 1"
          @mouseenter="selectedIndex = index + 1" :class="[
            'group flex items-center gap-3 px-3 py-2.5 rounded-lg cursor-pointer transition-all duration-150 relative',
            selectedIndex === index + 1
              ? 'bg-gray-100 before:absolute before:left-0 before:top-1/2 before:-translate-y-1/2 before:w-0.5 before:h-6 before:bg-indigo-600 before:rounded-r-sm dark:bg-gray-600'
              : 'hover:bg-gray-50 dark:hover:bg-gray-700'
          ]">

          <!-- File Icon -->
          <FileIcon class="shrink-0 size-5 text-gray-500 dark:text-gray-400" />

          <div class="flex-1 min-w-0">
            <div class="text-sm font-medium text-gray-900 truncate dark:text-gray-100">
              <span v-if="file.isSubdirectoryFile" class="text-xs text-gray-500 dark:text-gray-400">
                {{ file.subdirectoryName }}/
              </span>
              {{ file.displayName }}
            </div>
            <div class="flex items-center gap-2 text-gray-400 mt-0.5 text-xs">
              <span
                class="text-[10px] text-medium text-gray-500 bg-gray-200 px-1 py-0.5 rounded-md dark:bg-gray-700 dark:text-gray-400">{{
                  getFriendlyFileType(file.contentType) }}</span>
              <span>•</span>
              <span>{{ file.sizeText }}</span>
            </div>
          </div>

          <!-- Delete Button -->
          <ActionButton v-show="selectedIndex === index + 1" @click.stop="handleRemoveFile(index)"
            :label="$t('common.delete')" shortcut="⌘D" variant="danger" />
        </div>
      </div>
    </div>

    <!-- Empty state -->
    <div v-if="selectedFiles.length === 0" class="flex flex-col items-center justify-center px-6 py-12 text-center">
      <FilePlusCornerIcon class="size-12 text-gray-300 mb-3 dark:text-gray-600" />
      <div class="text-base font-semibold text-gray-700 mb-1 dark:text-gray-300">{{ $t('files.noFilesSelected') }}</div>
      <div class="text-sm text-gray-400">{{ $t('files.selectFolderHint') }}</div>
      <div class="text-xs text-gray-400 mt-2">{{ $t('files.subdirectoryInfo') }}</div>
      <div class="mt-4 text-xs text-gray-400">{{ $t('hints.cancelHint') }}</div>
    </div>
  </SearchLayout>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import SearchLayout from '@/views/layout/application.vue'
import ActionButton from '@/components/ActionButton.vue'
import { FolderOpenIcon, FileIcon, FilePlusCornerIcon, TriangleAlertIcon } from '@/utils/icons.js'
import { SHORTCUTS } from '@/utils/constants.js'
import { getFriendlyFileType } from '@/utils/helpers.js'
import { useAppStore } from '@/stores/app'
import { useGlobalKeyboard } from '@/composables/useGlobalKeyboard'
import { useListNavigation } from '@/composables/useListNavigation'
import { useFileSelection } from '@/composables/useFileSelection'
import { useStoreForm } from '@/composables/useStoreForm'

const appStore = useAppStore()
const layoutRef = ref(null)

// File selection
const {
  selectedFiles,
  isSelectingFile,
  selectedDirectoryPath,
  selectedDirectoryFullPath,
  openFileSelector,
  removeFile,
  clearSelection
} = useFileSelection()

// Form validation and submission
const {
  nameError,
  filesError,
  submitAndNavigate,
  cancel
} = useStoreForm({
  selectedFiles,
  selectedDirectoryFullPath,
  clearSelection
})

// List navigation (1 folder button + files)
const itemCount = computed(() => 1 + selectedFiles.value.length)

const {
  selectedIndex,
  navigateUp,
  navigateDown
} = useListNavigation({
  itemCount,
  layoutRef
})

// Get input ref from layout for global keyboard
const getInputRef = () => layoutRef.value?.searchInput

// Setup global keyboard shortcuts (disabled when file dialog is open)
const {
  onNavigateUp,
  onNavigateDown,
  onEnter,
  onEscape,
  onCreateNew,
  onOpenFile,
  onDelete
} = useGlobalKeyboard({
  inputRef: computed(() => getInputRef()),
  isDisabled: isSelectingFile
})

// Clear search query on mount
onMounted(() => {
  appStore.clearSearchQuery()
})

// Event handlers
const handleOpenLocalFolder = async () => {
  isSelectingFile.value = true
  await invoke('set_prevent_auto_hide', { prevent: true })
  await openFileSelector()
}

const handleRemoveFile = (index) => {
  removeFile(index)
  // Adjust selected index if needed (files start at index 1)
  if (selectedIndex.value > selectedFiles.value.length) {
    selectedIndex.value = Math.max(0, selectedFiles.value.length)
  }
}

const handleDeleteSelected = () => {
  // Files start at index 1 (after folder button)
  if (selectedIndex.value >= 1 && selectedIndex.value < itemCount.value) {
    handleRemoveFile(selectedIndex.value - 1)
  }
}

const handleEnter = () => {
  // If "Select Local Folder" button is selected
  if (selectedIndex.value === 0) {
    handleOpenLocalFolder()
    return
  }
  // For other items, Enter key does nothing (use CMD+N to create store)
}

const handleEscape = () => {
  if (!isSelectingFile.value) {
    selectedIndex.value = 0
    cancel()
  }
}

const handleKeydown = async (e) => {
  // CMD+O to open file selector
  if (e.key === SHORTCUTS.OPEN_FILE && e.metaKey) {
    e.preventDefault()
    await handleOpenLocalFolder()
  }
  // CMD+N to create store
  else if (e.key === SHORTCUTS.CREATE_NEW && e.metaKey) {
    e.preventDefault()
    submitAndNavigate()
  }
  // CMD+D to delete selected file
  else if (e.key === SHORTCUTS.DELETE && e.metaKey) {
    e.preventDefault()
    handleDeleteSelected()
  }
  // ESC to go back to list (only when not selecting file)
  else if (e.key === SHORTCUTS.ESCAPE && !isSelectingFile.value) {
    e.preventDefault()
    handleEscape()
  }
}

// Register global keyboard handlers
onNavigateUp(navigateUp)
onNavigateDown(navigateDown)
onEnter(handleEnter)
onEscape(handleEscape)
onCreateNew(submitAndNavigate)
onOpenFile(handleOpenLocalFolder)
onDelete(handleDeleteSelected)
</script>
