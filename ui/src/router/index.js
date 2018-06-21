import Vue from 'vue'
import Router from 'vue-router'
import Dashboard from '@/components/Dashboard'
import ItemTracker from '@/components/ItemTracker'
import MapTracker from '@/components/MapTracker'

Vue.use(Router)

export default new Router({
  routes: [
    {
      path: '/',
      name: 'Dashboard',
      component: Dashboard
    },
    {
      path: '/items',
      name: 'Item Tracker',
      component: ItemTracker
    },
    {
      path: '/map',
      name: 'Map Tracker',
      component: MapTracker
    }
  ]
})
