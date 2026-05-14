<template>
  <span class="badge" :class="badgeClass">{{ label }}</span>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { DOWN, UP, PENDING } from '../stores/monitors'

const props = defineProps<{ status: number; uptime?: number }>()

const badgeClass = computed(() => ({
  'badge-up':      props.status === UP,
  'badge-down':    props.status === DOWN,
  'badge-pending': props.status === PENDING,
}))

const label = computed(() => {
  if (props.uptime !== undefined) return `${props.uptime.toFixed(1)}%`
  return props.status === UP ? 'Up' : props.status === DOWN ? 'Down' : 'Pending'
})
</script>

<style lang="scss" scoped>
@use '../assets/vars' as *;

.badge {
  display: inline-block;
  padding: 2px 10px;
  border-radius: $border-radius;
  font-size: 12px;
  font-weight: 600;
}

.badge-up      { background: $primary; color: #000; }
.badge-down    { background: $danger;  color: #fff; }
.badge-pending { background: $warning; color: #000; }
</style>
