<template>
  <component
    :is="tag"
    :class="[
      'flex items-center gap-1 px-2 py-1 text-xs font-sans font-medium rounded-md',
      !static && 'cursor-pointer transition-colors',
      solid ? solidClasses : baseClasses,
      !static && !solid && variantClasses
    ]">
    <span>{{ label }}</span>
    <kbd v-if="shortcut" class="font-sans text-[10px] opacity-60">{{ shortcut }}</kbd>
  </component>
</template>

<script setup>
import { computed } from 'vue'

const props = defineProps({
  label: {
    type: String,
    required: true
  },
  shortcut: {
    type: String,
    default: ''
  },
  variant: {
    type: String,
    default: 'default',
    validator: (value) => ['default', 'primary', 'danger', 'ai'].includes(value)
  },
  static: {
    type: Boolean,
    default: false
  },
  solid: {
    type: Boolean,
    default: false
  },
  tag: {
    type: String,
    default: 'button'
  }
})

// Base classes (default appearance)
const baseClasses = 'text-gray-500 bg-white border border-gray-200 dark:bg-gray-700 dark:border-gray-600 dark:text-gray-400'

// Solid classes (always show variant color)
const solidClasses = computed(() => {
  switch (props.variant) {
    case 'primary':
      return 'bg-indigo-50 text-indigo-600 border border-indigo-200 dark:bg-indigo-900/30 dark:text-indigo-400 dark:border-indigo-800'
    case 'danger':
      return 'bg-red-50 text-red-600 border border-red-200 dark:bg-red-900/30 dark:text-red-400 dark:border-red-800'
    case 'ai':
      return 'bg-gradient-to-r from-indigo-500 to-purple-600 text-white border border-transparent'
    default:
      return 'bg-gray-100 text-gray-700 border border-gray-300 dark:bg-gray-900/30 dark:text-gray-400 dark:border-gray-800'
  }
})

// Hover classes (only applied when not solid)
const variantClasses = computed(() => {
  switch (props.variant) {
    case 'primary':
      return 'hover:bg-indigo-50 hover:text-indigo-600 hover:border-indigo-200 dark:hover:bg-indigo-900/30 dark:hover:text-indigo-400 dark:hover:border-indigo-800'
    case 'danger':
      return 'hover:bg-red-50 hover:text-red-600 hover:border-red-200 dark:hover:bg-red-900/30 dark:hover:text-red-400 dark:hover:border-red-800'
    case 'ai':
      return 'hover:bg-linear-to-r hover:from-indigo-500 hover:to-purple-600 hover:text-white hover:border-transparent'
    default:
      return 'hover:bg-gray-100 hover:text-gray-700 hover:border-gray-300 dark:hover:bg-gray-900/30 dark:hover:text-gray-400 dark:hover:border-gray-800'
  }
})
</script>
