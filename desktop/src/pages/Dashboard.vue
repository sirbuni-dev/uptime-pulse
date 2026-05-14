<template>
  <div class="dashboard">
    <aside class="dashboard__sidebar" :style="{ width: sidebarWidth + 'px' }">
      <MonitorList
        :selected-id="selectedId"
        @select="selectedId = $event"
        @add-monitor="showAddModal = true"
      />
    </aside>

    <div
      class="drag-handle"
      :class="{ 'drag-handle--active': isDragging }"
      @mousedown.prevent="startDrag"
    />

    <main class="dashboard__main">
      <MonitorDetail v-if="selectedMonitor" :monitor-id="selectedId!" />
      <div v-else class="dashboard__empty">
        <Activity :size="40" class="empty-icon" />
        <p>Select a monitor from the list, or add a new one.</p>
      </div>
    </main>

    <MonitorFormModal
      :visible="showAddModal"
      @close="showAddModal = false"
      @submit="onCreate"
    />

    <ToastContainer />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onUnmounted } from 'vue'
import { Activity } from 'lucide-vue-next'
import { useMonitorStore } from '../stores/monitors'
import MonitorList from '../components/MonitorList.vue'
import MonitorDetail from './MonitorDetail.vue'
import MonitorFormModal from '../components/MonitorFormModal.vue'
import ToastContainer from '../components/ToastContainer.vue'

const store        = useMonitorStore()
const selectedId   = ref<number | null>(null)
const showAddModal = ref(false)

const selectedMonitor = computed(() =>
  store.monitors.find(m => m.id === selectedId.value) ?? null
)

async function onCreate(payload: { name: string; url: string; interval: number; timeout: number }) {
  const m = await store.createMonitor(payload)
  selectedId.value   = m.id
  showAddModal.value = false
}

// ── Resizable sidebar ──────────────────────────────────────────────────────
const MIN_SIDEBAR_W = 180
const MAX_SIDEBAR_W = 480

const sidebarWidth = ref(300)
const isDragging   = ref(false)

let _dragStartX = 0
let _dragStartW = 0

function startDrag(e: MouseEvent) {
  isDragging.value = true
  _dragStartX = e.clientX
  _dragStartW = sidebarWidth.value
  document.addEventListener('mousemove', onDrag)
  document.addEventListener('mouseup',   stopDrag)
  document.body.style.cursor     = 'col-resize'
  document.body.style.userSelect = 'none'
}

function onDrag(e: MouseEvent) {
  const delta = e.clientX - _dragStartX
  sidebarWidth.value = Math.min(MAX_SIDEBAR_W, Math.max(MIN_SIDEBAR_W, _dragStartW + delta))
}

function stopDrag() {
  isDragging.value = false
  document.removeEventListener('mousemove', onDrag)
  document.removeEventListener('mouseup',   stopDrag)
  document.body.style.cursor     = ''
  document.body.style.userSelect = ''
}

onUnmounted(stopDrag)
</script>

<style lang="scss" scoped>
@use '../assets/vars' as *;

.dashboard {
  display: flex;
  height: 100vh;
  overflow: hidden;
  background: var(--bg);
}

.dashboard__sidebar {
  flex-shrink: 0;
  height: 100%;
  overflow: hidden;
}

.drag-handle {
  width: 4px;
  flex-shrink: 0;
  height: 100%;
  background: var(--border);
  cursor: col-resize;
  transition: background 0.15s;

  &:hover,
  &.drag-handle--active { background: rgba(92, 221, 139, 0.45); }
}

.dashboard__main {
  flex: 1;
  padding: 24px 28px;
  overflow-y: auto;
  background: var(--bg);
  color: var(--text);
}

.dashboard__empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  gap: 12px;
  color: var(--text4);

  p { margin: 0; font-size: 14px; }
}

.empty-icon { opacity: 0.3; }
</style>
