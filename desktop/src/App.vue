<template>
  <router-view />
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import { useMonitorStore } from './stores/monitors'
import { useThemeStore } from './stores/theme'

const store = useMonitorStore()
const theme = useThemeStore()

onMounted(async () => {
  theme.init()
  await store.subscribeToEvents()
  await store.fetchAll()
  await store.startAllMonitors()
})
</script>
