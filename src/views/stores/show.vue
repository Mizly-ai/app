<template>
  <SearchLayout :placeholder="$t('stores.searchInStore')" :prevent-close="true" @escape="handleEscape"
    @keydown="handleKeydown" @navigate-up="navigateUp" @navigate-down="navigateDown" ref="layoutRef">

    <template #search-icon>
      <div
        class="px-2 py-0.5 bg-indigo-100 text-indigo-500 text-xs font-medium rounded-md whitespace-nowrap dark:bg-indigo-900 dark:text-indigo-300">
        {{ selectedStore?.title || $t('stores.title') }}
      </div>
    </template>

    <template #header-actions>
      <div class="flex items-center gap-1.5">
        <ActionButton @click="handleEscape" :label="$t('common.back')" shortcut="ESC" />
      </div>
    </template>

    <!-- Files List -->
    <div v-if="selectedStore" class="mb-4 last:mb-0">
      <div class="flex flex-col gap-0.5">
        <!-- Directory Info - Always at top -->
        <div :data-result-index="0"
          @mouseenter="selectedIndex = 0"
          :class="[
            'group flex items-center gap-3 px-3 py-2.5 rounded-lg cursor-pointer transition-all duration-150 relative',
            selectedIndex === 0
              ? 'bg-gray-100 before:absolute before:left-0 before:top-1/2 before:-translate-y-1/2 before:w-0.5 before:h-6 before:bg-indigo-600 before:rounded-r-sm dark:bg-gray-600'
              : 'hover:bg-gray-50 dark:hover:bg-gray-700'
          ]">
          <FolderIcon class="shrink-0 size-5 text-gray-500 dark:text-gray-400" />
          <div class="flex-1 min-w-0">
            <div class="text-sm font-medium text-gray-900 truncate dark:text-gray-100">
              {{ selectedStore.title }}
            </div>
            <div class="text-xs text-gray-500 dark:text-gray-400 mt-0.5 truncate">
              {{ selectedStore.directoryPath }}
            </div>
          </div>
          <ActionButton v-show="selectedIndex === 0" @click.stop="openDirectory"
            :label="$t('common.open')" shortcut="⌘O" variant="primary" />
        </div>

        <!-- Documents -->
        <div v-for="(doc, index) in filteredDocuments" :key="doc.id" :data-result-index="index + 1"
          @mouseenter="selectedIndex = index + 1" :class="[
            'group flex items-center gap-3 px-3 py-2.5 rounded-lg cursor-pointer transition-all duration-150 relative',
            selectedIndex === index + 1
              ? 'bg-gray-100 before:absolute before:left-0 before:top-1/2 before:-translate-y-1/2 before:w-0.5 before:h-6 before:bg-indigo-600 before:rounded-r-sm dark:bg-gray-600'
              : 'hover:bg-gray-50 dark:hover:bg-gray-700'
          ]">
          <!-- File icon -->
          <FileIcon class="shrink-0 size-5 text-gray-500 dark:text-gray-400" />
          <div class="flex-1 min-w-0">
            <div class="text-sm font-medium text-gray-900 truncate dark:text-gray-100">
              {{ doc.name }}
            </div>
            <div class="flex items-center gap-2 text-gray-400 mt-0.5 text-xs">
              <span class="text-[10px] text-medium text-gray-500 bg-gray-200 px-1 py-0.5 rounded-md dark:bg-gray-700 dark:text-gray-400">{{ getFriendlyFileType(doc.contentType) }}</span>
              <span>•</span>
              <span>{{ formatFileSize(doc.size) }}</span>
            </div>
          </div>

          <!-- Status Icon (only show for non-completed states) -->
          <div v-if="doc.status !== 'completed'" class="flex items-center">
            <!-- Failed -->
            <XIcon v-if="doc.status === 'failed'" class="size-4 text-red-500" />
            <!-- Processing / Pending -->
            <LoadingSpinner v-else class="size-4 text-indigo-500" />
          </div>

          <!-- Action Buttons -->
          <div v-show="selectedIndex === index + 1" class="flex items-center gap-1">
            <ActionButton @click.stop="openFile(doc)" :label="$t('common.open')" shortcut="⌘O" variant="primary" />
            <ConfirmDeleteButton
              :active="selectedIndex === index + 1"
              @delete="handleDeleteDocument(doc)"
            />
          </div>
        </div>
      </div>
    </div>

    <!-- Empty state -->
    <div v-if="filteredDocuments.length === 0"
      class="flex flex-col items-center justify-center px-6 py-12 text-center">
      <FileTextIcon class="size-12 text-gray-300 mb-3 dark:text-gray-600" />
      <div class="text-base font-semibold text-gray-700 mb-1 dark:text-gray-300">{{ $t('stores.noFiles') }}</div>
      <div class="text-sm text-gray-400">{{ $t('stores.addFilesHint') }}</div>
      <div class="mt-4 text-xs text-gray-400">{{ $t('hints.goBackHint') }}</div>
    </div>
  </SearchLayout>
</template>

<script setup>
import { ref, computed } from 'vue'
import SearchLayout from '@/views/layout/application.vue'
import ActionButton from '@/components/ActionButton.vue'
import ConfirmDeleteButton from '@/components/ConfirmDeleteButton.vue'
import { SHORTCUTS } from '@/utils/constants.js'
import { FolderIcon, FileIcon, XIcon, LoadingSpinner, FileTextIcon } from '@/utils/icons.js'
import { formatFileSize, getFriendlyFileType } from '@/utils/helpers.js'
import { useGlobalKeyboard } from '@/composables/useGlobalKeyboard'
import { useListNavigation } from '@/composables/useListNavigation'
import { useStoreDetail } from '@/composables/useStoreDetail'

const layoutRef = ref(null)

// Store data and operations
const {
  selectedStore,
  filteredDocuments,
  searchQuery,
  openDirectory,
  openFile,
  deleteDocument,
  goBack
} = useStoreDetail()

// List navigation
const itemCount = computed(() => filteredDocuments.value.length + 1) // +1 for directory row

const {
  selectedIndex,
  navigateUp,
  navigateDown,
  adjustSelectionAfterRemoval
} = useListNavigation({
  itemCount,
  layoutRef,
  searchQuery
})

// Get input ref from layout for global keyboard
const getInputRef = () => layoutRef.value?.searchInput

// Setup global keyboard shortcuts
const {
  onNavigateUp,
  onNavigateDown,
  onEscape,
  onOpenFile
} = useGlobalKeyboard({
  inputRef: computed(() => getInputRef())
})

// Event handlers
const handleEscape = () => {
  goBack()
}

const handleOpenFileOrDirectory = () => {
  if (selectedIndex.value === 0) {
    openDirectory()
  } else {
    const docIndex = selectedIndex.value - 1
    if (docIndex >= 0 && docIndex < filteredDocuments.value.length) {
      openFile(filteredDocuments.value[docIndex])
    }
  }
}

const handleDeleteDocument = async (doc) => {
  await deleteDocument(doc)
  adjustSelectionAfterRemoval()
}

const handleKeydown = (e) => {
  // Check for Cmd+O (Meta+O) to open file or directory
  if (e.key === SHORTCUTS.OPEN_FILE && e.metaKey) {
    e.preventDefault()
    handleOpenFileOrDirectory()
  }
  // DELETE shortcut is now handled by ConfirmDeleteButton component
}

// Register global keyboard handlers
onNavigateUp(navigateUp)
onNavigateDown(navigateDown)
onEscape(handleEscape)
onOpenFile(handleOpenFileOrDirectory)
// onDelete is now handled by ConfirmDeleteButton component
</script>
