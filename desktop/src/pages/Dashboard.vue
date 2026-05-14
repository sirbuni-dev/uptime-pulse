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
        Select a monitor from the list, or add a new one.
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
.dashboard {
  display: flex;
  height: 100vh;
  overflow: hidden;
}

.dashboard__sidebar {
  width: 300px;
  min-width: 220px;
  flex-shrink: 0;
  height: 100%;
  overflow: hidden;
}

.dashboard__main {
  flex: 1;
  padding: 20px 24px;
  overflow-y: auto;
}

.dashboard__empty {
  margin-top: 80px;
  text-align: center;
  color: #aaa;
  font-size: 14px;
}
</style>
