<template>
  <div class="monitor-list">
    <!-- Header -->
    <div class="monitor-list__header">
      <img src="/vite.svg" class="app-logo" alt="" />
      <span class="app-title">Uptime Pulse</span>
      <div class="header-actions">
        <button class="btn-icon-only" :title="theme.dark ? 'Switch to light' : 'Switch to dark'" @click="theme.toggle()">
          <Sun v-if="theme.dark" :size="16" />
          <Moon v-else :size="16" />
        </button>
      </div>
    </div>

    <!-- Add button -->
    <div class="monitor-list__toolbar">
      <button class="btn-add" @click="$emit('addMonitor')">
        <Plus :size="14" />
        Add New Monitor
      </button>
    </div>

    <!-- Search -->
    <div class="monitor-list__search">
      <Search :size="13" class="search-icon" />
      <input
        v-model="searchQuery"
        class="search-input"
        placeholder="Search monitors…"
        type="search"
      />
    </div>

    <!-- List -->
    <div v-if="store.loading" class="monitor-list__empty">Loading…</div>
    <div v-else-if="filtered.length === 0" class="monitor-list__empty">
      {{ store.monitors.length === 0 ? 'No monitors yet.' : 'No results.' }}
    </div>
    <MonitorListItem
      v-for="m in filtered"
      :key="m.id"
      :monitor="m"
      :is-selected="m.id === selectedId"
      @select="$emit('select', $event)"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { Plus, Search, Sun, Moon } from 'lucide-vue-next'
import { useMonitorStore } from '../stores/monitors'
import { useThemeStore } from '../stores/theme'
import MonitorListItem from './MonitorListItem.vue'

defineProps<{ selectedId: number | null }>()
defineEmits<{ select: [id: number]; addMonitor: [] }>()

const store       = useMonitorStore()
const theme       = useThemeStore()
const searchQuery = ref('')

const filtered = computed(() => store.filteredMonitors(searchQuery.value))
</script>

<style lang="scss" scoped>
@use '../assets/vars' as *;

.monitor-list {
  height: 100%;
  display: flex;
  flex-direction: column;
  background: var(--bg2);
  overflow: hidden;
}

.monitor-list__header {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 14px 12px 12px;
  border-bottom: 1px solid var(--border);
  flex-shrink: 0;
}

.app-logo {
  width: 20px;
  height: 20px;
  opacity: 0.8;
}

.app-title {
  font-weight: 700;
  font-size: 14px;
  color: var(--heading);
  flex: 1;
}

.header-actions {
  display: flex;
  gap: 4px;
}

.btn-icon-only {
  background: none;
  border: none;
  color: var(--text3);
  padding: 4px;
  border-radius: 6px;
  display: flex;
  align-items: center;
  transition: color 0.15s, background 0.15s;

  &:hover { color: var(--text); background: var(--input-bg); }
}

.monitor-list__toolbar {
  padding: 10px 12px 6px;
  flex-shrink: 0;
}

.btn-add {
  width: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  background: $primary;
  color: #000;
  border: none;
  border-radius: 8px;
  padding: 8px 12px;
  font-size: 13px;
  font-weight: 600;
  transition: background 0.15s;

  &:hover { background: $highlight; }
}

.monitor-list__search {
  position: relative;
  padding: 0 12px 8px;
  flex-shrink: 0;
}

.search-icon {
  position: absolute;
  left: 22px;
  top: 50%;
  transform: translateY(-60%);
  color: var(--text3);
  pointer-events: none;
}

.search-input {
  width: 100%;
  background: var(--input-bg);
  border: 1px solid var(--border);
  border-radius: 8px;
  padding: 6px 10px 6px 30px;
  font-size: 12px;
  color: var(--text);
  outline: none;
  transition: border-color 0.15s;

  &::placeholder  { color: var(--text4); }
  &:focus         { border-color: rgba(92,221,139,0.4); }

  &::-webkit-search-cancel-button { display: none; }
}

.monitor-list__empty {
  padding: 20px 12px;
  color: var(--text4);
  font-size: 12px;
  text-align: center;
}
</style>
