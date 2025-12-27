import { createI18n } from 'vue-i18n'
import en from '@/locales/en.js'
import zhTW from '@/locales/zh-TW.js'
import ja from '@/locales/ja.js'

export const SUPPORTED_LOCALES = [
  { code: 'en', name: 'English', nativeName: 'English' },
  { code: 'zh-TW', name: 'Traditional Chinese', nativeName: '繁體中文' },
  { code: 'ja', name: 'Japanese', nativeName: '日本語' }
]

export const DEFAULT_LOCALE = 'en'

const i18n = createI18n({
  legacy: false,
  locale: DEFAULT_LOCALE,
  fallbackLocale: DEFAULT_LOCALE,
  messages: {
    en,
    'zh-TW': zhTW,
    ja
  }
})

export default i18n
