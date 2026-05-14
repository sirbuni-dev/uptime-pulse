<template>
  <div v-if="monitor" class="monitor-detail">
    <div class="monitor-detail__header">
      <div>
        <h1>{{ monitor.name }}</h1>
        <a :href="monitor.url" target="_blank" class="monitor-detail__url">{{ monitor.url }}</a>
      </div>
      <div class="monitor-detail__actions">
        <StatusBadge :status="currentStatus" />
        <button @click="showEdit = true">Edit</button>
        <button class="btn-danger" @click="confirmDelete">Delete</button>
      </div>
    </div>

    <section class="monitor-detail__section">
      <h3>Uptime (24h)</h3>
      <div class="uptime-value">{{ uptime.toFixed(2) }}%</div>
    </section>

    <section class="monitor-detail__section">
      <h3>Heartbeat History</h3>
      <HeartbeatBar :beats="beats" />
    </section>

    <section class="monitor-detail__section">
      <h3>Recent Checks</h3>
      <table class="checks-table">
        <thead>
          <tr><th>Time</th><th>Status</th><th>Ping</th><th>Message</th></tr>
        </thead>
        <tbody>
          <tr v-for="b in beats.slice(0, 20)" :key="b.id">
            <td>{{ b.checkedAt }}</td>
            <td><StatusBadge :status="b.status" /></td>
            <td>{{ b.ping !== null ? b.ping.toFixed(0) + 'ms' : '—' }}</td>
            <td>{{ b.msg ?? '' }}</td>
          </tr>
        </tbody>
      </table>
    </section>

    <MonitorFormModal
      :visible="showEdit"
      :monitor="monitor"
      @close="showEdit = false"
      @submit="onEdit"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useMonitorStore } from '../stores/monitors'
import HeartbeatBar from '../components/HeartbeatBar.vue'
import StatusBadge from '../components/StatusBadge.vue'
import MonitorFormModal from '../components/MonitorFormModal.vue'

const props = defineProps<{ monitorId: number }>()
const store = useMonitorStore()
const showEdit = ref(false)

const monitor       = computed(() => store.monitors.find(m => m.id === props.monitorId) ?? null)
const beats         = computed(() => store.heartbeats[props.monitorId] ?? [])
const uptime        = computed(() => store.uptime[props.monitorId] ?? 0)
const currentStatus = computed(() => store.currentStatus(props.monitorId))

async function onEdit(payload: { name: string; url: string; interval: number; timeout: number }) {
  if (!monitor.value) return
  await store.updateMonitor({ id: monitor.value.id, active: monitor.value.active, ...payload })
  showEdit.value = false
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

.monitor-detail__header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 24px;

  h1 { margin: 0 0 4px; font-size: 22px; }
}

.monitor-detail__url {
  font-size: 13px;
  color: $secondary-text;
  &:hover { color: $primary; }
}

.monitor-detail__actions {
  display: flex;
  gap: 8px;
  align-items: center;

  button {
    padding: 5px 14px;
    border-radius: 8px;
    border: 1px solid #ddd;
    cursor: pointer;
    font-size: 13px;
    background: none;
  }
}

.btn-danger {
  border-color: $danger;
  color: $danger;
  &:hover { background: $danger; color: #fff; }
}

.monitor-detail__section {
  margin-bottom: 28px;
  h3 { font-size: 14px; font-weight: 600; color: $secondary-text; margin-bottom: 10px; }
}

.uptime-value { font-size: 32px; font-weight: 700; color: $primary; }

.checks-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 13px;

  th, td {
    padding: 6px 10px;
    text-align: left;
    border-bottom: 1px solid #eee;
  }

  th { font-weight: 600; color: $secondary-text; }

  .dark & th,
  .dark & td { border-color: $dark-border-color; }
}
</style>
