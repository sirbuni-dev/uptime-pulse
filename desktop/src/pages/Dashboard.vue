<template>
  <div class="dashboard">
    <aside class="dashboard__sidebar">
      <MonitorList
        :selected-id="selectedId"
        @select="selectedId = $event"
        @add-monitor="showAddModal = true"
      />
    </aside>

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
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { Activity } from 'lucide-vue-next'
import { useMonitorStore } from '../stores/monitors'
import MonitorList from '../components/MonitorList.vue'
import MonitorDetail from './MonitorDetail.vue'
import MonitorFormModal from '../components/MonitorFormModal.vue'

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
</script>

<style lang="scss" scoped>
@use '../assets/vars' as *;

.dashboard {
  display: flex;
  height: 100vh;
  overflow: hidden;
  background: $dark-bg;
}

.dashboard__sidebar {
  width: 280px;
  min-width: 220px;
  flex-shrink: 0;
  height: 100%;
  overflow: hidden;
  border-right: 1px solid $dark-border-color;
}

.dashboard__main {
  flex: 1;
  padding: 24px 28px;
  overflow-y: auto;
  background: $dark-bg;
  color: $dark-font-color;
}

.dashboard__empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  gap: 12px;
  color: #4b5563;

  p { margin: 0; font-size: 14px; }
}

.empty-icon { opacity: 0.3; }
</style>
