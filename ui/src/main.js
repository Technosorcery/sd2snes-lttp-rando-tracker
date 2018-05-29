// The Vue build version to load with the `import` command
// (runtime-only or standalone) has been set in webpack.base.conf with an alias.
import actions from './actions'
import App from './App'
import ItemTracker from '@/components/ItemTracker'
import mutations from './mutations'
import router from './router'
import Vue from 'vue'
import Vuex from 'vuex'

Vue.config.productionTip = false
Vue.use(Vuex)

const store = new Vuex.Store({
  actions,
  mutations,
  state: {
    game: {}
  }
})

/* eslint-disable no-new */
new Vue({
  el: '#app',
  router,
  components: {
    App,
    ItemTracker
  },
  store,
  template: '<App/>'
})
