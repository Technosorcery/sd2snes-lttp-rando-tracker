<template>
  <div
    :class="dungeonClass"
    :style="displayStyle"
    @click="toggleCleared">
    <span
      v-if="title"
      class="corner">{{ title }}</span>
    <span
      v-if="dungeon.totalChests > 0"
      class="chest"
      :style="chestsStyle"
      @click="openChest"></span>
    <span
      v-if="dungeon.hasReward"
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
    dungeon: Object
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
    rewardNumber() {
      return this.rewardSequence.indexOf(this.dungeon.reward)
    },

    medallionNumber() {
      return this.medallionSequence.indexOf(this.dungeon.medallion)
    },

    dungeonClass() {
      if (this.dungeon.name === 'Aga' && !this.dungeon.cleared) {
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
        '/static/image/chest' + (this.dungeon.totalChests - this.dungeon.foundChests) + '.png'
      )
    },

    medallionImage() {
      return '/static/image/medallion' + this.medallionNumber + '.png'
    },

    rewardImage() {
      return '/static/image/dungeon' + this.rewardNumber + '.png'
    },

    displayImage() {
      if (this.dungeon.cleared) {
        return '/static/image/' + this.dungeon.clearedImage
      } else {
        return '/static/image/' + this.dungeon.defaultImage
      }
    },

    medallionRequired() {
      switch (this.dungeon.dungeonCode) {
        case 'MM':
        case 'TR':
          return true
        default:
          return false
      }
    },

    title() {
      if (this.dungeon.dungeonCode === 'Aga') {
        return false
      } else {
        return this.dungeon.dungeonCode
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

      let newFoundChests = this.dungeon.foundChests + 1
      if (newFoundChests > this.dungeon.totalChests) {
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
      let newIndex = this[sequence + 'Number'] + 1
      if (newIndex >= this[sequence + 'Sequence'].length) {
        newIndex = 0
      }

      let data = {}
      data[sequence] = this[sequence + 'Sequence'][newIndex]
      this.updateServerState(data)
    },

    toggleCleared(event) {
      event.stopPropagation()

      let cleared = !this.dungeon.cleared
      let data = { cleared: cleared }
      this.updateServerState(data)
    },

    updateServerState(data) {
      let host = window.location.hostname + ':' + this.$store.state.serverConfig.apiPort

      var xhr = new XMLHttpRequest()
      xhr.open('POST', 'http://' + host + '/dungeon_state/' + this.dungeon.dungeonCode, true)
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
