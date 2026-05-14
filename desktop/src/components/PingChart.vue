<template>
  <div ref="wrap" class="ping-chart">
    <canvas ref="canvas" class="ping-chart__canvas" />
    <div v-if="!hasPings" class="ping-chart__empty">No ping data yet</div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted, onUnmounted } from 'vue'
import type { Heartbeat } from '../stores/monitors'

const props = defineProps<{ beats: Heartbeat[]; dark?: boolean }>()

const wrap   = ref<HTMLDivElement | null>(null)
const canvas = ref<HTMLCanvasElement | null>(null)

const pings = computed(() =>
  [...props.beats].reverse().filter(b => b.ping != null).map(b => b.ping!)
)

const hasPings = computed(() => pings.value.length > 0)

const isDark = () => document.documentElement.classList.contains('dark')

function spline(ctx: CanvasRenderingContext2D, pts: [number, number][]) {
  if (pts.length < 2) return
  ctx.moveTo(pts[0][0], pts[0][1])
  for (let i = 0; i < pts.length - 1; i++) {
    const cp1x = (pts[i][0] + pts[i + 1][0]) / 2
    ctx.bezierCurveTo(cp1x, pts[i][1], cp1x, pts[i + 1][1], pts[i + 1][0], pts[i + 1][1])
  }
}

function draw() {
  const c = canvas.value
  const w = wrap.value
  if (!c || !w) return

  const dpr  = window.devicePixelRatio || 1
  const W    = w.clientWidth
  const H    = 120
  const dark = isDark()

  c.width        = W * dpr
  c.height       = H * dpr
  c.style.width  = W + 'px'
  c.style.height = H + 'px'

  const ctx = c.getContext('2d')!
  ctx.scale(dpr, dpr)
  ctx.clearRect(0, 0, W, H)

  const data = pings.value
  if (!data.length) return

  const pad  = { t: 16, b: 24, l: 10, r: 48 }
  const cw   = W - pad.l - pad.r
  const ch   = H - pad.t - pad.b
  const min  = Math.min(...data)
  const max  = Math.max(...data)
  const rng  = max - min || 1
  const avg  = data.reduce((a, b) => a + b, 0) / data.length

  const xOf = (i: number) => pad.l + (i / Math.max(data.length - 1, 1)) * cw
  const yOf = (v: number) => pad.t + ch - ((v - min) / rng) * ch

  // Grid lines
  const gridColor = dark ? 'rgba(255,255,255,0.06)' : 'rgba(0,0,0,0.06)'
  ;[0, 0.5, 1].forEach(f => {
    const y = pad.t + f * ch
    ctx.beginPath()
    ctx.moveTo(pad.l, y)
    ctx.lineTo(W - pad.r, y)
    ctx.strokeStyle = gridColor
    ctx.lineWidth   = 1
    ctx.stroke()
  })

  const pts: [number, number][] = data.map((v, i) => [xOf(i), yOf(v)])

  // Gradient fill
  const grad = ctx.createLinearGradient(0, pad.t, 0, pad.t + ch)
  grad.addColorStop(0, 'rgba(92,221,139,0.28)')
  grad.addColorStop(1, 'rgba(92,221,139,0.01)')

  ctx.beginPath()
  spline(ctx, pts)
  ctx.lineTo(xOf(data.length - 1), pad.t + ch)
  ctx.lineTo(xOf(0), pad.t + ch)
  ctx.closePath()
  ctx.fillStyle = grad
  ctx.fill()

  // Line
  ctx.beginPath()
  spline(ctx, pts)
  ctx.strokeStyle = '#5cdd8b'
  ctx.lineWidth   = 2
  ctx.lineJoin    = 'round'
  ctx.stroke()

  // Last-point dot
  const lx = xOf(data.length - 1)
  const ly = yOf(data[data.length - 1])
  ctx.beginPath()
  ctx.arc(lx, ly, 4, 0, Math.PI * 2)
  ctx.fillStyle = '#5cdd8b'
  ctx.fill()

  // Y-axis labels (min, avg, max)
  const labelColor = dark ? '#6b7280' : '#9ca3af'
  ctx.fillStyle  = labelColor
  ctx.font       = '10px system-ui, sans-serif'
  ctx.textAlign  = 'left'
  ctx.fillText(max.toFixed(0) + 'ms', W - pad.r + 6, pad.t + 4)
  ctx.fillText(avg.toFixed(0) + 'ms', W - pad.r + 6, yOf(avg) + 4)
  ctx.fillText(min.toFixed(0) + 'ms', W - pad.r + 6, pad.t + ch + 4)
}

let ro: ResizeObserver | null = null

onMounted(() => {
  draw()
  ro = new ResizeObserver(draw)
  if (wrap.value) ro.observe(wrap.value)
})

onUnmounted(() => ro?.disconnect())

watch(() => [props.beats, props.dark], draw, { deep: true })
</script>

<style lang="scss" scoped>
.ping-chart {
  position: relative;
  width: 100%;
  height: 120px;
  border-radius: 8px;
  overflow: hidden;
}

.ping-chart__canvas {
  display: block;
  width: 100%;
  height: 120px;
}

.ping-chart__empty {
  position: absolute;
  inset: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 12px;
  color: #9ca3af;
}
</style>
