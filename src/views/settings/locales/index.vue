<template>
  <SearchLayout :placeholder="$t('locale.searchPlaceholder')" :prevent-close="true" @enter="handleEnter"
    @navigate-up="navigateUp" @navigate-down="navigateDown" @escape="goBack" ref="layoutRef">

    <template #search-icon>
      <div
        class="px-2 py-0.5 bg-indigo-100 text-indigo-500 text-xs font-medium rounded-md whitespace-nowrap dark:bg-indigo-900 dark:text-indigo-300">
        {{ $t('locale.title') }}
      </div>
    </template>

    <template #header-actions>
      <div class="flex items-center gap-1.5">
        <ActionButton @click="goBack" :label="$t('common.back')" shortcut="ESC" />
      </div>
    </template>

    <!-- Language List -->
    <div v-if="displayItems.length > 0" class="mb-4 last:mb-0">
      <div class="flex flex-col gap-0.5">
        <div v-for="(item, index) in displayItems" :key="item.code" :data-result-index="index"
          @click="selectLocale(item)" @mouseenter="selectedIndex = index" :class="[
            'group flex items-center gap-3 px-3 py-2.5 rounded-lg cursor-pointer transition-all duration-150 relative',
            selectedIndex === index
              ? 'bg-gray-100 before:absolute before:left-0 before:top-1/2 before:-translate-y-1/2 before:w-0.5 before:h-6 before:bg-indigo-600 before:rounded-r-sm dark:bg-gray-600'
              : 'hover:bg-gray-50 dark:hover:bg-gray-700'
          ]" data-no-drag>
          <GlobeIcon class="shrink-0 size-5 text-gray-500 dark:text-gray-400" />
          <div class="flex-1 min-w-0">
            <div
              class="text-sm font-medium text-gray-900 truncate dark:text-gray-100 [&_mark]:bg-amber-100 [&_mark]:text-amber-900 [&_mark]:font-semibold [&_mark]:px-0.5 [&_mark]:rounded-sm"
              v-html="highlightMatch(item.nativeName)"></div>
            <div class="text-[13px] text-gray-400 mt-0.5 truncate dark:text-gray-400">
              {{ item.name }}
            </div>
          </div>

          <!-- Current indicator -->
          <span v-if="item.code === currentLocale"
            class="shrink-0 px-2 py-1 bg-indigo-100 text-indigo-500 text-xs font-medium rounded-md dark:bg-indigo-900 dark:text-indigo-300">
            {{ $t('locale.current') }}
          </span>
        </div>
      </div>
    </div>
  </SearchLayout>
</template>

<script setup>
import { ref, computed } from 'vue'
import SearchLayout from '@/views/layout/application.vue'
import ActionButton from '@/components/ActionButton.vue'
import { GlobeIcon } from '@/utils/icons.js'
import { useGlobalKeyboard } from '@/composables/useGlobalKeyboard'
import { useListNavigation } from '@/composables/useListNavigation'
import { useLocaleSelector } from '@/composables/useLocaleSelector'

const layoutRef = ref(null)

// Locale selector functionality
const {
  displayItems,
  currentLocale,
  searchQuery,
  selectLocale,
  goBack,
  highlightMatch
} = useLocaleSelector()

// List navigation
const itemCount = computed(() => displayItems.value.length)

const {
  selectedIndex,
  navigateUp,
  navigateDown
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
  onEnter,
  onEscape
} = useGlobalKeyboard({
  inputRef: computed(() => getInputRef())
})

// Handle enter to select locale
const handleEnter = () => {
  const items = displayItems.value
  if (items.length > 0 && selectedIndex.value >= 0 && selectedIndex.value < items.length) {
    selectLocale(items[selectedIndex.value])
  }
}

// Register global keyboard handlers
onNavigateUp(navigateUp)
onNavigateDown(navigateDown)
onEnter(handleEnter)
onEscape(goBack)
</script>
