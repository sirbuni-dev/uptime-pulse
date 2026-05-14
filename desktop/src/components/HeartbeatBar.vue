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
    <div v-if="tooltip.visible" class="hb-tooltip" :style="tooltipStyle">
      <div>{{ tooltip.time }}</div>
      <div v-if="tooltip.msg" class="hb-tooltip-msg">{{ tooltip.msg }}</div>
      <div>{{ tooltip.ping !== null ? tooltip.ping + 'ms' : '' }}</div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted, onUnmounted } from 'vue'
import type { Heartbeat } from '../stores/monitors'
import { DOWN, UP, PENDING } from '../stores/monitors'

const props = defineProps<{ beats: Heartbeat[] }>()

const BEAT_W   = 10
const BEAT_H   = 30
const BEAT_PAD = 4
const HOVER_SCALE = 1.5

const wrap       = ref<HTMLDivElement | null>(null)
const canvas     = ref<HTMLCanvasElement | null>(null)
const maxBeats   = ref(40)
const hoveredIdx = ref(-1)
const tooltip    = ref({ visible: false, time: '', msg: '', ping: null as number | null, x: 0 })

const visibleBeats = computed(() => {
  const n     = maxBeats.value
  const beats = [...props.beats].reverse().slice(0, n)
  while (beats.length < n) beats.unshift(null as any)
  return beats
})

const canvasWidth  = computed(() => visibleBeats.value.length * (BEAT_W + BEAT_PAD * 2))
const canvasHeight = computed(() => BEAT_H * HOVER_SCALE)

const tooltipStyle = computed(() => ({
  left:      `${tooltip.value.x}px`,
  top:       `${canvasHeight.value + 6}px`,
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
      tooltip.value = {
        visible: true,
        time:    beat.checkedAt,
        msg:     beat.msg ?? '',
        ping:    beat.ping,
        x:       idx * full + full / 2,
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
  overflow: hidden;
}

.hb-canvas {
  display: block;
  cursor: pointer;
}

.hb-tooltip {
  position: absolute;
  background: rgba(0, 0, 0, 0.8);
  color: #fff;
  padding: 4px 8px;
  border-radius: 6px;
  font-size: 11px;
  pointer-events: none;
  white-space: nowrap;
  z-index: 10;
}

.hb-tooltip-msg { color: #f8a306; }
</style>
