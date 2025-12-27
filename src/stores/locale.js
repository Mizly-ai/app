import { defineStore } from 'pinia'
import { ref } from 'vue'
import { load, Store } from '@tauri-apps/plugin-store'
import i18n from '@/config/i18n'
import { DEFAULT_LOCALE, SUPPORTED_LOCALES } from '@/config/i18n'

const STORE_NAME = 'settings.json'
const LOCALE_KEY = 'locale'

export const useLocaleStore = defineStore('locale', () => {
  const currentLocale = ref(DEFAULT_LOCALE)
  const isInitialized = ref(false)
  let store = null

  const initialize = async () => {
    if (isInitialized.value) return

    try {
      store = await load(STORE_NAME)
      const savedLocale = await store.get(LOCALE_KEY)

      if (savedLocale && SUPPORTED_LOCALES.some(l => l.code === savedLocale)) {
        currentLocale.value = savedLocale
        i18n.global.locale.value = savedLocale
      }
    } catch (e) {
      console.warn('Failed to load locale preference:', e)
    }

    isInitialized.value = true
  }

  const setLocale = async (locale) => {
    if (!SUPPORTED_LOCALES.some(l => l.code === locale)) {
      console.warn('Unsupported locale:', locale)
      return
    }

    currentLocale.value = locale
    i18n.global.locale.value = locale

    try {
      if (!store) {
        store = await load(STORE_NAME)
      }
      await store.set(LOCALE_KEY, locale)
      await store.save()
    } catch (e) {
      console.warn('Failed to save locale preference:', e)
    }
  }

  return {
    currentLocale,
    isInitialized,
    initialize,
    setLocale,
    supportedLocales: SUPPORTED_LOCALES
  }
})
