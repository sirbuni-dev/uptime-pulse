<template>
  <div class="monitor-list">
    <div class="monitor-list__header">
      <span>Monitors ({{ store.monitors.length }})</span>
      <button class="btn-add" @click="$emit('addMonitor')">+ Add</button>
    </div>
    <div v-if="store.loading" class="monitor-list__empty">Loading…</div>
    <div v-else-if="store.monitors.length === 0" class="monitor-list__empty">
      No monitors yet. Add one to get started.
    </div>
    <MonitorListItem
      v-for="m in store.monitors"
      :key="m.id"
      :monitor="m"
      :is-selected="m.id === selectedId"
      @select="$emit('select', $event)"
    />
  </div>
</template>

<script setup lang="ts">
import { useMonitorStore } from '../stores/monitors'
import MonitorListItem from './MonitorListItem.vue'

defineProps<{ selectedId: number | null }>()
defineEmits<{ select: [id: number]; addMonitor: [] }>()

const store = useMonitorStore()
</script>

<style lang="scss" scoped>
@use '../assets/vars' as *;

.monitor-list {
  height: 100%;
  overflow-y: auto;
  border-right: 1px solid #eee;

  .dark & { border-color: $dark-border-color; }
}

.monitor-list__header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 14px;
  font-weight: 700;
  border-bottom: 1px solid #eee;
  position: sticky;
  top: 0;
  background: #fff;

  .dark & { background: $dark-bg; border-color: $dark-border-color; }
}

.monitor-list__empty {
  padding: 20px 14px;
  color: $secondary-text;
  font-size: 13px;
}

.btn-add {
  background: none;
  border: 1px solid $primary;
  color: $primary;
  border-radius: 6px;
  padding: 2px 10px;
  cursor: pointer;
  font-size: 13px;
  &:hover { background: $primary; color: #000; }
}
</style>
