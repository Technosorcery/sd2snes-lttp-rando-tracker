export default {
  setItemState(state, serverData) {
    state.game = serverData
  },

  setDungeonState(state, dungeonData) {
    state.dungeons = dungeonData
  },

  setLocationState(state, locationData) {
    state.locations = locationData
  }
}
