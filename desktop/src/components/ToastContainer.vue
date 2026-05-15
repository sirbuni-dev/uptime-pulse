<template>
  <Teleport to="body">
    <div class="toast-container" aria-live="polite">
      <TransitionGroup name="toast" tag="div">
        <div
          v-for="t in store.toasts"
          :key="t.id"
          class="toast"
          :class="`toast--${t.type}`"
        >
          <AlertTriangle v-if="t.type === 'down' || t.type === 'error'" :size="15" class="toast-icon" />
          <Info          v-else-if="t.type === 'info'"                   :size="15" class="toast-icon" />
          <CheckCircle   v-else                                           :size="15" class="toast-icon" />
          <span class="toast-msg">{{ t.message }}</span>
          <button class="toast-close" :aria-label="'Dismiss'" @click="store.dismiss(t.id)">×</button>
        </div>
      </TransitionGroup>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
import { AlertTriangle, CheckCircle, Info } from 'lucide-vue-next'
import { useToastStore } from '../stores/toasts'

const store = useToastStore()
</script>

<style lang="scss" scoped>
@use '../assets/vars' as *;

.toast-container {
  position: fixed;
  top: 20px;
  right: 20px;
  z-index: 9999;
  display: flex;
  flex-direction: column;
  gap: 8px;
  max-width: 420px;
  pointer-events: none;
}

.toast {
  display: flex;
  align-items: flex-start;
  gap: 10px;
  padding: 12px 14px;
  border-radius: 8px;
  font-size: 13px;
  font-weight: 500;
  line-height: 1.4;
  pointer-events: all;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.35);

  &--down  { background: rgba(185, 30, 42, 0.95); color: #fff; }
  &--up    { background: rgba(30, 120, 60, 0.95); color: #fff; }
  &--info  { background: rgba(30, 80, 160, 0.95); color: #fff; }
  &--error { background: rgba(185, 30, 42, 0.95); color: #fff; }
}

.toast-icon {
  flex-shrink: 0;
  margin-top: 1px;
  opacity: 0.9;
}

.toast-msg {
  flex: 1;
  word-break: break-word;
}

.toast-close {
  background: none;
  border: none;
  color: rgba(255, 255, 255, 0.7);
  font-size: 18px;
  line-height: 1;
  padding: 0 2px;
  flex-shrink: 0;
  margin-top: -2px;
  transition: color 0.15s;

  &:hover { color: #fff; }
}

// TransitionGroup animations
.toast-enter-active {
  transition: all 0.25s ease;
}
.toast-leave-active {
  transition: all 0.2s ease;
}
.toast-enter-from {
  opacity: 0;
  transform: translateX(40px);
}
.toast-leave-to {
  opacity: 0;
  transform: translateX(40px);
}
.toast-move {
  transition: transform 0.2s ease;
}
</style>
