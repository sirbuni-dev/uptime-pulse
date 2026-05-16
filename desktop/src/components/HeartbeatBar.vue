<template>
  <div ref="wrap" class="hb-wrap">
    <canvas
      ref="canvas"
      class="hb-canvas"
      :width="canvasWidth"
      :height="canvasHeight"
      @mousemove="onMouseMove"
      @mouseleave="onMouseLeave"
    />
    <div class="hb-timeline">
      <span class="hb-label-now">now</span>
    </div>
    <div v-if="tooltip.visible" class="hb-tooltip" :style="tooltipStyle">
      <div class="hb-tooltip-status" :class="tooltip.statusClass">{{ tooltip.statusLabel }}</div>
      <div class="hb-tooltip-time">{{ tooltip.time }}</div>
      <div v-if="tooltip.msg" class="hb-tooltip-msg">{{ tooltip.msg }}</div>
      <div v-if="tooltip.ping !== null">{{ tooltip.ping }}ms</div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted, onUnmounted } from 'vue'
import type { Heartbeat } from '../stores/monitors'
import { DOWN, UP, PENDING } from '../stores/monitors'
import { formatDateTime } from '../utils/time'

const props = defineProps<{ beats: Heartbeat[] }>()

const BEAT_W   = 10
const BEAT_H   = 30
const BEAT_PAD = 4
const HOVER_SCALE = 1.5

const wrap       = ref<HTMLDivElement | null>(null)
const canvas     = ref<HTMLCanvasElement | null>(null)
const maxBeats   = ref(40)
const hoveredIdx = ref(-1)
const TIMELINE_H = 18  // height of .hb-timeline row in px

const tooltip    = ref({ visible: false, statusLabel: '', statusClass: '', time: '', msg: '', ping: null as number | null, clientX: 0, clientY: 0 })

const visibleBeats = computed(() => {
  const n     = maxBeats.value
  const beats = [...props.beats].slice(0, n).reverse()
  while (beats.length < n) beats.unshift(null as any)
  return beats
})

const canvasWidth  = computed(() => visibleBeats.value.length * (BEAT_W + BEAT_PAD * 2))
const canvasHeight = computed(() => BEAT_H * HOVER_SCALE)

const tooltipStyle = computed(() => ({
  left:      `${tooltip.value.clientX}px`,
  top:       `${tooltip.value.clientY}px`,
  transform: 'translateX(-50%)',
}))

function getColor(beat: Heartbeat | null) {
  if (!beat)                   return '#848484'
  if (beat.status === DOWN)    return '#dc3545'
  if (beat.status === PENDING) return '#f8a306'
  if (beat.status === UP)      return '#5cdd8b'
  return '#848484'
}

function drawCanvas() {
  const c = canvas.value
  if (!c) return
  const ctx = c.getContext('2d')!
  const dpr = window.devicePixelRatio || 1
  const full = BEAT_W + BEAT_PAD * 2
  const cy   = canvasHeight.value / 2

  c.width        = canvasWidth.value * dpr
  c.height       = canvasHeight.value * dpr
  c.style.width  = canvasWidth.value + 'px'
  c.style.height = canvasHeight.value + 'px'
  ctx.scale(dpr, dpr)
  ctx.clearRect(0, 0, canvasWidth.value, canvasHeight.value)

  visibleBeats.value.forEach((beat, i) => {
    const hovered = i === hoveredIdx.value && beat
    const w = hovered ? BEAT_W * HOVER_SCALE : BEAT_W
    const h = hovered ? BEAT_H * HOVER_SCALE : BEAT_H
    const x = i * full + BEAT_PAD - (w - BEAT_W) / 2
    const y = cy - h / 2
    const r = w / 2

    ctx.fillStyle = getColor(beat)
    ctx.beginPath()
    ctx.moveTo(x + r, y)
    ctx.lineTo(x + w - r, y)
    ctx.quadraticCurveTo(x + w, y,     x + w, y + r)
    ctx.lineTo(x + w, y + h - r)
    ctx.quadraticCurveTo(x + w, y + h, x + w - r, y + h)
    ctx.lineTo(x + r, y + h)
    ctx.quadraticCurveTo(x,     y + h, x, y + h - r)
    ctx.lineTo(x, y + r)
    ctx.quadraticCurveTo(x,     y,     x + r, y)
    ctx.closePath()
    ctx.fill()
  })
}

function onMouseMove(e: MouseEvent) {
  const c = canvas.value
  if (!c) return
  const rect = c.getBoundingClientRect()
  const x    = e.clientX - rect.left
  const full = BEAT_W + BEAT_PAD * 2
  const idx  = Math.floor(x / full)

  if (idx >= 0 && idx < visibleBeats.value.length) {
    const beat = visibleBeats.value[idx]
    hoveredIdx.value = idx
    if (beat) {
      const statusLabel = beat.status === UP ? 'UP' : beat.status === DOWN ? 'DOWN' : 'PENDING'
      const statusClass = beat.status === UP ? 'status-up' : beat.status === DOWN ? 'status-down' : 'status-pending'
      tooltip.value = {
        visible: true,
        statusLabel,
        statusClass,
        time:    formatDateTime(beat.checkedAt),
        msg:     beat.msg ?? '',
        ping:    beat.ping,
        clientX: e.clientX,
        clientY: rect.bottom + TIMELINE_H + 6,
      }
    } else {
      tooltip.value.visible = false
    }
  } else {
    hoveredIdx.value      = -1
    tooltip.value.visible = false
  }
}

function onMouseLeave() {
  hoveredIdx.value      = -1
  tooltip.value.visible = false
}

function resize() {
  if (wrap.value) maxBeats.value = Math.floor(wrap.value.clientWidth / (BEAT_W + BEAT_PAD * 2))
}

watch([visibleBeats, hoveredIdx], drawCanvas)
onMounted(() => { window.addEventListener('resize', resize); resize() })
onUnmounted(() => window.removeEventListener('resize', resize))
</script>

<style lang="scss" scoped>
.hb-wrap {
  position: relative;
  width: 100%;
  overflow-x: clip;
  overflow-y: visible;
}

.hb-canvas {
  display: block;
  cursor: pointer;
}

.hb-timeline {
  display: flex;
  justify-content: flex-end;
  padding-top: 3px;
  pointer-events: none;
}

.hb-label-now {
  font-size: 10px;
  color: var(--text4);
  line-height: 1;
}

.hb-tooltip {
  position: fixed;
  background: #1a2232;
  color: #dde3eb;
  padding: 8px 14px;
  border-radius: 8px;
  font-size: 12px;
  pointer-events: none;
  white-space: nowrap;
  z-index: 9999;
  text-align: center;
  line-height: 1.6;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.45);

  &::before {
    content: '';
    position: absolute;
    top: -13px;
    left: 50%;
    transform: translateX(-50%);
    border-left: 13px solid transparent;
    border-right: 13px solid transparent;
    border-bottom: 13px solid #1a2232;
  }
}

.hb-tooltip-status {
  font-weight: 700;
  font-size: 12px;
  margin-bottom: 2px;

  &.status-up      { color: #5cdd8b; }
  &.status-down    { color: #dc3545; }
  &.status-pending { color: #f8a306; }
}

.hb-tooltip-time {
  color: rgba(255, 255, 255, 0.55);
  font-size: 10px;
  font-family: monospace;
}

.hb-tooltip-msg { color: rgba(255, 255, 255, 0.85); }
</style>
