<template>
  <div class="app-container">
    <!-- ===== 标题栏 ===== -->
    <div class="titlebar" data-tauri-drag-region>
      <span class="titlebar-title">实时翻译器</span>
      <div class="titlebar-actions">
        <button
          class="titlebar-btn pin-btn"
          :class="{ 'pin-active': isPinned }"
          @click="togglePin"
          :title="isPinned ? '取消置顶' : '置顶'"
        >
          <svg class="pin-icon" width="14" height="14" viewBox="0 0 16 16" fill="none">
            <path d="M9.828 4.172a4 4 0 0 1 5.656 5.656l-1.414 1.414a1 1 0 0 1-1.414 0L7.07 6.656a1 1 0 0 1 0-1.414l1.414-1.414a4 4 0 0 1 1.344-.93z" fill="currentColor"/>
            <path d="M2.343 13.657l2-6 4 4-6 2z" fill="currentColor"/>
          </svg>
        </button>
        <button class="titlebar-btn" @click="minimizeWindow" title="最小化">─</button>
        <button class="titlebar-btn close-btn" @click="closeWindow" title="关闭">✕</button>
      </div>
    </div>

    <!-- ===== 设置区域 ===== -->
    <div class="settings-area">
      <!-- 第一行：按钮 + 选区信息 -->
      <div class="settings-row">
        <div class="settings-left">
          <button
            class="btn btn-primary btn-start"
            :class="{ running: isRunning }"
            @click="toggleRunning"
          >
            {{ isRunning ? '停止' : '开始' }}
            <span class="shortcut-hint" v-if="!isRunning">({{ shortcutToggle }})</span>
          </button>
          <button class="btn btn-area" @click="openAreaSelector">
            区域设置 <span class="shortcut-hint">({{ shortcutArea }})</span>
          </button>
          <button class="btn btn-icon" @click="showAiSettings = true" title="设置">
            <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
              <path d="M8 1a1.5 1.5 0 0 0-1.34.83L6.15 2.8a6.87 6.87 0 0 0-1.03.4l-1.8-.45a1.5 1.5 0 0 0-1.45.43l-.47.47a1.5 1.5 0 0 0-.43 1.45l.45 1.8a6.87 6.87 0 0 0-.4 1.03l-1.17.51A1.5 1.5 0 0 0 0 9.34v.66a1.5 1.5 0 0 0 .83 1.34l1.17.51c.1.35.24.7.4 1.03l-.45 1.8a1.5 1.5 0 0 0 .43 1.45l.47.47a1.5 1.5 0 0 0 1.45.43l1.8-.45c.33.16.68.3 1.03.4l.51 1.17A1.5 1.5 0 0 0 8 16h.66a1.5 1.5 0 0 0 1.34-.83l.51-1.17c.35-.1.7-.24 1.03-.4l1.8.45a1.5 1.5 0 0 0 1.45-.43l.47-.47a1.5 1.5 0 0 0 .43-1.45l-.45-1.8c.16-.33.3-.68.4-1.03l1.17-.51A1.5 1.5 0 0 0 16 8v-.66a1.5 1.5 0 0 0-.83-1.34l-1.17-.51a6.87 6.87 0 0 0-.4-1.03l.45-1.8a1.5 1.5 0 0 0-.43-1.45l-.47-.47a1.5 1.5 0 0 0-1.45-.43l-1.8.45c-.33-.16-.68-.3-1.03-.4L9.34 1.83A1.5 1.5 0 0 0 8 1H8zm0 4.5A2.5 2.5 0 1 1 5.5 8 2.5 2.5 0 0 1 8 5.5z"/>
            </svg>
          </button>
        </div>
      </div>

      <!-- 第二行：下拉菜单 -->
      <div class="settings-row">
        <div class="settings-label-group">
          <label>扫描频率</label>
          <select class="select" v-model.number="scanFrequency">
            <option :value="1">1次/秒</option>
            <option :value="2">2次/秒</option>
            <option :value="3">3次/秒</option>
            <option :value="4">4次/秒</option>
            <option :value="5">5次/秒</option>
          </select>
        </div>
        <div class="settings-label-group">
          <label>OCR 语言</label>
          <select class="select" v-model="ocrLanguage">
            <option value="ja">日文</option>
            <option value="en">英文</option>
          </select>
        </div>
        <div class="settings-label-group">
          <label>字体</label>
          <select class="select" v-model="fontFamily">
            <option value="Microsoft YaHei">微软雅黑</option>
            <option value="SimSun">宋体</option>
            <option value="KaiTi">楷体</option>
            <option value="Arial">Arial</option>
            <option value="Segoe UI">Segoe UI</option>
          </select>
        </div>
        <div class="settings-label-group">
          <label>字号</label>
          <select class="select" v-model.number="fontSize">
            <option :value="12">12</option>
            <option :value="14">14</option>
            <option :value="16">16</option>
            <option :value="18">18</option>
            <option :value="20">20</option>
            <option :value="24">24</option>
            <option :value="28">28</option>
            <option :value="32">32</option>
            <option :value="36">36</option>
            <option :value="40">40</option>
          </select>
        </div>
        <div class="settings-label-group">
          <label>文字颜色</label>
          <input type="color" class="color-picker" v-model="textColor" />
        </div>
        <div class="settings-label-group">
          <label>背景颜色</label>
          <input type="color" class="color-picker" v-model="bgColor" />
        </div>
      </div>
    </div>

    <!-- ===== 翻译显示区域 ===== -->
    <div
      class="translation-area"
      :style="{
        fontFamily: fontFamily,
        fontSize: fontSize + 'px',
        color: textColor,
        backgroundColor: bgColor,
      }"
    >
      <div v-if="!translationResult" class="translation-placeholder">
        暂无翻译内容
      </div>
      <div v-else class="translation-content">
        {{ translationResult }}
      </div>
    </div>

    <!-- ===== 状态栏 ===== -->
    <div class="statusbar">
      <span :class="['status-text', statusClass]">{{ statusText }}</span>
      <span class="status-region">
        当前选区：
        <template v-if="region">
          ({{ region.x }}, {{ region.y }}) {{ region.width }}×{{ region.height }}
        </template>
        <template v-else>未设置</template>
      </span>
    </div>

    <!-- ===== AI 设置弹窗 ===== -->
    <AiSettingsModal
      :visible="showAiSettings"
      :apiKey="apiKey"
      :model="model"
      :prompt="prompt"
      :shortcutToggle="shortcutToggle"
      :shortcutArea="shortcutArea"
      @update:visible="showAiSettings = $event"
      @save="onAiSettingsSave"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted, onUnmounted } from 'vue'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { WebviewWindow } from '@tauri-apps/api/webviewWindow'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { register, unregister } from '@tauri-apps/plugin-global-shortcut'
