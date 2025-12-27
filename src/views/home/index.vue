<template>
  <SearchLayout :placeholder="$t('search.placeholder')" @navigate-up="navigateUp" @navigate-down="navigateDown"
    @enter="handleEnter" ref="layoutRef">

    <!-- Settings section -->
    <div v-if="!searchQuery" class="mb-4 last:mb-0">
      <div
        class="px-3 py-1 mb-1 text-[11px] font-semibold uppercase tracking-wider text-gray-400 font-['Geist_Mono',monospace] dark:text-gray-500">
        {{ $t('settings.title') }}</div>
      <div class="flex flex-col gap-0.5">
        <div v-for="(item, index) in settingsItems" :key="item.id" :data-result-index="index"
          @click="selectResult(item)" @mouseenter="selectedIndex = index" :class="[
            'flex items-center gap-3 px-3 py-2.5 rounded-lg cursor-pointer transition-all duration-150 relative',
            selectedIndex === index
              ? 'bg-gray-100 before:absolute before:left-0 before:top-1/2 before:-translate-y-1/2 before:w-0.5 before:h-6 before:bg-indigo-600 before:rounded-r-sm dark:bg-gray-600'
              : 'hover:bg-gray-50 dark:hover:bg-gray-700'
          ]" data-no-drag>
          <ThemeIcon v-if="item.action === 'changeTheme'" class="shrink-0 size-5 text-gray-500 dark:text-gray-400" />
          <GlobeIcon v-else-if="item.action === 'changeLanguage'" class="shrink-0 size-5 text-gray-500 dark:text-gray-400" />
          <KeyIcon v-else-if="item.action === 'configureApiKey'" class="shrink-0 size-5 text-gray-500 dark:text-gray-400" />
          <LoadingSpinner v-else-if="item.action === 'checkUpdate' && isChecking" class="shrink-0 size-5 text-gray-500 dark:text-gray-400" />
          <DownloadIcon v-else-if="item.action === 'checkUpdate'" class="shrink-0 size-5 text-gray-500 dark:text-gray-400" />
          <div class="flex-1 min-w-0">
            <div
              class="text-sm font-medium text-gray-900 truncate dark:text-gray-100">
              {{ item.title }}</div>
            <div v-if="item.action === 'checkUpdate' && updateStatus"
              class="text-xs text-gray-400 mt-0.5 truncate dark:text-gray-500">
              {{ updateStatus }}</div>
          </div>
        </div>
      </div>
    </div>

    <!-- Search results when typing -->
    <div v-if="searchQuery && filteredResults.length > 0" class="mb-4 last:mb-0">
      <div class="flex flex-col gap-0.5">
        <div v-for="(result, index) in filteredResults" :key="result.id" :data-result-index="index"
          @click="selectResult(result)" @mouseenter="selectedIndex = index" :class="[
            'flex items-center gap-3 px-3 py-2.5 rounded-lg cursor-pointer transition-all duration-150 relative',
            selectedIndex === index
              ? 'bg-gray-100 before:absolute before:left-0 before:top-1/2 before:-translate-y-1/2 before:w-0.5 before:h-6 before:bg-indigo-600 before:rounded-r-sm dark:bg-gray-600'
              : 'hover:bg-gray-50 dark:hover:bg-gray-700'
          ]" data-no-drag>
          <ThemeIcon v-if="result.action === 'changeTheme'" class="shrink-0 size-5 text-gray-500 dark:text-gray-400" />
          <GlobeIcon v-else-if="result.action === 'changeLanguage'" class="shrink-0 size-5 text-gray-500 dark:text-gray-400" />
          <KeyIcon v-else-if="result.action === 'configureApiKey'" class="shrink-0 size-5 text-gray-500 dark:text-gray-400" />
          <LoadingSpinner v-else-if="result.action === 'checkUpdate' && isChecking" class="shrink-0 size-5 text-gray-500 dark:text-gray-400" />
          <DownloadIcon v-else-if="result.action === 'checkUpdate'" class="shrink-0 size-5 text-gray-500 dark:text-gray-400" />
          <div class="flex-1 min-w-0">
            <div
              class="text-sm font-medium text-gray-900 truncate dark:text-gray-100 [&_mark]:bg-amber-100 [&_mark]:text-amber-900 [&_mark]:font-semibold [&_mark]:px-0.5 [&_mark]:rounded-sm"
              v-html="highlightMatch(result.title)"></div>
            <div v-if="result.subtitle"
              class="text-[13px] text-gray-400 mt-0.5 truncate dark:text-gray-400">
              {{ result.subtitle }}</div>
          </div>
          <div v-if="result.badge"
            class="shrink-0 px-2 py-1 bg-blue-100 text-blue-800 text-xs font-medium rounded-md">{{ result.badge }}</div>
        </div>
      </div>
    </div>

    <!-- Empty state -->
    <div v-if="searchQuery && filteredResults.length === 0"
      class="flex flex-col items-center justify-center px-6 py-12 text-center">
      <div class="text-gray-200 dark:text-gray-600 mb-4">
        <CirclePlusIcon class="size-12" />
      </div>
      <div class="text-base font-semibold text-gray-700 mb-1 dark:text-gray-300">{{ $t('empty.noResults') }}</div>
      <div class="text-sm text-gray-400">{{ $t('empty.trySearchingElse') }}</div>
    </div>
  </SearchLayout>
