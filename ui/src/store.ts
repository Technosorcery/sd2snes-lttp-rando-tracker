import { AppConfig, InjectionKey } from 'vue'
import { createStore, useStore as baseUseStore, Store } from 'vuex'
import type { GameState } from "./server_types/GameState"
import type { Dungeon } from "./server_types/Dungeon"
import type { Location } from "./server_types/Location"
import type { ServerConfig } from "./server_types/ServerConfig"

export interface State {
  caption?: string,
  game?: GameState,
  dungeons: Dungeon[],
  locations: Location[],
  serverConfig: ServerConfig,
}

export const key: InjectionKey<Store<State>> = Symbol()

export const store = createStore({
  state(): State {
    return {
      caption: "",
      dungeons: [],
      locations: [],
      serverConfig: {
        dataPollRate: 1000,
        dataSource: {
          type: "LocalFile",
          source: "./example-data.json",
        },
        logic: "glitchless",
        apiPort: 8000,
      },
    }
  },
  actions: {
    setItemState({ commit }, state) {
      commit('setItemState', state)
    },
    setDungeonState({ commit }, state) {
      commit('setDungeonState', state)
    },
    setLocationState({ commit }, state) {
      commit('setLocationState', state)
    },
  
    updateDungeonState({ commit }, data) {
      commit('updateDungeonState', data)
    },
  
    updateLocationState({ commit }, data) {
      commit('updateLocationState', data)
    },
    updateCaption({ commit }, data) {
      commit('updateCaption', data)
    }  
  },
  getters: {
    getDungeonByDungeonCode: (state: any) => (code: String) => {
      return state.dungeons.find(function (dungeon: any) {
        return dungeon.dungeonCode === code
      })
    },
    mappableLocations: (state) => {
      return state.locations.filter(function (location: any) {
        return !!location.position
      })
    },
    mappableDungeons: (state) => {
      return state.dungeons.filter(function (dungeon: any) {
        return !!dungeon.position
      })
    }  
  },
  mutations: {
    setItemState(state, serverData) {
      state.game = serverData
    },
  
    setDungeonState(state, dungeonData) {
      state.dungeons = dungeonData
    },
  
    setLocationState(state, locationData) {
      state.locations = locationData
    },
  
    updateCaption(state, data) {
      state.caption = data
    },
  
    async fetchServerConfig(state, apiPort) {
      let host =
        typeof apiPort === 'undefined'
          ? window.location.host
          : window.location.hostname + ':' + apiPort
  
      await fetch('http://' + host + '/api/config', { headers: { 'Content-Type': 'application/json' } })
        .then(response => response.json())
        .then(data => {
          window.console.log('Storing server config: ', data)
          state.serverConfig = data
        })
    }  
  }
})

export function useStore () {
  return baseUseStore(key)
}
