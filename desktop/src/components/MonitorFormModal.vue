<template>
  <div v-if="visible" class="modal-overlay" @click.self="$emit('close')">
    <div class="modal-box">
      <h2>{{ isEdit ? 'Edit Monitor' : 'Add Monitor' }}</h2>
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
          <button type="button" @click="$emit('close')">Cancel</button>
          <button type="submit" class="btn-primary">{{ isEdit ? 'Save' : 'Add' }}</button>
        </div>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
import { reactive, watch, computed } from 'vue'
import type { Monitor } from '../stores/monitors'

const props = defineProps<{ visible: boolean; monitor?: Monitor | null }>()
const emit  = defineEmits<{
  close:  []
  submit: [payload: { name: string; url: string; interval: number; timeout: number }]
}>()

const isEdit = computed(() => !!props.monitor)

const form = reactive({
  name:     props.monitor?.name     ?? '',
  url:      props.monitor?.url      ?? '',
  interval: props.monitor?.interval ?? 60,
  timeout:  props.monitor?.timeout  ?? 10,
})

watch(() => props.monitor, (m) => {
  if (m) {
    form.name     = m.name
    form.url      = m.url
    form.interval = m.interval
    form.timeout  = m.timeout
  }
})

function submit() {
  emit('submit', { ...form })
}
</script>

<style lang="scss" scoped>
@use '../assets/vars' as *;

.modal-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.4);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 100;
}

.modal-box {
  background: #fff;
  border-radius: 12px;
  padding: 28px;
  width: 380px;
  max-width: 95vw;

  .dark & { background: $dark-header-bg; color: $dark-font-color; }

  h2 { margin: 0 0 18px; font-size: 18px; }

  label {
    display: flex;
    flex-direction: column;
    gap: 4px;
    margin-bottom: 14px;
    font-size: 13px;
  }

  input {
    border: 1px solid #ddd;
    border-radius: 8px;
    padding: 7px 10px;
    font-size: 14px;

    .dark & { background: $dark-bg2; border-color: $dark-border-color; color: $dark-font-color; }
  }
}

.modal-actions {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
  margin-top: 8px;

  button {
    padding: 7px 18px;
    border-radius: 8px;
    border: 1px solid #ddd;
    cursor: pointer;
    font-size: 14px;
    background: none;
  }

  .btn-primary {
    background: $primary;
    border-color: $primary;
    color: #000;
    font-weight: 600;
    &:hover { background: $highlight; }
  }
}
</style>
