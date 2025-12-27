<template>
  <div
    class="flex items-start justify-center h-screen font-['Geist',-apple-system,BlinkMacSystemFont,system-ui,sans-serif] select-none"
    @mousedown="handleDragStart"
    @click.self="handleBackdropClick">
    <!-- Main search palette -->
    <div
      class="relative w-full max-w-[700px] h-[500px] bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-2xl flex flex-col overflow-hidden select-none"
      @wheel="handleWheel">

      <!-- Search header with input -->
      <div
        class="border-b border-gray-200 bg-linear-to-b from-white to-gray-50 dark:border-gray-700 dark:from-gray-800 dark:to-gray-900">
        <div class="flex items-center px-5 py-4 gap-3">
          <div class="shrink-0 text-gray-400 flex items-center justify-center">
            <template v-if="isAiChatMode">
              <span
                class="px-2 py-1 text-xs font-sans font-medium text-white bg-linear-to-r from-indigo-500 to-purple-600 rounded-md">
                {{ $t('aiChat.title') }}
              </span>
            </template>
            <template v-else>
              <slot name="search-icon">
                <SearchIcon />
              </slot>
            </template>
          </div>
          <form @submit.prevent="handleEnter" class="flex-1 flex">
            <input ref="searchInput" :value="appStore.searchQuery" @input="handleSearch" @keydown="handleKeydown"
              @keydown.up.prevent="navigateUp" @keydown.down.prevent="navigateDown" @keydown.left="handleLeft"
              @keydown.right="handleRight" @keydown.tab.prevent="handleTab" type="text"
              :placeholder="currentPlaceholder" :disabled="isInputDisabled"
              class="flex-1 border-none outline-none text-base font-[450] text-gray-900 bg-transparent font-['Geist',-apple-system,BlinkMacSystemFont,system-ui,sans-serif] placeholder:text-gray-400 placeholder:font-normal dark:text-gray-100 dark:placeholder:text-gray-500 disabled:opacity-50 disabled:cursor-not-allowed"
              autocomplete="off" spellcheck="false" />
          </form>
          <div class="flex items-center gap-1">
            <slot name="header-actions">
              <ActionButton @click="handleTab" :label="$t('keyboard.aiChat')" shortcut="Tab" variant="ai" />
            </slot>
          </div>
        </div>

        <!-- Tab navigation (hidden in AI Chat mode) -->
        <div v-if="!isAiChatMode" class="flex gap-0.5 px-2 py-2 overflow-x-auto scrollbar-none">
          <button v-for="tab in localizedTabs" :key="tab.id" @click="appStore.changeTab(tab.id)" :class="[
            'flex items-center gap-1.5 px-3.5 py-2 border-none text-sm font-medium cursor-pointer rounded-lg transition-all duration-150 whitespace-nowrap font-[\'Geist\',-apple-system,BlinkMacSystemFont,system-ui,sans-serif]',
            appStore.activeTab === tab.id
              ? 'bg-gray-900 text-white dark:bg-indigo-600'
              : 'bg-transparent text-gray-500 hover:bg-gray-100 hover:text-gray-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-gray-200'
          ]" :data-count="tab.count">
            <HomeIcon v-if="tab.id === 'home'" class="size-4" />
            <StoreIcon v-else-if="tab.id === 'stores'" class="size-4" />
            <span>{{ tab.label }}</span>
            <span v-if="tab.count"
              class="px-1.5 min-w-5 h-[18px] flex items-center justify-center bg-white/20 rounded-[9px] text-[11px] font-semibold font-['Geist_Mono',monospace]">
              {{ tab.count }}</span>
          </button>
        </div>
      </div>

      <!-- Search results -->
      <div ref="resultsContainer"
        class="flex-1 flex flex-col overflow-y-auto p-2 scrollbar-thin scrollbar-track-transparent scrollbar-thumb-gray-200 dark:scrollbar-thumb-gray-700">
        <slot></slot>
      </div>

      <!-- Footer -->
      <div
        class="border-t border-gray-200 px-5 py-3 flex items-center justify-between bg-linear-to-t from-white to-gray-50 dark:border-gray-700 dark:from-gray-800 dark:to-gray-900">
        <div class="flex items-center gap-4">
          <template v-if="isAiChatMode">
            <KeyboardHint key-label="↵" :label="$t('common.send')" />
            <KeyboardHint key-label="ESC" :label="$t('keyboard.exitAiChat')" />
          </template>
          <template v-else>
            <KeyboardHint key-label="Tab" :label="$t('keyboard.aiChat')" />
            <KeyboardHint key-label="↓↑" :label="$t('keyboard.navigate')" />
            <KeyboardHint key-label="↵" :label="$t('keyboard.select')" />
            <KeyboardHint key-label="ESC" :label="$t('keyboard.close')" />
          </template>
        </div>
        <div class="flex items-center gap-2 text-xs">
          <span class="text-gray-700 font-semibold font-['Geist_Mono',monospace] dark:text-gray-200">{{ $t('app.name') }}</span>
          <span class="text-gray-400 font-['Geist_Mono',monospace]">v{{ version }}</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow'