import type { UnlistenFn } from '@tauri-apps/api/event'
import AiSettingsModal from './components/AiSettingsModal.vue'

// ===== 窗口引用 =====
const appWindow = getCurrentWindow()

// ===== 窗口控制 =====
const isPinned = ref(false)

async function togglePin() {
  isPinned.value = !isPinned.value
  await appWindow.setAlwaysOnTop(isPinned.value)
}

function minimizeWindow() {
  appWindow.minimize()
}

function closeWindow() {
  appWindow.close()
}

// ===== 区域 =====
interface Region {
  x: number
  y: number
  width: number
  height: number
}

const region = ref<Region | null>(null)

let unlistenArea: UnlistenFn | null = null

async function openAreaSelector() {
  try {
    const selector = await WebviewWindow.getByLabel('area-selector')
    if (selector) {
      await selector.show()
      await selector.setFocus()
      await appWindow.hide()
    } else {
      statusText.value = '选区窗口未找到'
    }
  } catch (err) {
    console.error('打开区域选择器失败:', err)
  }
}

// ===== 设置项 =====
const scanFrequency = ref(3)
const ocrLanguage = ref('ja')
const fontFamily = ref('Microsoft YaHei')
const fontSize = ref(18)
const textColor = ref('#ffffff')
const bgColor = ref('#1a1a2e')

// ===== AI 设置 =====
const showAiSettings = ref(false)
const apiKey = ref('')
const model = ref('deepseek-v4-flash')
const prompt = ref('你是一个专业的本地化翻译器，只将用户输入的文本翻译成简体中文，不做任何解释，不添加额外输出。')
const shortcutToggle = ref('Ctrl+Shift+T')
const shortcutArea = ref('Ctrl+Shift+A')

