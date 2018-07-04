<template>
  <div class="map-tracker">
    <div id="map" class="map">
      <Location
        v-for="poi in poiLocations"
        :key="poi.name"
        :name="poi.name"
        :top="poi.top"
        :left="poi.left"
        :hoverText="poi.hoverText"></Location>
      <span
        v-for="dungeon in dungeonLocations"
        :key="dungeon.name"
        class="mapspan dungeon"
        :class="locationAvailability(dungeon)"
        :style="dungeonStyle(dungeon)"></span>
      <span
        v-for="boss in bossLocations"
        :key="boss.name"
        class="mapspan boss"
        :class="locationAvailability(boss)"
        :style="bossStyle(boss)"></span>
    </div>
  </div>
</template>

<script>
import Location from '@/components/map/Location.vue'

import poiLocations from '@/poiLocations.js'
import dungeonLocations from '@/dungeonLocations.js'
import bossLocations from '@/bossLocations.js'

export default {
  name: 'MapTracker',
  components: {
    Location
  },
  data() {
    return {
      poiLocations,
      dungeonLocations,
      bossLocations
    }
  },
  computed: {},
  methods: {
    poiStyle(poi) {
      return (
        'background-image: url("/static/image/poi.png"); left: ' +
        poi.left +
        '%; top: ' +
        poi.top +
        '%;'
      )
    },
    dungeonStyle(dungeon) {
      return (
        'background-image: url("/static/image/poi.png"); left: ' +
        dungeon.left +
        '%; top: ' +
        dungeon.top +
        '%;'
      )
    },
    bossStyle(boss) {
      return (
        'background-image: url("/static/image/boss' +
        boss.imageNumber +
        '.png"); left: ' +
        boss.left +
        '%; top: ' +
        boss.top +
        '%;'
      )
    },
    locationAvailability(location) {
      if (
        typeof location.available === 'undefined' ||
        location.available.some(this.haveItemsAndDungeons)
      ) {
        return 'available'
      } else if (
        typeof location.possible !== 'undefined' &&
        location.possible.some(this.haveItemsAndDungeons)
      ) {
        return 'possible'
      }

      return 'unavailable'
    },
    haveItemsAndDungeons(availability) {
      return this.haveItems(availability) && this.haveDungeons(availability)
    },
    haveItems(availability) {
      if (typeof availability === 'undefined') return true

      return availability.items.every(item => {
        return this.$store.state.game[item]
      })
    },
    haveDungeons(availability) {
      return true
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
.map-tracker .map {
  background-size: 100% 100%;
  background: url(/static/image/map.png) no-repeat;
  position: relative;
  width: 884px;
  height: 442px;
  left: 0;
}

.map .chest {
  width: 24px;
  height: 24px;
  background-size: 100% 100%;
  position: absolute;
  margin-left: -12px;
  margin-top: -12px;
}

.map .dungeon {
  width: 48px;
  height: 48px;
  background-size: 100% 100%;
  position: absolute;
  margin-left: -24px;
  margin-top: -24px;
}

.map .boss {
  width: 24px;
  height: 24px;
  background: no-repeat center;
  background-size: 75% 75%;
  position: absolute;
  margin-left: -12px;
  margin-top: -12px;
  z-index: 2;
}

.map .available {
  background-color: rgb(0, 255, 0);
  color: rgb(0, 0, 0);
}

.map .possible {
  background-color: rgb(255, 255, 0);
  color: rgb(0, 0, 0);
}

.map .unavailable {
  background-color: rgb(255, 0, 0);
}
</style>
