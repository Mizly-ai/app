export default {
  // Common
  common: {
    create: 'Create',
    cancel: 'Cancel',
    delete: 'Delete',
    confirmDelete: 'Confirm',
    remove: 'Remove',
    back: 'Back',
    open: 'Open',
    exit: 'Exit',
    send: 'Send',
    tryAgain: 'Try Again'
  },

  // App
  app: {
    name: 'Mizly',
    tabs: {
      main: 'Main',
      stores: 'Stores'
    }
  },

  // Keyboard hints
  keyboard: {
    aiChat: 'AI Chat',
    navigate: 'Navigate',
    select: 'Select',
    close: 'Close',
    exitAiChat: 'Exit AI Chat'
  },

  // Search
  search: {
    placeholder: 'Search...',
    askAi: 'Ask AI anything...'
  },

  // Settings
  settings: {
    title: 'SETTINGS',
    changeTheme: 'Change Theme',
    language: 'Language',
    apiKey: 'API Key',
    checkUpdate: 'Check for Updates'
  },

  // Updater
  updater: {
    checking: 'Checking for updates...',
    noUpdate: 'You are using the latest version',
    updateAvailable: 'Update available: v{version}',
    downloading: 'Downloading update...',
    downloadProgress: 'Downloading: {progress}%',
    installing: 'Installing update...',
    restartRequired: 'Restart required to complete update',
    error: 'Update check failed'
  },

  // API Key settings
  apiKey: {
    title: 'API KEY',
    searchPlaceholder: 'Configure API key...',
    configured: 'Configured',
    notConfigured: 'Not Configured',
    inputPlaceholder: 'Enter your Gemini API key',
    updatePlaceholder: 'Enter new API key to update',
    inputHint: 'Your API key is stored locally and never sent to our servers',
    save: 'Save',
    clear: 'Clear',
    getApiKey: 'Get API Key',
    getApiKeyHint: 'Get your free Gemini API key from Google AI Studio',
    invalidFormat: 'Invalid API key format',
    saved: 'API key saved successfully',
    cleared: 'API key cleared'
  },

  // Stores
  stores: {
    title: 'Stores',
    searchPlaceholder: 'Search for stores...',
    searchInStore: 'Search in store...',
    newStore: 'New Store',
    enterName: 'Enter store name',
    noStores: 'No stores found. Create one!',
    noFiles: 'No files in this store',
    addFilesHint: 'Add files to organize them in this store'
  },

  // Files
  files: {
    selectFolder: 'Select Folder',
    clickToChange: 'Click to change folder',
    noFilesSelected: 'No Files Selected',
    selectFolderHint: 'Press ⌘O or click "Select Folder" to choose a directory',
    subdirectoryInfo: 'Files from subdirectories (1 level deep) will be included',
    noSizeInfo: 'No size info'
  },

  // Validation errors
  validation: {
    storeNameRequired: 'Please enter a store name',
    filesRequired: 'Please select at least one file'
  },

  // AI Chat
  aiChat: {
    title: 'AI Chat',
    you: 'You',
    assistant: 'AI Assistant',
    thinking: 'Thinking...',
    askAnything: 'Ask AI anything',
    typeQuestion: 'Type your question and press Enter to send',
    sources: 'Sources',
    somethingWentWrong: 'Something went wrong',
    suggestQuestions: 'Suggested Questions'
  },

  // Empty states
  empty: {
    noResults: 'No results found',
    trySearchingElse: 'Try searching for something else'
  },

  // Navigation hints
  hints: {
    cancelHint: 'Press Esc to cancel',
    goBackHint: 'Press Esc to go back'
  },

  // Locale settings
  locale: {
    title: 'LANGUAGE',
    searchPlaceholder: 'Search languages...',
    current: 'Current',
    languages: {
      en: 'English',
      'zh-TW': 'Traditional Chinese',
      ja: 'Japanese'
    },
    nativeNames: {
      en: 'English',
      'zh-TW': '繁體中文',
      ja: '日本語'
    }
  }
}