function onAiSettingsSave(payload: { apiKey: string; model: string; prompt: string; shortcutToggle: string; shortcutArea: string }) {
  apiKey.value = payload.apiKey
  model.value = payload.model
  prompt.value = payload.prompt
  shortcutToggle.value = payload.shortcutToggle
  shortcutArea.value = payload.shortcutArea
  // 重新注册快捷键
  unregisterShortcuts()
  registerShortcuts()
  scheduleSave()
}

// ===== 翻译结果 =====
const translationResult = ref('')

// ===== 状态栏 =====
const statusText = ref('就绪')
const statusClass = computed(() => {
  if (statusText.value.includes('出错') || statusText.value.includes('失败')) return 'error'
  if (statusText.value.includes('翻译中') || statusText.value.includes('检测')) return 'warning'
  if (statusText.value === '运行中') return 'active'
  return ''
})

// ===== 诊断日志 =====
async function log(msg: string) {
  console.log(`[DBG] ${msg}`)
  try { await invoke('debug_log', { message: msg }) } catch {}
}

// ===== 全局快捷键 =====
async function registerShortcuts() {
  await log(`=== 开始注册快捷键 ===`)
  await log(`toggle shortcut: "${shortcutToggle.value}"`)
  await log(`area shortcut: "${shortcutArea.value}"`)

  try {
    if (shortcutToggle.value) {
      await log(`正在注册: ${shortcutToggle.value}`)
      await register(shortcutToggle.value, (ev) => {
        invoke('debug_log', { message: `[CALLBACK] toggle 触发, state=${ev.state}` }).catch(() => {})
        console.log(`[CALLBACK] toggle 触发, state=`, ev.state)
        if (ev.state === 'Pressed') {
          log('快捷键调用 toggleRunning()')
          toggleRunning()
        }
      })
      await log(`注册成功: ${shortcutToggle.value}`)
    }
  } catch (err: any) {
    const msg = `注册失败 (${shortcutToggle.value}): ${err}`
    await log(msg)
    console.error(msg, err)
  }

  try {
    if (shortcutArea.value) {
      await log(`正在注册: ${shortcutArea.value}`)
      await register(shortcutArea.value, (ev) => {
        invoke('debug_log', { message: `[CALLBACK] area 触发, state=${ev.state}` }).catch(() => {})
        console.log(`[CALLBACK] area 触发, state=`, ev.state)
        if (ev.state === 'Pressed') {
          log('快捷键调用 openAreaSelector()')
          openAreaSelector()
        }
      })
      await log(`注册成功: ${shortcutArea.value}`)
    }
  } catch (err: any) {
    const msg = `注册失败 (${shortcutArea.value}): ${err}`
    await log(msg)
    console.error(msg, err)
  }

  await log(`=== 快捷键注册完成 ===`)
}

async function unregisterShortcuts() {
  await log('注销全部快捷键')
  try { if (shortcutToggle.value) await unregister(shortcutToggle.value) } catch {}
  try { if (shortcutArea.value) await unregister(shortcutArea.value) } catch {}
}

// ===== OCR 轮询 =====
const isRunning = ref(false)
const stableDuration = 500
const minTranslationInterval = 1000

let loopTimer: number | null = null
let lastOcrText = ''
let stableSince: number | null = null
let lastTranslationTime = 0
let lastTranslatedText = ''

function toggleRunning() {
  if (isRunning.value) {
    stopLoop()
  } else {
    if (!region.value) {
      statusText.value = '请先设置翻译区域'
      return
    }
    startLoop()
  }
}

function startLoop() {
  isRunning.value = true
  lastOcrText = ''
  stableSince = null
  lastTranslationTime = 0
  lastTranslatedText = ''
  scheduleNextOcr()
}

function stopLoop() {
  isRunning.value = false
  if (loopTimer) {
    clearTimeout(loopTimer)
    loopTimer = null
  }
  statusText.value = '就绪'
}

function scheduleNextOcr() {
  if (!isRunning.value) return
  const interval = Math.round(1000 / scanFrequency.value)
  loopTimer = window.setTimeout(() => doOcr(), interval)
}

