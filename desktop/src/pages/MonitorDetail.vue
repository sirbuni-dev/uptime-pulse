<template>
  <div v-if="monitor" class="monitor-detail">

    <!-- Header -->
    <div class="monitor-detail__header">
      <div class="monitor-detail__title">
        <h1>{{ monitor.name }}</h1>
        <a :href="monitor.url" target="_blank" class="monitor-detail__url">
          <ExternalLink :size="11" />
          {{ monitor.url }}
        </a>
      </div>
      <div class="monitor-detail__actions">
        <StatusBadge :status="currentStatus" />
        <button
          class="btn-action"
          :class="monitor.active ? 'btn-action--warn' : 'btn-action--success'"
          :title="monitor.active ? 'Pause' : 'Resume'"
          @click="togglePause"
        >
          <Pause v-if="monitor.active" :size="13" />
          <Play v-else :size="13" />
          {{ monitor.active ? 'Pause' : 'Resume' }}
        </button>
        <button class="btn-action" title="Edit" @click="showEdit = true">
          <Pencil :size="13" /> Edit
        </button>
        <button class="btn-action" title="Clone" @click="openClone">
          <Copy :size="13" /> Clone
        </button>
        <button class="btn-action btn-action--danger" title="Delete" @click="confirmDelete">
          <Trash2 :size="13" /> Delete
        </button>
      </div>
    </div>

    <!-- Heartbeat bar -->
    <div class="hb-section">
      <HeartbeatBar :beats="beats" />
      <div class="hb-meta">
        Checks every {{ monitor.interval }}s
        <span class="hb-meta__status" :class="statusClass">
          &bull; {{ statusLabel }}
        </span>
      </div>
    </div>

    <!-- Stats row -->
    <div class="stats-row">
      <div class="stat-box">
        <div class="stat-box__label">Response <em>(Current)</em></div>
        <div class="stat-box__value">{{ latestPing != null ? latestPing.toFixed(0) + 'ms' : '—' }}</div>
      </div>
      <div class="stat-box">
        <div class="stat-box__label">Avg. Response <em>(24 hours)</em></div>
        <div class="stat-box__value">{{ avgPing != null ? avgPing.toFixed(0) + 'ms' : '—' }}</div>
      </div>
      <div class="stat-box">
        <div class="stat-box__label">Uptime <em>(24 hours)</em></div>
        <div class="stat-box__value uptime">{{ periods?.h24?.toFixed(2) ?? '—' }}%</div>
      </div>
      <div class="stat-box">
        <div class="stat-box__label">Uptime <em>(30 days)</em></div>
        <div class="stat-box__value uptime">{{ periods?.d30?.toFixed(2) ?? '—' }}%</div>
      </div>
      <div class="stat-box">
        <div class="stat-box__label">Uptime <em>(1 year)</em></div>
        <div class="stat-box__value uptime">{{ periods?.y1?.toFixed(2) ?? '—' }}%</div>
      </div>
    </div>

    <!-- Ping chart -->
    <section class="monitor-detail__section">
      <div class="section-header">
        <h3>Avg / Ping</h3>
        <button
          class="btn-chart-toggle"
          :class="{ 'btn-chart-toggle--active': showChart }"
          @click="showChart = !showChart"
        >
          <Activity :size="12" />
          Live Chart
        </button>
      </div>
      <PingChart v-if="showChart" :beats="beats" />
    </section>

    <!-- Recent checks -->
    <section class="monitor-detail__section">
      <div class="section-header">
        <h3>Recent Checks</h3>
        <div class="page-controls">
          <label class="page-size-label">
            Show
            <select v-model.number="pageSize" @change="currentPage = 1">
              <option :value="10">10</option>
              <option :value="25">25</option>
              <option :value="50">50</option>
            </select>
          </label>
          <span class="page-info">{{ paginationLabel }}</span>
          <button class="btn-page" :disabled="currentPage === 1" @click="currentPage--">
            <ChevronLeft :size="14" />
          </button>
          <button class="btn-page" :disabled="currentPage >= totalPages" @click="currentPage++">
            <ChevronRight :size="14" />
          </button>
        </div>
      </div>

      <table class="checks-table">
        <thead>
          <tr>
            <th>Status</th>
            <th>Date / Time</th>
            <th>Ping</th>
            <th>Message</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="b in paginatedBeats" :key="b.id" :class="rowClass(b.status)">
            <td><StatusBadge :status="b.status" /></td>
            <td class="td-time">{{ b.checkedAt }}</td>
            <td>{{ b.ping != null ? b.ping.toFixed(0) + 'ms' : '—' }}</td>
            <td class="td-msg">{{ b.msg ?? '' }}</td>
          </tr>
          <tr v-if="paginatedBeats.length === 0">
            <td colspan="4" class="td-empty">No checks recorded yet.</td>
          </tr>
        </tbody>
      </table>
    </section>

    <!-- Modals -->
    <MonitorFormModal
      :visible="showEdit"
      :monitor="monitor"
      @close="showEdit = false"
      @submit="onEdit"
    />
    <MonitorFormModal
      :visible="showClone"
      :defaults="cloneDefaults"
      @close="showClone = false"
      @submit="onClone"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import {
  Pause, Play, Pencil, Copy, Trash2,
  ExternalLink, Activity, ChevronLeft, ChevronRight,
} from 'lucide-vue-next'
import { useMonitorStore, DOWN, UP, PENDING } from '../stores/monitors'
import HeartbeatBar from '../components/HeartbeatBar.vue'
import PingChart from '../components/PingChart.vue'
import StatusBadge from '../components/StatusBadge.vue'
import MonitorFormModal from '../components/MonitorFormModal.vue'

