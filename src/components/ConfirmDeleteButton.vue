<template>
  <div class="flex items-center gap-1">
    <!-- Initial Delete Button -->
    <ActionButton
      v-if="!isConfirming"
      @click.stop="startConfirm"
      :label="$t('common.delete')"
      :shortcut="deleteShortcut"
      variant="danger"
    />

    <!-- Confirmation Button -->
    <ActionButton
      v-else
      @click.stop="handleConfirm"
      :label="$t('common.confirmDelete')"
      :shortcut="confirmShortcut"
      variant="danger"
      :solid="true"
    />
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted, watch } from 'vue'
import ActionButton from '@/components/ActionButton.vue'
import { SHORTCUTS } from '@/utils/constants.js'

const props = defineProps({
  // Whether this button is currently active (selected in list)
  active: {
    type: Boolean,
    default: false
  },
  // Custom shortcuts (optional)
  deleteShortcut: {
    type: String,
    default: '⌘D'
  },
  confirmShortcut: {
    type: String,
    default: '⌘Y'
  }
})

const emit = defineEmits(['delete', 'confirm-start'])

const isConfirming = ref(false)

const startConfirm = () => {
  isConfirming.value = true
  emit('confirm-start')
}

const handleConfirm = () => {
  isConfirming.value = false
  emit('delete')
}

// Reset confirming state when active changes (e.g., selection moves away)
watch(() => props.active, (newActive) => {
  if (!newActive && isConfirming.value) {
    isConfirming.value = false
  }
})

// Handle keyboard shortcuts
const handleKeydown = (e) => {
  // Only handle if this button is active
  if (!props.active) return

  if (isConfirming.value) {
    // In confirmation mode
    if (e.key === SHORTCUTS.CONFIRM_DELETE && e.metaKey) {
      e.preventDefault()
      e.stopPropagation()
      handleConfirm()
    }
  } else {
    // In initial mode - trigger confirm on delete shortcut
    if (e.key === SHORTCUTS.DELETE && e.metaKey) {
      e.preventDefault()
      e.stopPropagation()
      startConfirm()
    }
  }
}

onMounted(() => {
  window.addEventListener('keydown', handleKeydown, true)
})

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeydown, true)
})

// Expose state and methods for parent component
defineExpose({
  isConfirming,
  startConfirm,
  handleConfirm
})
</script>
