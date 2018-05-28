import Vue from 'vue'
import Router from 'vue-router'
import Dashboard from '@/components/Dashboard'
import ItemTracker from '@/components/ItemTracker'

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
      name: 'ItemTracker',
      component: ItemTracker
    }
  ]
})
