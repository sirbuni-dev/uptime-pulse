import { defineStore } from 'pinia'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

export const DOWN = 0
export const UP   = 1
export const PENDING = 2

export interface Monitor {
  id:        number
  name:      string
  url:       string
  type:      string
  interval:  number
  timeout:   number
  active:    boolean
  createdAt: string
  updatedAt: string
}

export interface Heartbeat {
  id:        number
  monitorId: number
  status:    number
  ping:      number | null
  msg:       string | null
  checkedAt: string
}

export interface CreateMonitorPayload {
  name:      string
  url:       string
  interval?: number
  timeout?:  number
}

export interface UpdateMonitorPayload {
  id:       number
  name:     string
  url:      string
  interval: number
  timeout:  number
  active:   boolean
}

export const useMonitorStore = defineStore('monitors', {
  state: () => ({
    monitors:  [] as Monitor[],
    heartbeats: {} as Record<number, Heartbeat[]>,
    uptime:    {} as Record<number, number>,
    loading:   false,
  }),

  getters: {
    currentStatus: (state) => (monitorId: number): number => {
      const beats = state.heartbeats[monitorId]
      return beats && beats.length > 0 ? beats[0].status : PENDING
    },
  },

  actions: {
    async fetchAll() {
      this.loading = true
      try {
        this.monitors = await invoke<Monitor[]>('list_monitors')
        await Promise.all(this.monitors.map(m => this.fetchHeartbeats(m.id)))
        await Promise.all(this.monitors.map(m => this.fetchUptime(m.id)))
      } finally {
        this.loading = false
      }
    },

    async fetchHeartbeats(monitorId: number, limit = 90) {
      this.heartbeats[monitorId] = await invoke<Heartbeat[]>('get_heartbeats', { monitorId, limit })
    },

    async fetchUptime(monitorId: number, hours = 24) {
      this.uptime[monitorId] = await invoke<number>('get_uptime_percentage', { monitorId, hours })
    },

    async createMonitor(payload: CreateMonitorPayload) {
      const m = await invoke<Monitor>('create_monitor', { payload })
      this.monitors.push(m)
      this.heartbeats[m.id] = []
      this.uptime[m.id] = 0
      return m
    },

    async updateMonitor(payload: UpdateMonitorPayload) {
      const m = await invoke<Monitor>('update_monitor', { payload })
      const idx = this.monitors.findIndex(x => x.id === m.id)
      if (idx !== -1) this.monitors[idx] = m
      return m
    },

    async deleteMonitor(id: number) {
      await invoke('delete_monitor', { id })
      this.monitors = this.monitors.filter(m => m.id !== id)
      delete this.heartbeats[id]
      delete this.uptime[id]
    },

    handleNewHeartbeat(event: Omit<Heartbeat, 'id'> & { monitorId: number }) {
      const monitorId = event.monitorId
      if (!this.heartbeats[monitorId]) this.heartbeats[monitorId] = []
      const beat: Heartbeat = { id: Date.now(), ...event }
      this.heartbeats[monitorId] = [beat, ...this.heartbeats[monitorId]].slice(0, 90)
      this.fetchUptime(monitorId)
    },

    async startAllMonitors() {
      await invoke('start_all_monitors')
    },

    async subscribeToEvents() {
      await listen('monitor:heartbeat', (event) => {
        this.handleNewHeartbeat(event.payload as any)
      })
    },
  },
})
