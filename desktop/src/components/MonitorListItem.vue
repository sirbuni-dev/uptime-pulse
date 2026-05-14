<template>
  <div
    class="monitor-item"
    :class="{ 'monitor-item--active': isActive, 'monitor-item--selected': isSelected }"
    @click="$emit('select', monitor.id)"
  >
    <StatusBadge :status="currentStatus" />
    <span class="monitor-item__name">{{ monitor.name }}</span>
    <span class="monitor-item__url">{{ monitor.url }}</span>
    <span class="monitor-item__uptime">{{ uptimeLabel }}</span>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { Monitor } from '../stores/monitors'
import { useMonitorStore } from '../stores/monitors'
import StatusBadge from './StatusBadge.vue'

const props = defineProps<{ monitor: Monitor; isSelected: boolean }>()
defineEmits<{ select: [id: number] }>()

const store         = useMonitorStore()
const currentStatus = computed(() => store.currentStatus(props.monitor.id))
const isActive      = computed(() => props.monitor.active)
const uptimeLabel   = computed(() => {
  const u = store.uptime[props.monitor.id]
  return u !== undefined ? u.toFixed(1) + '%' : '—'
})
</script>

<style lang="scss" scoped>
@use '../assets/vars' as *;

.monitor-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 14px;
  cursor: pointer;
  border-bottom: 1px solid #eee;
  transition: background $easing-out 0.15s;

  &:hover       { background: #f5f5f5; }
  &--selected   { background: #eaf9f0; }
  &:not(.monitor-item--active) { opacity: 0.5; }

  .dark & {
    border-color: $dark-border-color;
    &:hover     { background: $dark-header-bg; }
    &--selected { background: #0d2b1a; }
  }
}

.monitor-item__name {
  font-weight: 600;
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.monitor-item__url {
  font-size: 11px;
  color: $secondary-text;
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.monitor-item__uptime {
  font-size: 12px;
  font-weight: 600;
  min-width: 44px;
  text-align: right;
}
</style>
