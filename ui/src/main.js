// The Vue build version to load with the `import` command
// (runtime-only or standalone) has been set in webpack.base.conf with an alias.
import actions from './actions'
import App from './App'
import mutations from './mutations'
import ReconnectingWebSocket from 'reconnecting-websocket'
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
    locations: {},
    serverConfig: {
      apiPort: 8000,
      websocketPort: 8001
    }
  }
})

/* eslint-disable no-new */
new Vue({
  components: {
    App
  },
  created() {
    this.$store.commit('fetchServerConfig', process.env.API_PORT)
    this.startStreamingApi()

    setInterval(() => {
      if (this.stateApi.readyState !== 1) {
        this.stateApi.reconnect()
      } else {
        this.stateApi.send('PING')
      }
    }, 5000)
    // setInterval(() => {
    //   this.stateApi.send('HELLO')
    // }, 60000)
  },
  el: '#app',
  router,
  store,
  template: '<App/>',
  data() {
    return {
      stateApi: undefined
    }
  },
  methods: {
    startStreamingApi() {
      let websocketServer = 'ws://' + window.location.hostname + ':' + this.$store.state.serverConfig.websocketPort
      console.log('Using websocket server: ' + websocketServer)
      this.stateApi = new ReconnectingWebSocket(websocketServer, [], { connectionTimeout: 1500, debug: false })
      this.stateApi.addEventListener('message', this.handleSocketEvent.bind(this))
      this.stateApi.addEventListener('error', this.handleSocketError.bind(this))
      this.stateApi.addEventListener('open', this.handleSocketOpen.bind(this))
    },
    handleSocketEvent(event) {
      let myEvent = JSON.parse(event.data)
      switch (myEvent.type) {
        case 'item':
          this.$store.dispatch('setItemState', myEvent.data)
          break
        case 'dungeon':
          this.$store.dispatch('setDungeonState', myEvent.data)
          break
        case 'location':
          this.$store.dispatch('setLocationState', myEvent.data)
          break
      }
    },
    handleSocketError(error) {
      console.log('Socket error: ' + JSON.stringify(error))
      // this.stateApi.reconnect()
    },
    handleSocketOpen(event) {
      console.log('Sending HELO')
      this.stateApi.send('HELLO')
    }
  }
})
