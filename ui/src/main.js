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
    dungeons: {},
    locations: {}
  }
})

/* eslint-disable no-new */
new Vue({
  components: {
    App,
    ItemTracker
  },
  created() {
    let updateInterval = 250
    if (typeof this.$route.query.updateInterval !== 'undefined') {
      updateInterval = this.$route.query.updateInterval
    }
    setInterval(() => {
      this.$store.dispatch('retrieveGameState', process.env.API_PORT)
      this.$store.dispatch('retrieveDungeonState', process.env.API_PORT)
      this.$store.dispatch('retrieveLocationState', process.env.API_PORT)
    }, updateInterval)
  },
  el: '#app',
  router,
  store,
  template: '<App/>'
})