</template>

<script setup>
import { ref, computed, watch } from 'vue'
import SearchLayout from '@/views/layout/application.vue'
import { ThemeIcon, CirclePlusIcon, GlobeIcon, KeyIcon, DownloadIcon, LoadingSpinner } from '@/utils/icons.js'
import { useGlobalKeyboard } from '@/composables/useGlobalKeyboard'
import { useHomeSettings } from '@/composables/useHomeSettings'

const layoutRef = ref(null)
const selectedIndex = ref(-1)

// Home settings functionality
const {
  settingsItems,
  allItems,
  filteredResults,
  searchQuery,
  selectResult,
  highlightMatch,
  switchToPrevTab,
  switchToNextTab,
  enterAiChat,
  updateStatus,
  isChecking
} = useHomeSettings()

// Get input ref from layout for global keyboard
const getInputRef = () => layoutRef.value?.searchInput

// Setup global keyboard shortcuts
const {
  onNavigateUp,
  onNavigateDown,
  onNavigateLeft,
  onNavigateRight,
  onEnter,
  onTab
} = useGlobalKeyboard({
  inputRef: computed(() => getInputRef())
})

// Navigation (special: starts at -1, no initial selection)
const navigateUp = () => {
  const maxIndex = searchQuery.value ? filteredResults.value.length - 1 : allItems.value.length - 1
  if (selectedIndex.value < 0) {
    selectedIndex.value = 0
  } else if (selectedIndex.value > 0) {
    selectedIndex.value--
  } else {
    selectedIndex.value = maxIndex
  }
  if (layoutRef.value) {
    layoutRef.value.scrollToSelectedItem(selectedIndex.value)
  }
}

const navigateDown = () => {
  const maxIndex = searchQuery.value ? filteredResults.value.length - 1 : allItems.value.length - 1
  if (selectedIndex.value < 0) {
    selectedIndex.value = 0
  } else if (selectedIndex.value < maxIndex) {
    selectedIndex.value++
  } else {
    selectedIndex.value = 0
  }
  if (layoutRef.value) {
    layoutRef.value.scrollToSelectedItem(selectedIndex.value)
  }
}

const handleEnter = () => {
  const items = searchQuery.value ? filteredResults.value : allItems.value
  if (items.length > 0 && selectedIndex.value >= 0 && selectedIndex.value < items.length) {
    selectResult(items[selectedIndex.value])
  }
}

// Reset selection when search query changes
watch(searchQuery, () => {
  selectedIndex.value = -1
})

// Register global keyboard handlers
onNavigateUp(navigateUp)
onNavigateDown(navigateDown)
onNavigateLeft(switchToPrevTab)
onNavigateRight(switchToNextTab)
onEnter(handleEnter)
onTab(enterAiChat)
</script>
