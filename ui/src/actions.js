export default {
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
}
