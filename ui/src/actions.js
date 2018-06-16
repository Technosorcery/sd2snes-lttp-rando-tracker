let requestOptions = {
  headers: { 'Content-Type': 'application/json' }
}

export default {
  retrieveState ({commit}) {
    // TODO: Only set the port in development mode.
    let host = window.location.hostname + ':8000'

    fetch('http://' + host + '/game_state', requestOptions)
      .then(response => response.json())
      .then((data) => { commit('setState', data) })
  }
}
