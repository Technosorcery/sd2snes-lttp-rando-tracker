let requestOptions = {
  headers: { 'Content-Type': 'application/json' }
}

export default {
  setItemState(state, serverData) {
    state.game = serverData
  },

  setDungeonState(state, dungeonData) {
    state.dungeons = dungeonData
  },

  setLocationState(state, locationData) {
    state.locations = locationData
  },

  async fetchServerConfig(state, apiPort) {
    let host =
      typeof apiPort === 'undefined'
        ? window.location.host
        : window.location.hostname + ':' + apiPort

    await fetch('http://' + host + '/config', requestOptions)
      .then(response => response.json())
      .then(data => {
        console.log('Storing server config: ', data)
        state.serverConfig = data
      })
  }
}
