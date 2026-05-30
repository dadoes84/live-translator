import { createApp } from 'vue'
import { getCurrentWindow } from '@tauri-apps/api/window'
import App from './App.vue'
import AreaSelector from './components/AreaSelector.vue'
import './style.css'

const label = getCurrentWindow().label

if (label === 'area-selector') {
  createApp(AreaSelector).mount('#app')
} else {
  createApp(App).mount('#app')
}
