<template>
  <SearchLayout :is-ai-chat-mode="true" :is-input-disabled="chatStore.isLoading" :placeholder="$t('search.askAi')"
    @escape="exitChat" @ai-chat-send="sendMessage" @navigate-up="navigateSuggestUp"
    @navigate-down="navigateSuggestDown" @enter="handleSuggestEnter" ref="layoutRef">

    <template #header-actions>
      <div class="flex items-center gap-1.5">
        <ActionButton @click="exitChat" :label="$t('common.exit')" shortcut="ESC" />
      </div>
    </template>

    <!-- User question -->
    <div v-if="chatStore.currentQuestion" class="mb-4 last:mb-0">
      <div class="flex flex-col gap-0.5">
        <div class="flex items-start gap-3 px-3 py-2.5 rounded-lg bg-gray-50 dark:bg-gray-700/50" data-no-drag>
          <div
            class="shrink-0 size-8 rounded-full bg-linear-to-br from-gray-400 to-gray-500 flex items-center justify-center">
            <UserIcon class="size-4 text-white" />
          </div>
          <div class="flex-1 min-w-0">
            <div class="text-sm font-medium text-gray-900 dark:text-gray-100 mb-1">{{ $t('aiChat.you') }}</div>
            <div class="text-sm text-gray-700 dark:text-gray-300 break-word leading-relaxed select-text">
              {{ chatStore.currentQuestion }}
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Loading state -->
    <div v-if="chatStore.isLoading && !chatStore.response" class="mb-4 last:mb-0">
      <div class="flex flex-col gap-0.5">
        <div class="flex items-start gap-3 px-3 py-2.5 rounded-lg">
          <div
            class="shrink-0 size-8 rounded-full bg-linear-to-br from-indigo-500 to-purple-600 flex items-center justify-center">
            <BirdIcon class="size-4 text-white" />
          </div>
          <div class="flex-1 min-w-0">
            <div class="text-sm font-medium text-gray-900 dark:text-gray-100 mb-1">{{ $t('aiChat.assistant') }}</div>
            <div class="flex items-center gap-2">
              <div class="size-5 border-2 border-indigo-500 border-t-transparent rounded-full animate-spin"></div>
              <span class="text-sm text-gray-500 dark:text-gray-400">{{ $t('aiChat.thinking') }}</span>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Response content -->
    <div v-else-if="chatStore.response" class="mb-4 last:mb-0">
      <div class="flex flex-col gap-0.5">
        <div class="flex items-start gap-3 px-3 py-2.5 rounded-lg" data-no-drag>
          <div
            class="shrink-0 size-8 rounded-full bg-linear-to-br from-indigo-500 to-purple-600 flex items-center justify-center">
            <BirdIcon class="size-4 text-white" />
          </div>
          <div class="flex-1 min-w-0">
            <div class="text-sm font-medium text-gray-900 dark:text-gray-100 mb-1">{{ $t('aiChat.assistant') }}</div>
            <div
              class="text-sm text-gray-700 dark:text-gray-300 break-word leading-relaxed select-text prose prose-sm prose-gray dark:prose-invert max-w-none prose-pre:bg-gray-100 dark:prose-pre:bg-gray-800 prose-pre:text-gray-800 dark:prose-pre:text-gray-200 prose-code:text-indigo-600 dark:prose-code:text-indigo-400 prose-code:before:content-none prose-code:after:content-none prose-a:text-indigo-600 dark:prose-a:text-indigo-400"
              v-html="renderedContent">
            </div>
            <span v-if="chatStore.isStreaming"
              class="inline-block w-1.5 h-4 ml-0.5 bg-indigo-500 animate-pulse rounded-sm"></span>

            <!-- Source documents -->
            <div v-if="sourceDocuments.length > 0 && !chatStore.isStreaming"
              class="mt-4 pt-3 border-t border-gray-200 dark:border-gray-700">
              <div class="text-xs font-medium text-gray-500 dark:text-gray-400 mb-2">{{ $t('aiChat.sources') }}</div>
              <div class="flex flex-wrap gap-2">
                <button v-for="doc in sourceDocuments" :key="doc.id" @click="openDocument(doc)"
                  class="flex items-center gap-1.5 px-2.5 py-1.5 text-xs font-medium text-gray-700 dark:text-gray-300 bg-gray-100 dark:bg-gray-700 rounded-lg hover:bg-gray-200 dark:hover:bg-gray-600 transition-colors cursor-pointer">
                  <FileTextIcon class="size-3.5 text-gray-500 dark:text-gray-400" />
                  <span class="truncate max-w-[200px]">{{ doc.name }}</span>
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Error state -->
    <div v-else-if="chatStore.error" class="mb-4 last:mb-0">
      <div class="flex flex-col items-center justify-center px-6 py-12 text-center">
        <div class="size-12 rounded-full bg-red-100 dark:bg-red-900/30 flex items-center justify-center mb-3">
          <CircleXIcon class="size-6 text-red-500" />
        </div>
        <div class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">{{ $t('aiChat.somethingWentWrong') }}</div>
        <div class="text-xs text-gray-500 dark:text-gray-400 max-w-xs mb-3">{{ chatStore.error }}</div>
        <button @click="chatStore.retrySend()"
          class="px-4 py-1.5 text-sm font-medium text-white bg-indigo-600 hover:bg-indigo-700 rounded-lg transition-colors">
          {{ $t('common.tryAgain') }}
        </button>
      </div>
    </div>

    <!-- Suggest Questions (show after loaded) -->
    <div v-else-if="showSuggestQuestions" class="mb-4 last:mb-0 animate-fade-in">
      <div
        class="px-3 py-1 mb-1 text-[11px] font-semibold uppercase tracking-wider text-gray-400 font-['Geist_Mono',monospace] dark:text-gray-500">
        {{ $t('aiChat.suggestQuestions') }}
      </div>
      <div class="flex flex-col gap-0.5">
        <div v-for="(question, index) in chatStore.suggestQuestions" :key="index" :data-result-index="index"
          @click="selectSuggestQuestion(question)" @mouseenter="selectedSuggestIndex = index"
          :style="{ animationDelay: `${index * 50}ms` }"
          :class="[
            'group flex items-center gap-3 px-3 py-2.5 rounded-lg cursor-pointer transition-all duration-150 relative animate-fade-in-item',
            selectedSuggestIndex === index
              ? 'bg-gray-100 before:absolute before:left-0 before:top-1/2 before:-translate-y-1/2 before:w-0.5 before:h-6 before:bg-indigo-600 before:rounded-r-sm dark:bg-gray-600'
              : 'hover:bg-gray-50 dark:hover:bg-gray-700'
          ]" data-no-drag>
          <MessageCircleQuestionIcon class="shrink-0 size-5 text-gray-500 dark:text-gray-400" />
          <div class="flex-1 min-w-0">
            <div class="text-sm font-medium text-gray-900 dark:text-gray-100 break-words leading-relaxed">
              {{ question }}
            </div>
          </div>
          <ActionButton v-show="selectedSuggestIndex === index" @click.stop="selectSuggestQuestion(question)"
            :label="$t('common.send')" shortcut="↵" variant="primary" />
        </div>
      </div>
    </div>

    <!-- Empty state / Instructions -->
    <div v-else class="flex-1 flex items-center justify-center">
      <div class="flex flex-col items-center justify-center px-6 py-12 text-center">
        <div
          class="size-12 rounded-full bg-linear-to-br from-indigo-100 to-purple-100 dark:from-indigo-900/30 dark:to-purple-900/30 flex items-center justify-center mb-3">
          <BirdIcon class="size-6 text-indigo-500" />
        </div>
        <div class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">{{ $t('aiChat.askAnything') }}</div>
        <div class="text-xs text-gray-500 dark:text-gray-400">{{ $t('aiChat.typeQuestion') }}</div>
      </div>
    </div>

    <!-- Scroll anchor -->
    <div ref="scrollAnchorRef"></div>
  </SearchLayout>
