# Frontend Architecture

This document describes the frontend architecture and coding conventions.

## Directory Structure

```
src/
├── assets/                 # Static assets (images, fonts, stylesheets)
│   └── stylesheets/
│       └── main.tailwind.css
├── components/             # Reusable UI components
│   ├── ActionButton.vue
│   ├── ConfirmDeleteButton.vue
│   └── KeyboardHint.vue
├── composables/            # Vue composables (reusable logic)
│   ├── useAiChat.js
│   ├── useFileSelection.js
│   ├── useGlobalKeyboard.js
│   ├── useHomeSettings.js
│   ├── useListNavigation.js
│   ├── useLocaleSelector.js
│   ├── useStoreDetail.js
│   ├── useStoreForm.js
│   ├── useStoreList.js
│   └── useWindowDrag.js
├── config/                 # Configuration files
│   ├── i18n.js
│   └── routes.js
├── locales/                # i18n translation files
│   ├── en.js
│   ├── ja.js
│   └── zh-TW.js
├── stores/                 # Pinia stores
│   ├── app.js
│   ├── auth.js
│   ├── chat.js
│   ├── locale.js
│   └── stores.js
├── utils/                  # Utility functions
│   ├── constants.js
│   ├── helpers.js
│   └── icons.js
├── views/                  # Page components
│   ├── chats/
│   │   └── show.vue
│   ├── home/
│   │   └── index.vue
│   ├── layout/
│   │   └── application.vue
│   ├── settings/
│   │   ├── api-key/
│   │   │   └── index.vue
│   │   └── locales/
│   │       └── index.vue
│   └── stores/
│       ├── index.vue
│       ├── new.vue
│       └── show.vue
├── App.vue                 # Root component
└── main.js                 # Application entry point
```

## Composables

Composables are the primary way to organize and reuse logic in this application.

### Available Composables

| Composable | File | Purpose |
|------------|------|---------|
| `useGlobalKeyboard` | `useGlobalKeyboard.js` | Handles global keyboard shortcuts when input is not focused |
| `useListNavigation` | `useListNavigation.js` | Provides list navigation (up/down), scroll behavior, and selection state |
| `useWindowDrag` | `useWindowDrag.js` | Handles window drag behavior for custom title bar |
| `useStoreList` | `useStoreList.js` | Manages stores list operations (load, filter, delete) |
| `useStoreDetail` | `useStoreDetail.js` | Manages single store detail (documents, open file/folder) |
| `useStoreForm` | `useStoreForm.js` | Manages store creation form validation and submission |
| `useFileSelection` | `useFileSelection.js` | Handles local file selection for uploads |
| `useAiChat` | `useAiChat.js` | Handles AI chat messaging and suggest questions |
| `useHomeSettings` | `useHomeSettings.js` | Manages home page settings items and actions |
| `useLocaleSelector` | `useLocaleSelector.js` | Handles language selection and filtering |

### Usage Pattern

```javascript
// In a Vue component
import { ref, computed } from 'vue'
import { useListNavigation } from '@/composables/useListNavigation'
import { useFeatureSpecific } from '@/composables/useFeatureSpecific'

const layoutRef = ref(null)

// Feature-specific composable
const {
  items,
  searchQuery,
  selectItem,
  deleteItem
} = useFeatureSpecific()

// Shared navigation composable
const itemCount = computed(() => items.value.length)
const {
  selectedIndex,
  navigateUp,
  navigateDown
} = useListNavigation({
  itemCount,
  layoutRef,
  searchQuery
})
```

### Creating a New Composable

1. Create file: `src/composables/use<Feature>.js`
2. Follow the standard pattern:

```javascript
import { ref, computed, onMounted, onUnmounted } from 'vue'

/**
 * Description of what this composable does
 *
 * @param {Object} options - Configuration options
 * @param {Ref} options.someOption - Description
 * @returns {Object} Public API
 */
export function useFeatureName(options = {}) {
  const { someOption } = options

  // State
  const state = ref(initialValue)

  // Computed
  const derivedState = computed(() => /* ... */)

  // Methods
  const doSomething = () => { /* ... */ }

  // Lifecycle (if needed)
  onMounted(() => { /* ... */ })
  onUnmounted(() => { /* ... */ })

  return {
    // State
    state,
    derivedState,
    // Methods
    doSomething
  }
}
```

## Views

Views are page-level components that combine:
- Template (UI structure with Tailwind CSS)
- Composables (business logic)
- Global keyboard handlers

### View Pattern

```vue
<template>
  <SearchLayout @navigate-up="navigateUp" @navigate-down="navigateDown" ref="layoutRef">
    <!-- Content -->
  </SearchLayout>
</template>

<script setup>
import { ref, computed } from 'vue'
import SearchLayout from '@/views/layout/application.vue'
import { useGlobalKeyboard } from '@/composables/useGlobalKeyboard'
import { useListNavigation } from '@/composables/useListNavigation'
import { useFeatureSpecific } from '@/composables/useFeatureSpecific'

const layoutRef = ref(null)

// 1. Feature-specific composable
const { items, searchQuery, ...methods } = useFeatureSpecific()

// 2. List navigation composable
const { selectedIndex, navigateUp, navigateDown } = useListNavigation({
  itemCount: computed(() => items.value.length),
  layoutRef,
  searchQuery
})

// 3. Global keyboard composable
const { onNavigateUp, onNavigateDown, onEnter } = useGlobalKeyboard({
  inputRef: computed(() => layoutRef.value?.searchInput)
})

// 4. Register handlers
onNavigateUp(navigateUp)
onNavigateDown(navigateDown)
onEnter(handleEnter)
</script>
```

## Stores (Pinia)

| Store | Purpose |
|-------|---------|
| `app` | Global app state (search query, AI chat mode) |
| `auth` | API key management |
| `chat` | AI chat state (messages, suggestions) |
| `stores` | Document stores state |
| `locale` | Language/locale settings |

## Styling Rules

- **NO `<style>` tags** in Vue components
- **Use Tailwind CSS** utility classes exclusively
- Common patterns are in `assets/stylesheets/main.tailwind.css`

## Icons

All icons are centralized in `utils/icons.js` and re-exported from `lucide-vue-next`:

```javascript
import { FolderIcon, FileIcon, LoadingSpinner } from '@/utils/icons.js'
```
