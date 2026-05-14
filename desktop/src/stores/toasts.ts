import { defineStore } from 'pinia'

export type ToastType = 'down' | 'up'

export interface Toast {
  id:      number
  type:    ToastType
  message: string
}

let _nextId = 0
const AUTO_DISMISS_MS = 10_000

export const useToastStore = defineStore('toasts', {
  state: () => ({
    toasts: [] as Toast[],
  }),
  actions: {
    push(type: ToastType, message: string) {
      const id = ++_nextId
      this.toasts.push({ id, type, message })
      setTimeout(() => this.dismiss(id), AUTO_DISMISS_MS)
    },
    dismiss(id: number) {
      this.toasts = this.toasts.filter(t => t.id !== id)
    },
  },
})
