import { createRouter, createWebHistory } from 'vue-router'
import Dashboard from '@/components/Dashboard.vue'
import ItemTracker from '@/components/ItemTracker.vue'
import MapTracker from '@/components/MapTracker.vue'

export const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      name: 'Dashboard',
      component: Dashboard,
      alias: '/ui',
    },
    {
      path: '/items',
      name: 'Item Tracker',
      component: ItemTracker,
      alias: '/ui/items',
    },
    {
      path: '/map',
      name: 'Map Tracker',
      component: MapTracker,
      alias: '/ui/map',
    },
  ],
})
