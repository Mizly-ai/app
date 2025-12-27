<template>
  <router-view />

  <!-- Update notification banner -->
  <Teleport to="body">
    <Transition
      enter-active-class="transition-all duration-300 ease-out"
      enter-from-class="translate-y-full opacity-0"
      enter-to-class="translate-y-0 opacity-100"
      leave-active-class="transition-all duration-200 ease-in"
      leave-from-class="translate-y-0 opacity-100"
      leave-to-class="translate-y-full opacity-0"
    >
      <div v-if="showUpdateBanner"
        class="fixed bottom-4 left-1/2 -translate-x-1/2 z-50 px-4 py-3 bg-indigo-600 text-white rounded-lg shadow-lg flex items-center gap-3">
        <DownloadIcon class="size-5" />
        <span class="text-sm font-medium">{{ $t('updater.updateAvailable', { version: updateVersion }) }}</span>
        <button @click="installUpdate"
          class="px-3 py-1 bg-white/20 hover:bg-white/30 rounded text-sm font-medium transition-colors">
          {{ $t('common.open') }}
        </button>
        <button @click="dismissUpdate"
          class="p-1 hover:bg-white/20 rounded transition-colors">
          <XIcon class="size-4" />
        </button>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { useUpdater } from '@/composables/useUpdater'
import { DownloadIcon, XIcon } from '@/utils/icons'

const { checkOnStartup, downloadAndInstall } = useUpdater()

const showUpdateBanner = ref(false)
const updateVersion = ref('')
const pendingUpdate = ref(null)

onMounted(async () => {
  const result = await checkOnStartup()
  if (result) {
    updateVersion.value = result.info.version
    pendingUpdate.value = result.update
    showUpdateBanner.value = true
  }
})

const installUpdate = async () => {
  if (pendingUpdate.value) {
    showUpdateBanner.value = false
    await downloadAndInstall(pendingUpdate.value)
  }
}

const dismissUpdate = () => {
  showUpdateBanner.value = false
}
</script>