async function doOcr() {
  if (!isRunning.value || !region.value) return

  const dpr = window.devicePixelRatio
  const r = region.value

  try {
    const text: string = await invoke('capture_and_ocr', {
      x: Math.round(r.x * dpr),
      y: Math.round(r.y * dpr),
      width: Math.round(r.width * dpr),
      height: Math.round(r.height * dpr),
      language: ocrLanguage.value,
    })

    const trimmed = text.trim()
    const now = Date.now()

    if (trimmed.length === 0) {
      stableSince = null
      statusText.value = '检测中...'
    } else if (trimmed === lastOcrText) {
      if (stableSince === null) {
        stableSince = now
      }
      const stableFor = now - stableSince
      const sinceLastTranslation = now - lastTranslationTime

      if (
        stableFor >= stableDuration &&
        sinceLastTranslation >= minTranslationInterval &&
        trimmed !== lastTranslatedText
      ) {
        statusText.value = '翻译中...'
        try {
          const result: string = await invoke('translate', {
            text: trimmed,
            sourceLang: ocrLanguage.value,
          })
          translationResult.value = result
          lastTranslationTime = Date.now()
          lastTranslatedText = trimmed
          statusText.value = '运行中'
        } catch (err) {
          console.error('翻译失败:', err)
          statusText.value = '翻译出错'
        }
      } else {
        statusText.value = `检测到文本 (稳定${(stableFor / 1000).toFixed(1)}s)`
        if (trimmed === lastTranslatedText) {
          statusText.value += ' · 已翻译'
        }
      }
    } else {
      lastOcrText = trimmed
      stableSince = null
      statusText.value = '检测到文本变化...'
    }
  } catch (err) {
    console.error('OCR 失败:', err)
    statusText.value = 'OCR 识别失败'
  }

  scheduleNextOcr()
}

// ===== 配置持久化 =====
let saveTimer: number | null = null

function scheduleSave() {
  if (saveTimer) clearTimeout(saveTimer)
  saveTimer = window.setTimeout(() => {
    invoke('save_config', {
      config: {
        scan_frequency: scanFrequency.value,
        ocr_language: ocrLanguage.value,
        font_family: fontFamily.value,
        font_size: fontSize.value,
        text_color: textColor.value,
        bg_color: bgColor.value,
        is_pinned: isPinned.value,
        api_key: apiKey.value,
        model: model.value,
        prompt: prompt.value,
        shortcut_toggle: shortcutToggle.value,
        shortcut_area: shortcutArea.value,
      },
    }).catch((err) => console.error('配置保存失败:', err))
  }, 500)
}

const watchedRefs = [
  scanFrequency, ocrLanguage, fontFamily, fontSize,
  textColor, bgColor, isPinned, apiKey, model, prompt,
  shortcutToggle, shortcutArea,
]
watch(watchedRefs, () => scheduleSave(), { deep: true })

onMounted(async () => {
  // 加载配置
  try {
    const config: any = await invoke('load_config')
    scanFrequency.value = config.scan_frequency ?? 3
    ocrLanguage.value = config.ocr_language ?? 'ja'
    fontFamily.value = config.font_family ?? 'Microsoft YaHei'
    fontSize.value = config.font_size ?? 18
    textColor.value = config.text_color ?? '#ffffff'
    bgColor.value = config.bg_color ?? '#1a1a2e'
    isPinned.value = config.is_pinned ?? false
    apiKey.value = config.api_key ?? ''
    model.value = config.model ?? 'deepseek-v4-flash'
    prompt.value = config.prompt ?? '你是一个专业的本地化翻译器，只将用户输入的文本翻译成简体中文，不做任何解释，不添加额外输出。'
    shortcutToggle.value = config.shortcut_toggle ?? 'Ctrl+Shift+T'
    shortcutArea.value = config.shortcut_area ?? 'Ctrl+Shift+A'
    region.value = config.region ?? null
    if (isPinned.value) {
      await appWindow.setAlwaysOnTop(true)
    }
  } catch (err) {
    console.error('加载配置失败:', err)
    scheduleSave()
  }

  // 监听选区结果
  unlistenArea = await listen<Region>('area-selected', (event) => {
    region.value = event.payload
  })

  // 注册全局快捷键
  registerShortcuts()
})

onUnmounted(() => {
  unlistenArea?.()
  unregisterShortcuts()
  stopLoop()
})
</script>