const props = defineProps<{ monitorId: number }>()
const store = useMonitorStore()

const showEdit  = ref(false)
const showClone = ref(false)
const showChart = ref(true)
const pageSize  = ref(10)
const currentPage = ref(1)

const monitor       = computed(() => store.monitors.find(m => m.id === props.monitorId) ?? null)
const beats         = computed(() => store.heartbeats[props.monitorId] ?? [])
const periods       = computed(() => store.uptimePeriods[props.monitorId])
const currentStatus = computed(() => store.currentStatus(props.monitorId))

const latestPing = computed(() => beats.value.find(b => b.ping != null)?.ping ?? null)

const avgPing = computed(() => {
  const ps = beats.value.filter(b => b.ping != null).map(b => b.ping!)
  if (!ps.length) return null
  return ps.reduce((a, b) => a + b, 0) / ps.length
})

const statusLabel = computed(() => {
  if (currentStatus.value === UP)      return 'Up'
  if (currentStatus.value === DOWN)    return 'Down'
  return 'Pending'
})

const statusClass = computed(() => ({
  'status-up':      currentStatus.value === UP,
  'status-down':    currentStatus.value === DOWN,
  'status-pending': currentStatus.value === PENDING,
}))

const totalPages = computed(() => Math.max(1, Math.ceil(beats.value.length / pageSize.value)))

const paginatedBeats = computed(() => {
  const start = (currentPage.value - 1) * pageSize.value
  return beats.value.slice(start, start + pageSize.value)
})

const paginationLabel = computed(() => {
  const total = beats.value.length
  if (!total) return '0 entries'
  const start = (currentPage.value - 1) * pageSize.value + 1
  const end   = Math.min(currentPage.value * pageSize.value, total)
  return `${start}–${end} of ${total}`
})

const cloneDefaults = computed(() =>
  monitor.value
    ? {
        name:     monitor.value.name + ' (Clone)',
        url:      monitor.value.url,
        interval: monitor.value.interval,
        timeout:  monitor.value.timeout,
      }
    : undefined
)

watch(() => props.monitorId, (id) => {
  currentPage.value = 1
  store.fetchUptimePeriods(id)
}, { immediate: true })

function rowClass(status: number) {
  return { 'row--down': status === DOWN }
}

async function togglePause() {
  if (!monitor.value) return
  await store.toggleActive(monitor.value.id, !monitor.value.active)
}

function openClone() { showClone.value = true }

async function onEdit(payload: { name: string; url: string; interval: number; timeout: number }) {
  if (!monitor.value) return
  await store.updateMonitor({ id: monitor.value.id, active: monitor.value.active, ...payload })
  showEdit.value = false
}

async function onClone(payload: { name: string; url: string; interval: number; timeout: number }) {
  await store.createMonitor(payload)
  showClone.value = false
}

async function confirmDelete() {
  if (!monitor.value) return
  if (confirm(`Delete "${monitor.value.name}"?`)) {
    await store.deleteMonitor(monitor.value.id)
  }
}
</script>

<style lang="scss" scoped>
@use '../assets/vars' as *;

.monitor-detail {
  color: var(--text);
}

// Header
.monitor-detail__header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 20px;
  flex-wrap: wrap;
  gap: 12px;
}

.monitor-detail__title {
  h1 { margin: 0 0 4px; font-size: 22px; color: var(--heading); font-weight: 700; }
}

