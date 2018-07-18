<template>
  <span>
    <span
      class="mapspan dungeon"
      :class="dungeonAvailability()"
      :style="dungeonStyle()"></span>
    <span
      class="mapspan boss"
      :class="bossAvailability()"
      :style="bossStyle()"></span>
  </span>
</template>

<script>
export default {
  name: 'Dungeon',
  props: {
    location: Object
  },
  computed: {
    dungeonCleared() {
      let remainingChests = this.location.totalChests - this.location.foundChests

      if (this.location.cleared) {
        return remainingChests === 0
      } else {
        return remainingChests <= 1
      }
    },
    bossCleared() {
      return this.location.cleared
    },
    locationLeft() {
      return this.location.position.horizontal.left
    },
    locationTop() {
      return this.location.position.horizontal.top
    }
  },
  methods: {
    dungeonStyle() {
      return (
        'background-image: url("/static/image/poi.png"); left: ' +
        this.locationLeft +
        '%; top: ' +
        this.locationTop +
        '%;'
      )
    },
    bossStyle() {
      return (
        'background-image: url("/static/image/boss' +
        this.location.boss.imageNumber +
        '.png"); left: ' +
        this.locationLeft +
        '%; top: ' +
        this.locationTop +
        '%;'
      )
    },
    dungeonAvailability() {
      if (this.dungeonCleared) {
        return 'opened'
      }

      return 'available'
    },
    bossAvailability() {
      if (this.location.cleared) {
        return 'opened'
      }

      return 'available'
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
.dungeon {
  width: 48px;
  height: 48px;
  background-size: 100% 100%;
  position: absolute;
  margin-left: -24px;
  margin-top: -24px;
}

.boss {
  width: 24px;
  height: 24px;
  background: no-repeat center;
  background-size: 75% 75%;
  position: absolute;
  margin-left: -12px;
  margin-top: -12px;
  z-index: 2;
}

.opened {
  background-color: rgb(127, 127, 127);
}

.available {
  background-color: rgb(0, 255, 0);
}

.possible {
  background-color: rgb(255, 255, 0);
}

.unavailable {
  background-color: rgb(255, 0, 0);
}
</style>
