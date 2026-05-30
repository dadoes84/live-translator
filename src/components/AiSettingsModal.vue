<template>
  <div v-if="visible" class="modal-overlay" @click.self="onCancel">
    <div class="modal">
      <div class="modal-header">
        <h3>设置</h3>
        <button class="modal-close" @click="onCancel">✕</button>
      </div>
      <div class="modal-body">
        <!-- 快捷键 -->
        <div class="form-section-title">快捷键</div>
        <div class="form-row">
          <div class="form-group flex-half">
            <label>开始 / 停止</label>
            <input
              class="input"
              type="text"
              :value="localShortcutToggle"
              readonly
              @keydown.prevent="(e: KeyboardEvent) => captureKey(e, 'toggle')"
              placeholder="点击后按键..."
            />
          </div>
          <div class="form-group flex-half">
            <label>区域设置</label>
            <input
              class="input"
              type="text"
              :value="localShortcutArea"
              readonly
              @keydown.prevent="(e: KeyboardEvent) => captureKey(e, 'area')"
              placeholder="点击后按键..."
            />
          </div>
        </div>

        <!-- AI 设置 -->
        <div class="form-section-title">AI 设置</div>

        <div class="form-group">
          <label>API Key</label>
          <input
            class="input"
            type="password"
            :value="localApiKey"
            @input="localApiKey = ($event.target as HTMLInputElement).value"
            placeholder="sk-..."
            autocomplete="off"
          />
        </div>

        <div class="form-group">
          <label>模型</label>
          <select class="select" v-model="localModel">
            <option value="deepseek-v4-flash">deepseek-v4-flash</option>
            <option value="deepseek-v4-pro">deepseek-v4-pro</option>
          </select>
        </div>

        <div class="form-group">
          <label>Prompt（系统提示词）</label>
          <textarea
            :value="localPrompt"
            @input="localPrompt = ($event.target as HTMLTextAreaElement).value"
            placeholder="输入发送给 AI 的系统提示词..."
          ></textarea>
        </div>

        <div v-if="testResult" :class="['test-result', testStatus]">
          {{ testResult }}
        </div>

        <button
          class="btn btn-small"
          :disabled="testing"
          @click="onTest"
        >
          {{ testing ? '检测中...' : '🔍 检测 API 设置' }}
        </button>
      </div>
      <div class="modal-footer">
        <button class="btn" @click="onCancel">取消</button>
        <button class="btn btn-primary" @click="onSave">保存</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const props = defineProps<{
  visible: boolean
  apiKey: string
  model: string
  prompt: string
  shortcutToggle: string
  shortcutArea: string
}>()

const emit = defineEmits<{
  (e: 'update:visible', v: boolean): void
  (e: 'save', payload: {
    apiKey: string
    model: string
    prompt: string
    shortcutToggle: string
    shortcutArea: string
  }): void
}>()

const localApiKey = ref(props.apiKey)
const localModel = ref(props.model)
const localPrompt = ref(props.prompt)
const localShortcutToggle = ref(props.shortcutToggle)
const localShortcutArea = ref(props.shortcutArea)

const testing = ref(false)
const testResult = ref('')
const testStatus = ref<'success' | 'fail' | 'loading'>('loading')

watch(() => [props.apiKey, props.model, props.prompt, props.shortcutToggle, props.shortcutArea],
  ([key, mdl, prm, st, sa]) => {
    localApiKey.value = key
    localModel.value = mdl
    localPrompt.value = prm
    localShortcutToggle.value = st
    localShortcutArea.value = sa
  })

// 按键捕获
function captureKey(e: KeyboardEvent, target: 'toggle' | 'area') {
  const parts: string[] = []
  if (e.ctrlKey) parts.push('Ctrl')
  if (e.altKey) parts.push('Alt')
  if (e.shiftKey) parts.push('Shift')
  if (e.metaKey) parts.push('Meta')

  const key = e.key
  // 忽略单独按下的修饰键
  if (['Control', 'Alt', 'Shift', 'Meta'].includes(key)) return

  // 主键：字母大写，其他保持原样（如 F1, Escape, Tab）
  const mainKey = key.length === 1 ? key.toUpperCase() : key
  parts.push(mainKey)

  const combo = parts.join('+')
  if (target === 'toggle') {
    localShortcutToggle.value = combo
  } else {
    localShortcutArea.value = combo
  }
}

function onCancel() {
  testResult.value = ''
  emit('update:visible', false)
}

function onSave() {
  testResult.value = ''
  emit('save', {
    apiKey: localApiKey.value,
    model: localModel.value,
    prompt: localPrompt.value,
    shortcutToggle: localShortcutToggle.value,
    shortcutArea: localShortcutArea.value,
  })
  emit('update:visible', false)
}

async function onTest() {
  if (!localApiKey.value.trim()) {
    testResult.value = '请先填写 API Key'
    testStatus.value = 'fail'
    return
  }

  testing.value = true
  testResult.value = '正在检测...'
  testStatus.value = 'loading'

  try {
    const result = await invoke<string>('test_api_connection', {
      apiKey: localApiKey.value,
      model: localModel.value,
    })
    testResult.value = result
    testStatus.value = 'success'
  } catch (err: any) {
    testResult.value = typeof err === 'string' ? err : (err.message ?? '连接失败')
    testStatus.value = 'fail'
  } finally {
    testing.value = false
  }
}
</script>
