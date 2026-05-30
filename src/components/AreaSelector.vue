<template>
  <div
    class="area-selector-root"
    :class="{ idle: phase === 'idle' }"
    ref="rootRef"
  >
    <canvas
      ref="canvasRef"
      class="overlay-canvas"
      :class="{ idle: phase === 'idle' }"
      @mousedown="onMouseDown"
      @mousemove="onMouseMove"
      @mouseup="onMouseUp"
    />
    <div
      v-if="phase === 'selected' && selection"
      class="toolbar"
      :style="toolbarStyle"
      @mousedown.stop
      @click.stop
    >
      <span class="dim">{{ selection.width }} x {{ selection.height }}</span>
      <button class="btn-cancel" @click.stop="cancel">取消</button>
      <button class="btn-confirm" @click.stop="confirm">确认</button>
    </div>
    <div v-if="phase === 'idle'" class="hint">
      拖拽鼠标选取需要翻译的屏幕区域
    </div>
    <div class="shortcut-hint">ESC 取消 · Enter 确认</div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, nextTick } from 'vue'
import { invoke } from '@tauri-apps/api/core'

type Phase = 'idle' | 'dragging' | 'selected'

const rootRef = ref<HTMLElement>()
const canvasRef = ref<HTMLCanvasElement>()
const phase = ref<Phase>('idle')
const startX = ref(0)
const startY = ref(0)
const currentX = ref(0)
const currentY = ref(0)

interface Rect {
  x: number
  y: number
  width: number
  height: number
}

const selection = ref<Rect | null>(null)

function normalizedRect(): Rect | null {
  if (phase.value === 'idle') return null
  const x = Math.min(startX.value, currentX.value)
  const y = Math.min(startY.value, currentY.value)
  const w = Math.abs(currentX.value - startX.value)
  const h = Math.abs(currentY.value - startY.value)
  return { x, y, width: w, height: h }
}

function getCtx(): CanvasRenderingContext2D | null {
  return canvasRef.value?.getContext('2d') ?? null
}

function resizeCanvas() {
  const canvas = canvasRef.value
  if (!canvas) return
  const dpr = window.devicePixelRatio
  canvas.width = window.innerWidth * dpr
  canvas.height = window.innerHeight * dpr
  canvas.style.width = window.innerWidth + 'px'
  canvas.style.height = window.innerHeight + 'px'
  const ctx = canvas.getContext('2d')
  if (ctx) ctx.scale(dpr, dpr)
  draw()
}

function draw() {
  const ctx = getCtx()
  if (!ctx) return

  ctx.save()
  ctx.setTransform(window.devicePixelRatio, 0, 0, window.devicePixelRatio, 0, 0)
  ctx.clearRect(0, 0, window.innerWidth, window.innerHeight)

  if (phase.value === 'idle') {
    ctx.fillStyle = 'rgba(0, 0, 0, 0.08)'
    ctx.fillRect(0, 0, window.innerWidth, window.innerHeight)
    ctx.restore()
    return
  }

  const rect = normalizedRect()
  if (!rect || rect.width < 2 || rect.height < 2) {
    ctx.restore()
    return
  }

  ctx.fillStyle = 'rgba(0, 0, 0, 0.4)'
  ctx.fillRect(0, 0, window.innerWidth, window.innerHeight)
  ctx.clearRect(rect.x, rect.y, rect.width, rect.height)
  ctx.strokeStyle = '#6b9e85'
  ctx.lineWidth = 2
  ctx.strokeRect(rect.x, rect.y, rect.width, rect.height)
  drawCorners(ctx, rect)
  ctx.restore()
}

function drawCorners(ctx: CanvasRenderingContext2D, r: Rect) {
  const size = 8
  ctx.fillStyle = '#6b9e85'
  const corners = [
    { x: r.x, y: r.y },
    { x: r.x + r.width, y: r.y },
    { x: r.x, y: r.y + r.height },
    { x: r.x + r.width, y: r.y + r.height },
  ]
  for (const c of corners) {
    ctx.fillRect(c.x - size / 2, c.y - size / 2, size, size)
  }
}

