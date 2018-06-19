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
  }
}
