export default {
  // Common
  common: {
    create: '建立',
    cancel: '取消',
    delete: '刪除',
    confirmDelete: '確認刪除',
    remove: '移除',
    back: '返回',
    open: '開啟',
    exit: '離開',
    send: '送出',
    tryAgain: '重試'
  },

  // App
  app: {
    name: 'Mizly',
    tabs: {
      main: '主頁',
      stores: '資料集'
    }
  },

  // Keyboard hints
  keyboard: {
    aiChat: 'AI 對話',
    navigate: '瀏覽',
    select: '選擇',
    close: '關閉',
    exitAiChat: '離開 AI 對話'
  },

  // Search
  search: {
    placeholder: '搜尋...',
    askAi: '問 AI 任何問題...'
  },

  // Settings
  settings: {
    title: '設定',
    changeTheme: '深色模式',
    language: '切換語言',
    apiKey: 'API 金鑰',
    checkUpdate: '檢查更新'
  },

  // Updater
  updater: {
    checking: '正在檢查更新...',
    noUpdate: '已是最新版本',
    updateAvailable: '有新版本可用：v{version}',
    downloading: '正在下載更新...',
    downloadProgress: '下載中：{progress}%',
    installing: '正在安裝更新...',
    restartRequired: '需要重新啟動以完成更新',
    error: '檢查更新失敗'
  },

  // API Key settings
  apiKey: {
    title: 'API 金鑰',
    searchPlaceholder: '設定 API 金鑰...',
    configured: '已設定',
    notConfigured: '尚未設定',
    inputPlaceholder: '輸入您的 Gemini API 金鑰',
    updatePlaceholder: '輸入新的 API 金鑰以更新',
    inputHint: '您的 API 金鑰僅儲存在本機，不會傳送至我們的伺服器',
    save: '儲存',
    clear: '清除',
    getApiKey: '取得 API 金鑰',
    getApiKeyHint: '從 Google AI Studio 取得免費的 Gemini API 金鑰',
    invalidFormat: 'API 金鑰格式無效',
    saved: 'API 金鑰已儲存',
    cleared: 'API 金鑰已清除'
  },

  // Stores
  stores: {
    title: '資料集',
    searchPlaceholder: '搜尋資料集...',
    searchInStore: '在資料集中搜尋...',
    newStore: '新增資料集',
    enterName: '輸入資料集名稱',
    noStores: '尚無資料集。立即建立一個！',
    noFiles: '此資料集中沒有檔案',
    addFilesHint: '將檔案加入此資料集以進行管理'
  },

  // Files
  files: {
    selectFolder: '選擇資料夾',
    clickToChange: '點擊以變更資料夾',
    noFilesSelected: '尚未選擇檔案',
    selectFolderHint: '按 ⌘O 或點擊「選擇資料夾」來選擇目錄',
    subdirectoryInfo: '將包含子目錄（深度一層）中的檔案',
    noSizeInfo: '無大小資訊'
  },

  // Validation errors
  validation: {
    storeNameRequired: '請輸入資料集名稱',
    filesRequired: '請選擇至少一個檔案'
  },

  // AI Chat
  aiChat: {
    title: 'AI 對話',
    you: '你',
    assistant: 'AI 助理',
    thinking: '思考中...',
    askAnything: '詢問 AI 任何問題',
    typeQuestion: '輸入問題並按 Enter 送出',
    sources: '來源',
    somethingWentWrong: '發生錯誤',
    suggestQuestions: '推薦問題'
  },

  // Empty states
  empty: {
    noResults: '找不到結果',
    trySearchingElse: '試試搜尋其他內容'
  },

  // Navigation hints
  hints: {
    cancelHint: '按 Esc 取消',
    goBackHint: '按 Esc 返回'
  },

  // Locale settings
  locale: {
    title: '語言',
    searchPlaceholder: '搜尋語言...',
    current: '目前語系',
    languages: {
      en: '英文',
      'zh-TW': '繁體中文',
      ja: '日文'
    },
    nativeNames: {
      en: 'English',
      'zh-TW': '繁體中文',
      ja: '日本語'
    }
  }
}
