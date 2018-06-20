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
Vue.config.devtools = true
Vue.use(Vuex)

const store = new Vuex.Store({
  actions,
  mutations,
  state: {
    game: {},
    dungeons: {}
  }
})

/* eslint-disable no-new */
new Vue({
  components: {
    App,
    ItemTracker
  },
  created() {
    setInterval(() => {
      this.$store.dispatch('retrieveGameState', process.env.API_PORT)
      this.$store.dispatch('retrieveDungeonState', process.env.API_PORT)
    }, 250)
  },
  el: '#app',
  router,
  store,
  template: '<App/>'
})
