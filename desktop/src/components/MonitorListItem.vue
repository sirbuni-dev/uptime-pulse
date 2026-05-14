<template>
  <div
    class="monitor-item"
    :class="{
      'monitor-item--selected': isSelected,
      'monitor-item--paused':   !monitor.active,
    }"
    @click="$emit('select', monitor.id)"
  >
    <span class="status-dot" :class="dotClass" />
    <span class="monitor-item__name">{{ monitor.name }}</span>
    <div class="mini-beats">
      <span
        v-for="(b, i) in miniBeats"
        :key="i"
        class="mini-beat"
        :class="b"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { Monitor } from '../stores/monitors'
import { useMonitorStore, DOWN, UP, PENDING } from '../stores/monitors'

const props = defineProps<{ monitor: Monitor; isSelected: boolean }>()
defineEmits<{ select: [id: number] }>()

const store         = useMonitorStore()
const currentStatus = computed(() => store.currentStatus(props.monitor.id))

const dotClass = computed(() => ({
  'dot--up':      currentStatus.value === UP,
  'dot--down':    currentStatus.value === DOWN,
  'dot--pending': currentStatus.value === PENDING,
}))

const miniBeats = computed(() => {
  const beats = store.heartbeats[props.monitor.id] ?? []
  const slots = 30
  const result: string[] = []
  for (let i = 0; i < slots; i++) {
    const b = beats[slots - 1 - i]
    if (!b)                   result.push('mini-beat--empty')
    else if (b.status === UP) result.push('mini-beat--up')
    else if (b.status === DOWN) result.push('mini-beat--down')
    else                      result.push('mini-beat--pending')
  }
  return result
})
</script>

<style lang="scss" scoped>
@use '../assets/vars' as *;

.monitor-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 12px;
  cursor: pointer;
  border-bottom: 1px solid rgba(255,255,255,0.04);
  transition: background 0.15s;

  &:hover        { background: rgba(255,255,255,0.05); }
  &--selected    { background: rgba(92,221,139,0.08); border-left: 3px solid $primary; padding-left: 9px; }
  &--paused      { opacity: 0.5; }
}

.status-dot {
  width: 10px;
  height: 10px;
  border-radius: 50%;
  flex-shrink: 0;

  &.dot--up      { background: $primary; box-shadow: 0 0 6px rgba(92,221,139,0.6); }
  &.dot--down    { background: $danger;  box-shadow: 0 0 6px rgba(220,53,69,0.6); }
  &.dot--pending { background: $warning; }
}

.monitor-item__name {
  font-size: 13px;
  font-weight: 500;
  color: #c9d1d9;
  flex: 1;
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;

  .monitor-item--selected & { color: #e6edf3; }
}

.mini-beats {
  display: flex;
  gap: 1.5px;
  align-items: center;
  flex-shrink: 0;
}

.mini-beat {
  width: 4px;
  height: 16px;
  border-radius: 2px;
  flex-shrink: 0;

  &--up      { background: $primary; }
  &--down    { background: $danger; }
  &--pending { background: $warning; }
  &--empty   { background: rgba(255,255,255,0.1); }
}
</style>
