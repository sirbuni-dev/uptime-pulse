import { createRouter, createWebHistory } from 'vue-router'
import Dashboard from './pages/Dashboard.vue'
import MonitorDetail from './pages/MonitorDetail.vue'

export default createRouter({
  history: createWebHistory(),
  routes: [
    { path: '/',             component: Dashboard },
    { path: '/monitor/:id', component: MonitorDetail, props: true },
  ],
})
