<template>
  <div v-if="visible" class="modal-overlay" @click.self="$emit('close')">
    <div class="modal-box">
      <div class="modal-header">
        <h2>{{ isEdit ? 'Edit Monitor' : 'Add Monitor' }}</h2>
        <button class="btn-close" @click="$emit('close')"><X :size="18" /></button>
      </div>
      <form @submit.prevent="submit">
        <label>Name
          <input v-model="form.name" required placeholder="My Service" />
        </label>
        <label>URL
          <input v-model="form.url" required type="url" placeholder="https://example.com" />
        </label>
        <label>Interval (seconds)
          <input v-model.number="form.interval" type="number" min="10" max="3600" />
        </label>
        <label>Timeout (seconds)
          <input v-model.number="form.timeout" type="number" min="1" max="60" />
        </label>
        <div class="modal-actions">
          <button type="button" class="btn-cancel" @click="$emit('close')">Cancel</button>
          <button type="submit" class="btn-submit">{{ isEdit ? 'Save' : 'Add' }}</button>
        </div>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
import { reactive, watch, computed } from 'vue'
import { X } from 'lucide-vue-next'
import type { Monitor } from '../stores/monitors'

interface FormDefaults {
  name?: string; url?: string; interval?: number; timeout?: number
}

const props = defineProps<{
  visible:   boolean
  monitor?:  Monitor | null
  defaults?: FormDefaults
}>()

const emit = defineEmits<{
  close:  []
  submit: [payload: { name: string; url: string; interval: number; timeout: number }]
}>()

const isEdit = computed(() => !!props.monitor)

const form = reactive({
  name:     props.monitor?.name     ?? props.defaults?.name     ?? '',
  url:      props.monitor?.url      ?? props.defaults?.url      ?? '',
  interval: props.monitor?.interval ?? props.defaults?.interval ?? 60,
  timeout:  props.monitor?.timeout  ?? props.defaults?.timeout  ?? 10,
})

watch(() => props.monitor, (m) => {
  if (m) { form.name = m.name; form.url = m.url; form.interval = m.interval; form.timeout = m.timeout }
})

watch(() => props.defaults, (d) => {
  if (d && !props.monitor) {
    form.name = d.name ?? ''; form.url = d.url ?? ''
    form.interval = d.interval ?? 60; form.timeout = d.timeout ?? 10
  }
})

function submit() { emit('submit', { ...form }) }
</script>

<style lang="scss" scoped>
@use '../assets/vars' as *;

.modal-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.6);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 100;
  backdrop-filter: blur(2px);
}

.modal-box {
  background: $dark-header-bg;
  border: 1px solid $dark-border-color;
  border-radius: 12px;
  padding: 24px 28px 28px;
  width: 400px;
  max-width: 95vw;
  color: $dark-font-color;
  box-shadow: 0 20px 60px rgba(0,0,0,0.5);
}

.modal-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 20px;

  h2 { margin: 0; font-size: 17px; color: #e6edf3; }
}

.btn-close {
  background: none;
  border: none;
  color: #6b7280;
  padding: 4px;
  border-radius: 6px;
  display: flex;
  &:hover { color: #c9d1d9; }
}

label {
  display: flex;
  flex-direction: column;
  gap: 5px;
  margin-bottom: 14px;
  font-size: 12px;
  color: #8b949e;
  font-weight: 500;
  text-transform: uppercase;
  letter-spacing: 0.04em;
}

input {
  background: $dark-bg2;
  border: 1px solid $dark-border-color;
  border-radius: 8px;
  padding: 8px 12px;
  font-size: 14px;
  color: #e6edf3;
  outline: none;
  transition: border-color 0.15s;

  &:focus { border-color: rgba(92,221,139,0.5); }
}

.modal-actions {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
  margin-top: 20px;
}

.btn-cancel {
  padding: 8px 20px;
  border-radius: 8px;
  border: 1px solid $dark-border-color;
  background: none;
  color: #8b949e;
  font-size: 14px;
  &:hover { border-color: #8b949e; color: #c9d1d9; }
}

.btn-submit {
  padding: 8px 20px;
  border-radius: 8px;
  border: none;
  background: $primary;
  color: #000;
  font-size: 14px;
  font-weight: 600;
  &:hover { background: $highlight; }
}
</style>
