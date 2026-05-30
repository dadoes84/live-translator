<template>
  <div class="app-container" @contextmenu.prevent>
    <!-- ===== 标题栏 ===== -->
    <div class="titlebar" data-tauri-drag-region @contextmenu.prevent>
      <span class="titlebar-title">实时翻译器</span>
      <div class="titlebar-actions">
        <button
          class="titlebar-btn pin-btn"
          :class="{ 'pin-active': isPinned }"
          @click="togglePin"
          :title="isPinned ? '取消置顶' : '置顶'"
        >
          <svg class="pin-icon" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
            <path d="M16.5 10.5V6.75a4.5 4.5 0 1 0-9 0v3.75m-.75 11.25h10.5a2.25 2.25 0 0 0 2.25-2.25v-6.75a2.25 2.25 0 0 0-2.25-2.25H6.75a2.25 2.25 0 0 0-2.25 2.25v6.75a2.25 2.25 0 0 0 2.25 2.25Z"/>
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
            <span class="shortcut-hint">({{ shortcutToggle }})</span>
          </button>
          <button class="btn btn-area" :disabled="isRunning" @click="openAreaSelector">
            区域设置 <span class="shortcut-hint">({{ shortcutArea }})</span>
          </button>
          <button class="btn btn-icon" @click="showAiSettings = true" title="设置">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
              <path d="M10.343 3.94c.09-.542.56-.94 1.11-.94h1.093c.55 0 1.02.398 1.11.94l.149.894c.07.424.384.764.78.93.398.164.855.142 1.205-.108l.737-.527a1.125 1.125 0 0 1 1.45.12l.773.774c.39.389.44 1.002.12 1.45l-.527.737c-.25.35-.272.806-.107 1.204.165.397.505.71.93.78l.893.15c.543.09.94.559.94 1.109v1.094c0 .55-.397 1.02-.94 1.11l-.894.149c-.424.07-.764.383-.929.78-.165.398-.143.854.107 1.204l.527.738c.32.447.269 1.06-.12 1.45l-.774.773a1.125 1.125 0 0 1-1.449.12l-.738-.527c-.35-.25-.806-.272-1.203-.107-.398.165-.71.505-.781.929l-.149.894c-.09.542-.56.94-1.11.94h-1.094c-.55 0-1.019-.398-1.11-.94l-.148-.894c-.071-.424-.384-.764-.781-.93-.398-.164-.854-.142-1.204.108l-.738.527c-.447.32-1.06.269-1.45-.12l-.773-.774a1.125 1.125 0 0 1-.12-1.45l.527-.737c.25-.35.272-.806.108-1.204-.165-.397-.506-.71-.93-.78l-.894-.15c-.542-.09-.94-.56-.94-1.109v-1.094c0-.55.398-1.02.94-1.11l.894-.149c.424-.07.765-.383.93-.78.165-.398.143-.854-.108-1.204l-.526-.738a1.125 1.125 0 0 1 .12-1.45l.773-.773a1.125 1.125 0 0 1 1.45-.12l.737.527c.35.25.807.272 1.204.107.397-.165.71-.505.78-.929l.15-.894Z"/>
              <path d="M15 12a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z"/>
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
      <button
        v-if="translationResult"
        class="copy-btn"
        @click="copyTranslation"
        title="复制翻译内容"
      >
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <rect x="9" y="9" width="13" height="13" rx="2" ry="2"/>
          <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"/>
        </svg>
      </button>
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

async function onAiSettingsSave(payload: { apiKey: string; model: string; prompt: string; shortcutToggle: string; shortcutArea: string }) {
  // 保存旧值用于注销
  const oldToggle = shortcutToggle.value
  const oldArea = shortcutArea.value

  apiKey.value = payload.apiKey
  model.value = payload.model
  prompt.value = payload.prompt
  shortcutToggle.value = payload.shortcutToggle
  shortcutArea.value = payload.shortcutArea

  // 用旧值注销，新值注册
  try { if (oldToggle && oldToggle !== shortcutToggle.value) await unregister(oldToggle) } catch {}
  try { if (oldArea && oldArea !== shortcutArea.value) await unregister(oldArea) } catch {}
  registerShortcuts()
  scheduleSave()
}

// ===== 翻译结果 =====
const translationResult = ref('')

async function copyTranslation() {
  try {
    await navigator.clipboard.writeText(translationResult.value)
  } catch {
    // fallback
  }
}

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
