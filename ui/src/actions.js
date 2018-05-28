let requestOptions = {
  headers: { 'content-type': 'application/json' }
}

export default {
  retrieveState ({commit}) {
    fetch('/game_state', requestOptions)
      .then(response => response.json())
      .then((data) => { commit('setState', data) })
  }
}
