<template>
  <div class="flex items-center gap-1.5 text-xs text-gray-500 dark:text-gray-400">
    <kbd
      class="inline-flex items-center justify-center min-w-6 h-[22px] px-1.5 bg-linear-to-b from-white to-gray-50 border border-gray-300 rounded-md shadow-[inset_0_-1px_0_0_rgb(229,231,235),0_1px_2px_0_rgba(0,0,0,0.05)] text-[11px] font-medium font-['Geist_Mono',monospace] text-gray-700 dark:from-gray-700 dark:to-gray-800 dark:border-gray-600 dark:shadow-[inset_0_-1px_0_0_rgb(31,41,55),0_1px_2px_0_rgba(0,0,0,0.2)] dark:text-gray-200">
      {{ keyLabel }}
    </kbd>
    <span class="font-[450]">{{ label }}</span>
  </div>
</template>

<script setup>
defineProps({
  keyLabel: {
    type: String,
    required: true
  },
  label: {
    type: String,
    required: true
  }
})
</script>