import { invoke } from '@tauri-apps/api/core'
import { useAppStore } from '@/stores/app'
import { useWindowDrag } from '@/composables/useWindowDrag'
import { SearchIcon, HomeIcon, StoreIcon } from '@/utils/icons.js'
import { version } from '../../../package.json'
import KeyboardHint from '@/components/KeyboardHint.vue'
import ActionButton from '@/components/ActionButton.vue'

const { t } = useI18n()
const { handleDragStart } = useWindowDrag()

const props = defineProps({
  placeholder: {
    type: String,
    default: ''
  },
  preventClose: {
    type: Boolean,
    default: false
  },
  preventTabSwitch: {
    type: Boolean,
    default: false
  },
  isAiChatMode: {
    type: Boolean,
    default: false
  },
  isInputDisabled: {
    type: Boolean,
    default: false
  }
})

const emit = defineEmits(['navigate-up', 'navigate-down', 'navigate-left', 'navigate-right', 'enter', 'escape', 'keydown', 'ai-chat-send'])

const appStore = useAppStore()

const localizedTabs = computed(() => {
  return appStore.tabs.map(tab => ({
    ...tab,
    label: tab.id === 'home' ? t('app.tabs.main') : tab.id === 'stores' ? t('app.tabs.stores') : tab.label
  }))
})

const currentPlaceholder = computed(() => {
  if (props.isAiChatMode) {
    return t('search.askAi')
  }
  return props.placeholder || t('search.placeholder')
})

const searchInput = ref(null)
const resultsContainer = ref(null)

const handleSearch = (e) => {
  appStore.setSearchQuery(e.target.value)
}

const blurInputAndHide = async () => {
  if (searchInput.value) {
    searchInput.value.blur()
  }
  await new Promise(resolve => setTimeout(resolve, 50))
  await invoke('hide_window')
}

const handleKeydown = (e) => {
  // Handle escape separately
  if (e.key === 'Escape') {
    handleEscape()
    return
  }

  // Emit keydown event to parent for custom shortcuts (cmd+N, cmd+O, cmd+D, etc.)
  emit('keydown', e)
}

const handleEscape = async () => {
  emit('escape')

  if (props.isAiChatMode) {
    return
  }

  if (props.preventClose) return

  if (appStore.searchQuery) {
    appStore.clearSearchQuery()
  } else {
    await blurInputAndHide()
  }
}

const handleTab = () => {
  if (!props.isAiChatMode) {
    appStore.enterAiChat()
    appStore.clearSearchQuery()
  }
}

const handleBackdropClick = async () => {
  if (props.preventClose) return
  await blurInputAndHide()
}

const handleEnter = () => {
  if (props.isAiChatMode) {
    if (appStore.searchQuery.trim()) {
      emit('ai-chat-send', appStore.searchQuery)
    } else {
      // Emit enter for suggest questions selection when input is empty
      emit('enter')
    }
  } else {
    emit('enter')
  }
}

const navigateUp = () => {
  emit('navigate-up')
}

const navigateDown = () => {
  emit('navigate-down')
}

const handleLeft = (e) => {
  if (props.isAiChatMode) return

  const input = searchInput.value
  if (!input) return

  if (input.selectionStart === 0 && input.selectionEnd === 0) {
    e.preventDefault()
    if (props.preventTabSwitch) {
      emit('navigate-left')
    } else {
      appStore.switchTab('prev')
    }
  }
}

const handleRight = (e) => {
  if (props.isAiChatMode) return

  const input = searchInput.value
  if (!input) return

  if (input.selectionStart === input.value.length && input.selectionEnd === input.value.length) {
    e.preventDefault()
    if (props.preventTabSwitch) {
      emit('navigate-right')
    } else {
      appStore.switchTab('next')
    }
  }
}

const handleWheel = (event) => {
  if (!resultsContainer.value) return
  event.preventDefault()
  resultsContainer.value.scrollTop += event.deltaY
}

const scrollToSelectedItem = (index) => {
  if (!resultsContainer.value) return

  const selectedElement = resultsContainer.value.querySelector(`[data-result-index="${index}"]`)
  if (!selectedElement) return

  const container = resultsContainer.value
  const elementRect = selectedElement.getBoundingClientRect()
  const containerRect = container.getBoundingClientRect()

  if (elementRect.top < containerRect.top) {
    selectedElement.scrollIntoView({ block: 'nearest', behavior: 'smooth' })
  }
  else if (elementRect.bottom > containerRect.bottom) {
    selectedElement.scrollIntoView({ block: 'nearest', behavior: 'smooth' })
  }
}

const focusInput = () => {
  if (searchInput.value) {
    searchInput.value.focus()
  }
}

defineExpose({
  scrollToSelectedItem,
  resultsContainer,
  focusInput,
  searchInput
})

let unlisten = null

onMounted(async () => {
  if (searchInput.value) {
    searchInput.value.focus()
  }

  try {
    const webview = getCurrentWebviewWindow()
    unlisten = await webview.listen('focus-search', () => {
      if (searchInput.value) {
        searchInput.value.focus()
        appStore.clearSearchQuery()
      }
    })
  } catch (error) {
    console.warn('Event listener setup skipped (likely in browser mode):', error)
  }
})

onUnmounted(() => {
  if (unlisten) {
    unlisten()
  }
})
</script>
