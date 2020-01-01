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
      return this.remainingChests === 0
    },
    bossCleared() {
      return this.location.cleared
    },
    remainingChests() {
      return this.location.totalChests - this.location.foundChests
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
        'left: ' +
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

      return this.location.dungeonAvailability
    },
    bossAvailability() {
      if (this.location.cleared) {
        return 'opened'
      }

      return this.location.bossAvailability
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
  background-image: url("/static/image/poi.png");
}

.dungeon:hover {
  background-image: url("/static/image/highlighted.png");
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
