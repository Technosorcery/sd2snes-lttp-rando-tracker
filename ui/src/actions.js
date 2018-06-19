let requestOptions = {
  headers: { 'Content-Type': 'application/json' }
}

export default {
  retrieveState({ commit }, apiPort) {
    let host =
      typeof apiPort === 'undefined'
        ? window.location.host
        : window.location.hostname + ':' + apiPort

    fetch('http://' + host + '/game_state', requestOptions)
      .then(response => response.json())
      .then(data => {
        commit('setState', data)
      })
  }
}
