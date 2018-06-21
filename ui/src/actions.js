let requestOptions = {
  headers: { 'Content-Type': 'application/json' }
}

export default {
  retrieveGameState({ commit }, apiPort) {
    let host =
      typeof apiPort === 'undefined'
        ? window.location.host
        : window.location.hostname + ':' + apiPort

    fetch('http://' + host + '/game_state', requestOptions)
      .then(response => response.json())
      .then(data => {
        commit('setGameState', data)
      })
  },

  retrieveDungeonState({ commit }, apiPort) {
    let host =
      typeof apiPort === 'undefined'
        ? window.location.host
        : window.location.hostname + ':' + apiPort

    fetch('http://' + host + '/dungeon_state', requestOptions)
      .then(response => response.json())
      .then(data => {
        commit('setDungeonState', data)
      })
  },

  updateDungeonState({ commit }, data) {
    commit('updateDungeonState', data)
  },

  retrieveLocationState({ commit }, apiPort) {
    let host =
      typeof apiPort === 'undefined'
        ? window.location.host
        : window.location.hostname + ':' + apiPort

    fetch('http://' + host + '/location_state', requestOptions)
      .then(response => response.json())
      .then(data => {
        commit('setLocationState', data)
      })
  },

  updateLocationState({ commit }, data) {
    commit('updateLocationState', data)
  }
}