</template>

<script setup>
import { ref, computed } from 'vue'
import SearchLayout from '@/views/layout/application.vue'
import ActionButton from '@/components/ActionButton.vue'
import { BirdIcon, UserIcon, FileTextIcon, CircleXIcon, MessageCircleQuestionIcon } from '@/utils/icons.js'
import { useGlobalKeyboard } from '@/composables/useGlobalKeyboard'
import { useAiChat } from '@/composables/useAiChat'

const layoutRef = ref(null)
const scrollAnchorRef = ref(null)

// AI Chat functionality
const {
  sourceDocuments,
  selectedSuggestIndex,
  renderedContent,
  showSuggestQuestions,
  chatStore,
  sendMessage,
  selectSuggestQuestion,
  exitChat,
  openDocument,
  navigateSuggestUp,
  navigateSuggestDown,
  handleSuggestEnter
} = useAiChat({
  layoutRef,
  scrollAnchorRef
})

// Get input ref from layout for global keyboard
const getInputRef = () => layoutRef.value?.searchInput

// Setup global keyboard shortcuts
const {
  onEscape,
  onNavigateUp,
  onNavigateDown,
  onEnter
} = useGlobalKeyboard({
  inputRef: computed(() => getInputRef())
})

// Register global keyboard handlers
onEscape(exitChat)
onNavigateUp(navigateSuggestUp)
onNavigateDown(navigateSuggestDown)
onEnter(handleSuggestEnter)
</script>
