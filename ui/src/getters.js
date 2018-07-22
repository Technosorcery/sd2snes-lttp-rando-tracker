export default {
  getDungeonByDungeonCode: (state) => (code) => {
    return state.dungeons.find(function (dungeon) {
      return dungeon.dungeonCode === code
    })
  },
  mappableLocations(state) {
    return state.locations.filter(function (location) {
      return !!location.position
    })
  },
  mappableDungeons(state) {
    return state.dungeons.filter(function (dungeon) {
      return !!dungeon.position
    })
  }
}
