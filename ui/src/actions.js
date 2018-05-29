let requestOptions = {
  headers: { 'Content-Type': 'application/json' }
}

export default {
  retrieveState ({commit}) {
    fetch('http://localhost:8000/game_state', requestOptions)
      .then(response => response.json())
      .then((data) => { commit('setState', data) })
  }
}