.monitor-detail__url {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  font-size: 12px;
  color: var(--text2);
  &:hover { color: $primary; }
}

.monitor-detail__actions {
  display: flex;
  align-items: center;
  gap: 6px;
  flex-wrap: wrap;
}

.btn-action {
  display: inline-flex;
  align-items: center;
  gap: 5px;
  padding: 5px 12px;
  border-radius: 7px;
  border: 1px solid var(--border);
  background: none;
  color: var(--text2);
  font-size: 12px;
  font-weight: 500;
  transition: all 0.15s;

  &:hover { border-color: var(--text2); color: var(--text); }

  &--success {
    border-color: rgba(92,221,139,0.4);
    color: $primary;
    &:hover { background: rgba(92,221,139,0.1); }
  }

  &--warn {
    border-color: rgba(248,163,6,0.4);
    color: $warning;
    &:hover { background: rgba(248,163,6,0.1); }
  }

  &--danger {
    border-color: rgba(220,53,69,0.4);
    color: $danger;
    &:hover { background: rgba(220,53,69,0.1); }
  }
}

// Heartbeat section
.hb-section {
  margin-bottom: 20px;
}

.hb-meta {
  margin-top: 6px;
  font-size: 11px;
  color: var(--text4);
}

.hb-meta__status {
  &.status-up      { color: $primary; }
  &.status-down    { color: $danger; }
  &.status-pending { color: $warning; }
}

// Stats
.stats-row {
  display: flex;
  gap: 1px;
  margin-bottom: 24px;
  background: var(--border);
  border: 1px solid var(--border);
  border-radius: 10px;
  overflow: hidden;
}

.stat-box {
  flex: 1;
  padding: 14px 16px;
  background: var(--bg-surface);
  min-width: 0;

  &:first-child { border-radius: 9px 0 0 9px; }
  &:last-child  { border-radius: 0 9px 9px 0; }
}

.stat-box__label {
  font-size: 11px;
  color: var(--text3);
  margin-bottom: 6px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;

  em { font-style: normal; display: block; font-size: 10px; color: var(--text4); }
}

.stat-box__value {
  font-size: 20px;
  font-weight: 700;
  color: var(--heading);

  &.uptime { color: $primary; }
}

// Sections
.monitor-detail__section {
  margin-bottom: 28px;
}

.section-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 10px;

  h3 {
    margin: 0;
    font-size: 12px;
    font-weight: 600;
    color: var(--text3);
    text-transform: uppercase;
    letter-spacing: 0.06em;
  }
}

.btn-chart-toggle {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  padding: 3px 10px;
  border-radius: 6px;
  border: 1px solid var(--border);
  background: none;
  color: var(--text3);
  font-size: 11px;
  transition: all 0.15s;

  &:hover { border-color: var(--text2); color: var(--text); }

  &--active {
    border-color: rgba(92,221,139,0.4);
    color: $primary;
    background: rgba(92,221,139,0.06);
  }
}

// Pagination
.page-controls {
  display: flex;
  align-items: center;
  gap: 8px;
}

.page-size-label {
  font-size: 11px;
  color: var(--text3);
  display: flex;
  align-items: center;
  gap: 5px;

  select {
    background: var(--bg2);
    border: 1px solid var(--border);
    color: var(--text);
    border-radius: 5px;
    padding: 2px 4px;
    font-size: 11px;
    outline: none;
  }
}

.page-info {
  font-size: 11px;
  color: var(--text3);
  white-space: nowrap;
}

.btn-page {
  background: none;
  border: 1px solid var(--border);
  color: var(--text2);
  border-radius: 5px;
  padding: 2px 5px;
  display: flex;
  align-items: center;
  transition: all 0.15s;

  &:hover:not(:disabled) { border-color: var(--text2); color: var(--text); }
  &:disabled { opacity: 0.3; cursor: not-allowed; }
}

// Table
.checks-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 12px;

  th {
    text-align: left;
    padding: 6px 10px;
    color: var(--text3);
    font-weight: 600;
    font-size: 11px;
    text-transform: uppercase;
    letter-spacing: 0.04em;
    border-bottom: 1px solid var(--border);
  }

  td {
    padding: 7px 10px;
    border-bottom: 1px solid var(--row-sep);
    color: var(--text);
    vertical-align: middle;
  }

  .td-time { color: var(--text2); font-family: monospace; font-size: 11px; }
  .td-msg  { color: var(--text2); font-size: 11px; }
  .td-empty { text-align: center; padding: 20px; color: var(--text4); }

  .row--down td { background: rgba(220,53,69,0.04); }
}
</style>
