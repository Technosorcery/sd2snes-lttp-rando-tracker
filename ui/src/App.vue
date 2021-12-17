<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://v3.vuejs.org/api/sfc-script-setup.html#sfc-script-setup
import { getCurrentInstance, ref } from 'vue'
import { useStore } from './store'

const store = useStore()
const internalInstance = getCurrentInstance()

store.commit('fetchServerConfig', store.state.serverConfig.apiPort)
console.log("stored server config:", store.state.serverConfig)

let websocket_path = 'ws://' + window.location.hostname + ':' + store.state.serverConfig.apiPort + '/api/ws'
console.log("Connecting to websocket API at: " + websocket_path)
const stateApi = new WebSocket(websocket_path)
stateApi.onmessage = handleSocketEvent.bind(internalInstance)
stateApi.onerror = handleSocketError.bind(internalInstance)

function handleSocketEvent(event: MessageEvent) {
  let myEvent = JSON.parse(event.data)
  let dispatch;
  switch (myEvent.type) {
    case 'item':
      dispatch = 'setItemState'
      break
    case 'dungeon':
      dispatch = 'setDungeonState'
      break
    case 'location':
      dispatch = 'setLocationState'
      break
    default:
      return
  }

  window.console.log("Received " + myEvent.type + ' update:', myEvent.data)
  store.dispatch(dispatch, myEvent.data)
}

function handleSocketError(error: any) {
  window.console.log('Socket error: ' + JSON.stringify(error))
}
</script>

<template>
  <div class="app">
    <router-view />
  </div>
</template>

<style>
</style>
