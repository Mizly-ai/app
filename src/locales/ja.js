export default {
  // Common
  common: {
    create: '作成',
    cancel: 'キャンセル',
    delete: '削除',
    confirmDelete: '確認',
    remove: '削除',
    back: '戻る',
    open: '開く',
    exit: '終了',
    send: '送信',
    tryAgain: '再試行'
  },

  // App
  app: {
    name: 'Mizly',
    tabs: {
      main: 'メイン',
      stores: 'ストア'
    }
  },

  // Keyboard hints
  keyboard: {
    aiChat: 'AIチャット',
    navigate: 'ナビゲート',
    select: '選択',
    close: '閉じる',
    exitAiChat: 'AIチャットを終了'
  },

  // Search
  search: {
    placeholder: '検索...',
    askAi: 'AIに何でも質問...'
  },

  // Settings
  settings: {
    title: '設定',
    changeTheme: 'ダークモード',
    language: '言語を変更',
    apiKey: 'APIキー',
    checkUpdate: 'アップデートを確認'
  },

  // Updater
  updater: {
    checking: 'アップデートを確認中...',
    noUpdate: '最新バージョンです',
    updateAvailable: '新しいバージョンがあります：v{version}',
    downloading: 'アップデートをダウンロード中...',
    downloadProgress: 'ダウンロード中：{progress}%',
    installing: 'アップデートをインストール中...',
    restartRequired: 'アップデートを完了するには再起動が必要です',
    error: 'アップデートに失敗しました：{message}'
  },

  // API Key settings
  apiKey: {
    title: 'APIキー',
    searchPlaceholder: 'APIキーを設定...',
    configured: '設定済み',
    notConfigured: '未設定',
    inputPlaceholder: 'Gemini APIキーを入力',
    updatePlaceholder: '新しいAPIキーを入力して更新',
    inputHint: 'APIキーはローカルに保存され、サーバーには送信されません',
    save: '保存',
    clear: 'クリア',
    getApiKey: 'APIキーを取得',
    getApiKeyHint: 'Google AI StudioからGemini APIキーを無料で取得',
    invalidFormat: 'APIキーの形式が無効です',
    saved: 'APIキーを保存しました',
    cleared: 'APIキーをクリアしました'
  },

  // Stores
  stores: {
    title: 'ストア',
    searchPlaceholder: 'ストアを検索...',
    searchInStore: 'ストア内を検索...',
    newStore: '新規ストア',
    enterName: 'ストア名を入力',
    noStores: 'ストアがありません。作成してください！',
    noFiles: 'このストアにファイルがありません',
    addFilesHint: 'ファイルを追加して整理しましょう'
  },

  // Files
  files: {
    selectFolder: 'フォルダを選択',
    clickToChange: 'クリックしてフォルダを変更',
    noFilesSelected: 'ファイルが選択されていません',
    selectFolderHint: '⌘O を押すか「フォルダを選択」をクリックしてディレクトリを選択',
    subdirectoryInfo: 'サブディレクトリ（1階層）のファイルも含まれます',
    noSizeInfo: 'サイズ情報なし'
  },

  // Validation errors
  validation: {
    storeNameRequired: 'ストア名を入力してください',
    filesRequired: 'ファイルを1つ以上選択してください'
  },

  // AI Chat
  aiChat: {
    title: 'AIチャット',
    you: 'あなた',
    assistant: 'AIアシスタント',
    thinking: '考え中...',
    askAnything: 'AIに何でも質問',
    typeQuestion: '質問を入力してEnterで送信',
    sources: '出典',
    somethingWentWrong: 'エラーが発生しました',
    suggestQuestions: 'おすすめの質問'
  },

  // Empty states
  empty: {
    noResults: '結果が見つかりません',
    trySearchingElse: '他のキーワードで検索してみてください'
  },

  // Navigation hints
  hints: {
    cancelHint: 'Esc でキャンセル',
    goBackHint: 'Esc で戻る'
  },

  // Locale settings
  locale: {
    title: '言語',
    searchPlaceholder: '言語を検索...',
    current: '現在',
    languages: {
      en: '英語',
      'zh-TW': '繁体字中国語',
      ja: '日本語'
    },
    nativeNames: {
      en: 'English',
      'zh-TW': '繁體中文',
      ja: '日本語'
    }
  }
}