function onMouseDown(e: MouseEvent) {
  if (e.button !== 0) return
  startX.value = e.clientX
  startY.value = e.clientY
  currentX.value = e.clientX
  currentY.value = e.clientY
  phase.value = 'dragging'
  selection.value = null
  draw()
}

function onMouseMove(e: MouseEvent) {
  if (phase.value !== 'dragging') return
  currentX.value = Math.min(Math.max(e.clientX, 0), window.innerWidth)
  currentY.value = Math.min(Math.max(e.clientY, 0), window.innerHeight)
  draw()
}

function onMouseUp(_e: MouseEvent) {
  if (phase.value !== 'dragging') return
  const rect = normalizedRect()
  if (rect && rect.width >= 5 && rect.height >= 5) {
    selection.value = rect
    phase.value = 'selected'
  } else {
    phase.value = 'idle'
    selection.value = null
  }
  draw()
}

const toolbarStyle = computed(() => {
  if (!selection.value) return {}
  const r = selection.value
  const top = r.y + r.height + 8 > window.innerHeight - 36 ? r.y - 36 : r.y + r.height + 8
  const left = Math.max(8, Math.min(r.x + r.width / 2 - 75, window.innerWidth - 170))
  return { top: top + 'px', left: left + 'px' }
})

function onKeyDown(e: KeyboardEvent) {
  if (e.key === 'Escape') {
    cancel()
  } else if (e.key === 'Enter' && phase.value === 'selected') {
    confirm()
  }
}

async function confirm() {
  const rect = selection.value
  if (!rect) return
  await invoke('confirm_area_selector', {
    x: Math.round(rect.x),
    y: Math.round(rect.y),
    width: Math.round(rect.width),
    height: Math.round(rect.height),
  })
  phase.value = 'idle'
  selection.value = null
}

async function cancel() {
  await invoke('cancel_area_selector')
  phase.value = 'idle'
  selection.value = null
}

let resizeObs: ResizeObserver | null = null

onMounted(async () => {
  await nextTick()
  resizeCanvas()
  resizeObs = new ResizeObserver(() => resizeCanvas())
  if (rootRef.value) resizeObs.observe(rootRef.value)
  window.addEventListener('keydown', onKeyDown)
})

onUnmounted(() => {
  resizeObs?.disconnect()
  window.removeEventListener('keydown', onKeyDown)
})
</script>

<style>
html, body, #app {
  background: transparent !important;
}
</style>

<style scoped>
.area-selector-root {
  position: fixed;
  inset: 0;
  outline: none;
}
.overlay-canvas {
  position: absolute;
  inset: 0;
  display: block;
}
.overlay-canvas.idle {
  cursor: crosshair;
}
.toolbar {
  position: fixed;
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 4px 8px;
  background: #2c2c2c;
  border-radius: 6px;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.4);
  z-index: 10;
  user-select: none;
}
.dim {
  color: #aaa;
  font-size: 12px;
  white-space: nowrap;
  margin-right: 4px;
}
.btn-cancel, .btn-confirm {
  border: none;
  border-radius: 4px;
  padding: 4px 14px;
  font-size: 13px;
  cursor: pointer;
  white-space: nowrap;
}
.btn-cancel { background: #555; color: #ddd; }
.btn-cancel:hover { background: #666; }
.btn-confirm { background: #6b9e85; color: white; }
.btn-confirm:hover { background: #5a8a72; }
.hint {
  position: fixed;
  top: 16px;
  left: 50%;
  transform: translateX(-50%);
  color: rgba(255, 255, 255, 0.9);
  font-size: 16px;
  text-shadow: 0 1px 4px rgba(0, 0, 0, 0.6);
  pointer-events: none;
  z-index: 5;
}
.shortcut-hint {
  position: fixed;
  bottom: 12px;
  right: 16px;
  color: rgba(255, 255, 255, 0.5);
  font-size: 12px;
  text-shadow: 0 1px 3px rgba(0, 0, 0, 0.4);
  pointer-events: none;
  z-index: 5;
}
</style>
