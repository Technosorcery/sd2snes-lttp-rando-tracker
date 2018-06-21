export default {
  setGameState(state, serverData) {
    state.game = serverData
  },

  setDungeonState(state, dungeonData) {
    state.dungeons = dungeonData
  },

  updateDungeonState(state, data) {
    for (var prop in data.data) {
      state.dungeons[data.name][prop] = data.data[prop]
    }
  },

  setLocationState(state, locationData) {
    state.locations = locationData
  },

  updateLocationState(state, data) {
    state.dungeons[data.name]['cleared'] = data.cleared
  }
}
