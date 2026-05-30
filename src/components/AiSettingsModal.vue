<template>
  <div v-if="visible" class="modal-overlay" @click.self="onCancel">
    <div class="modal">
      <div class="modal-header">
        <h3>AI 设置</h3>
        <button class="modal-close" @click="onCancel">✕</button>
      </div>
      <div class="modal-body">
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
}>()

const emit = defineEmits<{
  (e: 'update:visible', v: boolean): void
  (e: 'save', payload: { apiKey: string; model: string; prompt: string }): void
}>()

const localApiKey = ref(props.apiKey)
const localModel = ref(props.model)
const localPrompt = ref(props.prompt)

const testing = ref(false)
const testResult = ref('')
const testStatus = ref<'success' | 'fail' | 'loading'>('loading')

watch(() => [props.apiKey, props.model, props.prompt], ([key, mdl, prm]) => {
  localApiKey.value = key
  localModel.value = mdl
  localPrompt.value = prm
})

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
