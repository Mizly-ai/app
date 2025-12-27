<template>
  <SearchLayout :placeholder="$t('apiKey.searchPlaceholder')" :prevent-close="true" @escape="goBack">

    <template #search-icon>
      <div
        class="px-2 py-0.5 bg-indigo-100 text-indigo-500 text-xs font-medium rounded-md whitespace-nowrap dark:bg-indigo-900 dark:text-indigo-300">
        {{ $t('apiKey.title') }}
      </div>
    </template>

    <template #header-actions>
      <div class="flex items-center gap-1.5">
        <ActionButton @click="goBack" :label="$t('common.back')" shortcut="ESC" />
      </div>
    </template>

    <!-- API Key Status -->
    <div class="mb-4 last:mb-0">
      <div class="flex flex-col gap-0.5">
        <!-- Status indicator -->
        <div class="flex items-center gap-3 px-3 py-2.5 rounded-lg">
          <KeyIcon class="shrink-0 size-5 text-gray-500 dark:text-gray-400" />
          <div class="flex-1 min-w-0">
            <div class="text-sm font-medium text-gray-900 dark:text-gray-100">
              Gemini API Key
            </div>
            <div class="text-[13px] text-gray-400 mt-0.5 dark:text-gray-400">
              {{ $t('apiKey.inputHint') }}
            </div>
          </div>
          <span v-if="hasApiKey && authStore.maskedApiKey"
            class="shrink-0 px-2 py-1 bg-green-100 text-green-600 text-xs font-mono font-medium rounded-md dark:bg-green-900 dark:text-green-300">
            {{ authStore.maskedApiKey }}
          </span>
          <span v-else
            class="shrink-0 px-2 py-1 bg-amber-100 text-amber-600 text-xs font-medium rounded-md dark:bg-amber-900 dark:text-amber-300">
            {{ $t('apiKey.notConfigured') }}
          </span>
        </div>

        <!-- API Key input -->
        <div class="px-3 py-2.5" data-no-drag>
          <input
            ref="apiKeyInput"
            v-model="apiKeyValue"
            type="password"
            :placeholder="hasApiKey ? $t('apiKey.updatePlaceholder') : $t('apiKey.inputPlaceholder')"
            class="w-full px-3 py-2 text-sm bg-gray-50 border border-gray-200 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent dark:bg-gray-700 dark:border-gray-600 dark:text-gray-100 dark:placeholder-gray-400"
            @keydown.enter="saveApiKey"
          />
          <div v-if="errorMessage" class="mt-2 text-sm text-red-500 dark:text-red-400">
            {{ errorMessage }}
          </div>
          <div v-if="successMessage" class="mt-2 text-sm text-green-500 dark:text-green-400">
            {{ successMessage }}
          </div>
        </div>

        <!-- Action buttons -->
        <div class="flex items-center gap-2 px-3 py-2.5" data-no-drag>
          <button
            @click="saveApiKey"
            :disabled="!apiKeyValue || isSaving"
            class="px-4 py-2 text-sm font-medium text-white bg-indigo-600 rounded-lg hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 disabled:opacity-50 disabled:cursor-not-allowed dark:bg-indigo-500 dark:hover:bg-indigo-600"
          >
            {{ $t('apiKey.save') }}
          </button>
          <button
            v-if="hasApiKey"
            @click="clearApiKey"
            :disabled="isSaving"
            class="px-4 py-2 text-sm font-medium text-gray-700 bg-gray-100 rounded-lg hover:bg-gray-200 focus:outline-none focus:ring-2 focus:ring-gray-500 disabled:opacity-50 disabled:cursor-not-allowed dark:bg-gray-700 dark:text-gray-300 dark:hover:bg-gray-600"
          >
            {{ $t('apiKey.clear') }}
          </button>
        </div>

        <!-- Get API Key link -->
        <div
          class="flex items-center gap-3 px-3 py-2.5 rounded-lg cursor-pointer transition-all duration-150 hover:bg-gray-50 dark:hover:bg-gray-700"
          @click="openAiStudio"
          data-no-drag
        >
          <ExternalLinkIcon class="shrink-0 size-5 text-gray-500 dark:text-gray-400" />
          <div class="flex-1 min-w-0">
            <div class="text-sm font-medium text-indigo-600 dark:text-indigo-400">
              {{ $t('apiKey.getApiKey') }}
            </div>
            <div class="text-[13px] text-gray-400 mt-0.5 dark:text-gray-400">
              {{ $t('apiKey.getApiKeyHint') }}
            </div>
          </div>
        </div>
      </div>
    </div>
  </SearchLayout>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { openUrl } from '@tauri-apps/plugin-opener'
import SearchLayout from '@/views/layout/application.vue'
import ActionButton from '@/components/ActionButton.vue'
import { KeyIcon, ExternalLinkIcon } from '@/utils/icons.js'
import { useGlobalKeyboard } from '@/composables/useGlobalKeyboard'
import { useAuthStore } from '@/stores/auth'

const { t } = useI18n()
const router = useRouter()
const authStore = useAuthStore()

const apiKeyValue = ref('')
const errorMessage = ref('')
const successMessage = ref('')
const isSaving = ref(false)
const hasApiKey = ref(false)

// Check if API key is configured on mount
onMounted(async () => {
  await authStore.initialize()
  hasApiKey.value = authStore.hasApiKey
})

const goBack = () => {
  router.push({ name: 'home' })
}

const saveApiKey = async () => {
  if (!apiKeyValue.value || isSaving.value) return

  errorMessage.value = ''
  successMessage.value = ''
  isSaving.value = true

  try {
    await authStore.setApiKey(apiKeyValue.value)
    hasApiKey.value = true
    successMessage.value = t('apiKey.saved')
    apiKeyValue.value = ''
  } catch (e) {
    errorMessage.value = e.toString()
  } finally {
    isSaving.value = false
  }
}

const clearApiKey = async () => {
  if (isSaving.value) return

  errorMessage.value = ''
  successMessage.value = ''
  isSaving.value = true

  try {
    await authStore.clearApiKey()
    hasApiKey.value = false
    successMessage.value = t('apiKey.cleared')
  } catch (e) {
    errorMessage.value = e.toString()
  } finally {
    isSaving.value = false
  }
}

const openAiStudio = async () => {
  await openUrl('https://aistudio.google.com/apikey')
}

// Setup global keyboard shortcuts
const { onEscape } = useGlobalKeyboard()

onEscape(goBack)
</script>
