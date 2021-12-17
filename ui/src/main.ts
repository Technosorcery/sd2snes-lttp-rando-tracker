import { createApp } from 'vue'
import { store, key } from './store'
import { router } from './router'
import App from './App.vue'
import './index.css'

const app = createApp(App)
app.use(store, key)
app.use(router)
app.mount('#app')
