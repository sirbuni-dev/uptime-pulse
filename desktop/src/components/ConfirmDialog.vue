<template>
  <div v-if="visible" class="modal-overlay" @click.self="$emit('cancel')">
    <div class="modal-box">
      <h2>{{ title }}</h2>
      <p class="modal-message">{{ message }}</p>
      <div class="modal-actions">
        <button class="btn-cancel" @click="$emit('cancel')">Cancel</button>
        <button class="btn-danger" @click="$emit('confirm')">{{ confirmLabel }}</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  visible:       boolean
  title:         string
  message:       string
  confirmLabel?: string
}>()

defineEmits<{ confirm: []; cancel: [] }>()
</script>

<style lang="scss" scoped>
@use 'sass:color';
@use '../assets/vars' as *;

.modal-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.6);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 200;
  backdrop-filter: blur(2px);
}

.modal-box {
  background: var(--bg-surface);
  border: 1px solid var(--border);
  border-radius: 12px;
  padding: 28px 32px;
  width: 360px;
  max-width: 95vw;
  color: var(--text);
  box-shadow: 0 20px 60px rgba(0,0,0,0.5);

  h2 {
    margin: 0 0 10px;
    font-size: 16px;
    color: var(--heading);
  }
}

.modal-message {
  margin: 0 0 24px;
  font-size: 13px;
  color: var(--text2);
  line-height: 1.5;
}

.modal-actions {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
}

.btn-cancel {
  padding: 8px 20px;
  border-radius: 8px;
  border: 1px solid var(--border);
  background: none;
  color: var(--text2);
  font-size: 13px;
  &:hover { border-color: var(--text2); color: var(--text); }
}

.btn-danger {
  padding: 8px 20px;
  border-radius: 8px;
  border: none;
  background: $danger;
  color: #fff;
  font-size: 13px;
  font-weight: 600;
  &:hover { background: color.adjust($danger, $lightness: -8%); }
}
</style>
