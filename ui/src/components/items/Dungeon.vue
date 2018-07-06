<template>
  <div
    :class="dungeonClass"
    :style="displayStyle"
    @click="toggleCleared">
    <span
      v-if="title"
      class="corner">{{ title }}</span>
    <span
      v-if="totalChests > 0"
      class="chest"
      :style="chestsStyle"
      @click="openChest"></span>
    <span
      v-if="hasReward"
      class="reward"
      :style="rewardStyle"
      @click="cycleReward"></span>
    <span
      v-if="medallionRequired"
      class="medallion"
      :style="medallionStyle"
      @click="cycleMedallion"></span>
  </div>
</template>

<script>
export default {
  name: 'Dungeon',
  props: {
    name: String
  },
  data() {
    return {
      medallionSequence: ['Unknown', 'Bombos', 'Ether', 'Quake'],
      rewardSequence: [
        'Unknown',
        'GreenPendant',
        'RedBluePendant',
        'Crystal',
        'RedCrystal'
      ]
    }
  },
  computed: {
    reward() {
      if (this.$store.state.dungeons && this.$store.state.dungeons[this.name]) {
        return this.rewardSequence.indexOf(
          this.$store.state.dungeons[this.name].reward
        )
      }

      return 0
    },

    foundChests() {
      if (this.$store.state.dungeons && this.$store.state.dungeons[this.name]) {
        return this.$store.state.dungeons[this.name].foundChests
      }

      return 0
    },

    medallion() {
      if (this.$store.state.dungeons && this.$store.state.dungeons[this.name]) {
        return this.medallionSequence.indexOf(
          this.$store.state.dungeons[this.name].medallion
        )
      }

      return 0
    },

    gotReward() {
      if (this.$store.state.dungeons && this.$store.state.dungeons[this.name]) {
        return this.$store.state.dungeons[this.name].cleared
      }

      return false
    },

    dungeonClass() {
      if (this.name === 'Aga' && !this.gotReward) {
        return 'dungeon false'
      } else {
        return 'dungeon'
      }
    },

    displayStyle() {
      return 'background-image: url(' + this.displayImage + ');'
    },

    chestsStyle() {
      return 'background-image: url(' + this.chestImage + ');'
    },

    rewardStyle() {
      return 'background-image: url(' + this.rewardImage + ');'
    },

    medallionStyle() {
      return 'background-image: url(' + this.medallionImage + ');'
    },

    chestImage() {
      return (
        '/static/image/chest' + (this.totalChests - this.foundChests) + '.png'
      )
    },

    medallionImage() {
      return '/static/image/medallion' + this.medallion + '.png'
    },

    rewardImage() {
      return '/static/image/dungeon' + this.reward + '.png'
    },

    displayImage() {
      if (this.gotReward) {
        return '/static/image/' + this.clearedImage
      } else {
        return '/static/image/' + this.defaultImage
      }
    },

    totalChests() {
      switch (this.name) {
        case 'PoD':
          return 5
        case 'SP':
          return 6
        case 'SW':
          return 2
        case 'TT':
          return 4
        case 'IP':
          return 3
        case 'MM':
          return 2
        case 'TR':
          return 5
        case 'EP':
          return 3
        case 'DP':
          return 2
        case 'ToH':
          return 2
        default:
          return 0
      }
    },

    medallionRequired() {
      switch (this.name) {
        case 'MM':
        case 'TR':
          return true
        default:
          return false
      }
    },

    hasReward() {
      if (this.name === 'Aga') {
        return false
      } else {
        return true
      }
    },

    defaultImage() {
      switch (this.name) {
        case 'EP':
          return 'boss01.png'
        case 'DP':
          return 'boss11.png'
        case 'ToH':
          return 'boss21.png'
        case 'PoD':
          return 'boss31.png'
        case 'SP':
          return 'boss41.png'
        case 'SW':
          return 'boss51.png'
        case 'TT':
          return 'boss61.png'
        case 'IP':
          return 'boss71.png'
        case 'MM':
          return 'boss81.png'
        case 'TR':
          return 'boss91.png'
        case 'Aga':
          return 'agahnim0.png'
      }
    },

    clearedImage() {
      switch (this.name) {
        case 'EP':
          return 'boss02.png'
        case 'DP':
          return 'boss12.png'
        case 'ToH':
          return 'boss22.png'
        case 'PoD':
          return 'boss32.png'
        case 'SP':
          return 'boss42.png'
        case 'SW':
          return 'boss52.png'
        case 'TT':
          return 'boss62.png'
        case 'IP':
          return 'boss72.png'
        case 'MM':
          return 'boss82.png'
        case 'TR':
          return 'boss92.png'
        case 'Aga':
          return 'agahnim1.png'
      }
    },

    title() {
      if (this.name === 'Aga') {
        return ''
      } else {
        return this.name
      }
    }
  },

  methods: {
    cycleMedallion(event) {
      event.stopPropagation()

      this.cycleSequence('medallion')
    },

    openChest(event) {
      event.stopPropagation()

      let newFoundChests = this.foundChests + 1
      if (newFoundChests > this.totalChests) {
        newFoundChests = 0
      }

      let data = { foundChests: newFoundChests }
      this.updateServerState(data)
    },

    cycleReward(event) {
      event.stopPropagation()

      this.cycleSequence('reward')
    },

    cycleSequence(sequence) {
      let newIndex = this[sequence] + 1
      if (newIndex >= this[sequence + 'Sequence'].length) {
        newIndex = 0
      }

      let data = {}
      data[sequence] = this[sequence + 'Sequence'][newIndex]
      this.updateServerState(data)
    },

    toggleCleared(event) {
      event.stopPropagation()

      let cleared = !this.gotReward
      let data = { cleared: cleared }
      this.updateServerState(data)
    },

    updateServerState(data) {
      let host = window.location.hostname + ':' + this.$store.state.serverConfig.apiPort

      var xhr = new XMLHttpRequest()
      xhr.open('POST', 'http://' + host + '/dungeon_state/' + this.name, true)
      xhr.setRequestHeader('Content-Type', 'application/json')
      xhr.send(JSON.stringify(data))
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
.dungeon {
  display: inline-block;
  height: 64px;
  position: relative;
  width: 64px;
}

.dungeon.false {
  opacity: 0.25;
}

.dungeon .corner {
  color: lightgray;
  cursor: default;
  display: block;
  font: 26px 'VT323', monospace;
  height: 32px;
  position: absolute;
  text-shadow: 0 0 3px black;
  user-select: none;
  width: 32px;
}

.dungeon .chest {
  display: block;
  height: 32px;
  position: absolute;
  top: 32px;
  width: 32px;
}

.dungeon .reward {
  display: block;
  height: 32px;
  left: 32px;
  position: absolute;
  top: 32px;
  width: 32px;
}

.dungeon .medallion {
  display: block;
  height: 32px;
  left: 32px;
  position: absolute;
  width: 32px;
}
</style>
