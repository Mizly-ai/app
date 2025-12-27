<template>
  <SearchLayout :placeholder="$t('stores.searchPlaceholder')" @enter="handleEnter" @navigate-up="navigateUp"
    @navigate-down="navigateDown" @keydown="handleKeydown" ref="layoutRef">

    <!-- Stores List -->
    <div v-if="displayItems.length > 0" class="mb-4 last:mb-0">
      <div class="flex flex-col gap-0.5">
        <div v-for="(item, index) in displayItems" :key="item.id" :data-result-index="index"
          @click="selectItem(item)" @mouseenter="selectedIndex = index" :class="[
            'group flex items-center gap-3 px-3 py-2.5 rounded-lg cursor-pointer transition-all duration-150 relative',
            selectedIndex === index
              ? 'bg-gray-100 before:absolute before:left-0 before:top-1/2 before:-translate-y-1/2 before:w-0.5 before:h-6 before:bg-indigo-600 before:rounded-r-sm dark:bg-gray-600'
              : 'hover:bg-gray-50 dark:hover:bg-gray-700'
          ]" data-no-drag>
          <FolderPlusIcon v-if="item.action === 'create'" class="shrink-0 size-5 text-gray-500 dark:text-gray-400" />
          <FolderIcon v-else class="shrink-0 size-5 text-gray-500 dark:text-gray-400" />
          <div class="flex-1 min-w-0">
            <div
              class="text-sm font-medium text-gray-900 truncate dark:text-gray-100 [&_mark]:bg-amber-100 [&_mark]:text-amber-900 [&_mark]:font-semibold [&_mark]:px-0.5 [&_mark]:rounded-sm"
              v-html="highlightMatch(item.title)"></div>
            <div v-if="item.subtitle"
              class="text-[13px] text-gray-400 mt-0.5 truncate dark:text-gray-400">
              {{ item.subtitle }}</div>
          </div>

          <!-- Status Indicator -->
          <div v-if="item.status === 'processing' || item.failedCount > 0" class="flex items-center">
            <!-- Processing -->
            <LoadingSpinner v-if="item.status === 'processing'" class="size-4 text-indigo-500" />
            <!-- Has Failed Documents -->
            <XIcon v-else-if="item.failedCount > 0" class="size-4 text-red-500" />
          </div>

          <!-- Action Buttons -->
          <!-- New Store Button - Always visible (hint only) -->
          <ActionButton v-if="item.action === 'create'" tag="div" :label="$t('common.create')" shortcut="⌘N" :static="true" />

          <!-- Delete Button - Show on hover/select -->
          <ConfirmDeleteButton
            v-if="item.action !== 'create'"
            v-show="selectedIndex === index"
            :active="selectedIndex === index"
            @delete="handleDeleteItem(item)"
          />
        </div>
      </div>
    </div>

    <!-- Empty state -->
    <div v-if="displayItems.length === 0"
      class="flex flex-col items-center justify-center px-6 py-12 text-center">
      <div class="text-base font-semibold text-gray-700 mb-1">{{ $t('stores.title') }}</div>
      <div class="text-sm text-gray-400">{{ $t('stores.noStores') }}</div>
    </div>
  </SearchLayout>
</template>

<script setup>
import { ref, computed } from 'vue'
import SearchLayout from '@/views/layout/application.vue'
import ActionButton from '@/components/ActionButton.vue'
import ConfirmDeleteButton from '@/components/ConfirmDeleteButton.vue'
import { FolderIcon, FolderPlusIcon, LoadingSpinner, XIcon } from '@/utils/icons.js'
import { SHORTCUTS } from '@/utils/constants.js'
import { useGlobalKeyboard } from '@/composables/useGlobalKeyboard'
import { useListNavigation } from '@/composables/useListNavigation'
import { useStoreList } from '@/composables/useStoreList'

const layoutRef = ref(null)

// Store list data and operations
const {
  displayItems,
  searchQuery,
  selectItem,
  handleSelectAtIndex,
  deleteStore,
  createNew,
  highlightMatch,
  switchTab,
  enterAiChat
} = useStoreList()

// List navigation
const itemCount = computed(() => displayItems.value.length)

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
  onNavigateLeft,
  onNavigateRight,
  onEnter,
  onTab,
  onCreateNew
} = useGlobalKeyboard({
  inputRef: computed(() => getInputRef())
})

// Event handlers
const handleEnter = () => {
  handleSelectAtIndex(selectedIndex.value)
}

const handleDeleteItem = async (item) => {
  await deleteStore(item)
  adjustSelectionAfterRemoval()
}

const handleKeydown = (e) => {
  if (e.key === SHORTCUTS.CREATE_NEW && e.metaKey) {
    e.preventDefault()
    createNew()
  }
  // DELETE shortcut is now handled by ConfirmDeleteButton component
}

// Register global keyboard handlers
onNavigateUp(navigateUp)
onNavigateDown(navigateDown)
onNavigateLeft(() => switchTab('prev'))
onNavigateRight(() => switchTab('next'))
onEnter(handleEnter)
onTab(enterAiChat)
onCreateNew(createNew)
// onDelete is now handled by ConfirmDeleteButton component
</script>
