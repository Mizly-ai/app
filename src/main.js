import { createApp } from 'vue'
import { createPinia } from 'pinia'
import App from './App.vue'
import router from './config/routes'
import i18n from './config/i18n'
import './assets/stylesheets/main.tailwind.css'

const app = createApp(App)
const pinia = createPinia()

app.use(pinia)
app.use(router)
app.use(i18n)
app.mount('#app')
