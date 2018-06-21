<template>
  <div class="map-tracker">
    <div id="map" class="map">
      <span
        v-for="poi in poiLocations"
        :key="poi.name"
        class="mapspan chest"
        :class="locationAvailability(poi)"
        :style="poiStyle(poi)"></span>
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
export default {
  name: 'MapTracker',
  data() {
    return {
      poiLocations: [
        {
          name: "Link's House",
          left: '27.4',
          top: '67.9',
          hoverText: "Link's House"
        }
      ],
      dungeonLocations: [
        {
          name: 'Desert Palace',
          left: '3.8',
          top: '78.4',
          hoverText: 'Desert Palace',
          availableItems: [
            ['boots', 'book'],
            ['boots', 'flute', 'titans_mitt', 'mirror']
          ],
          possibleItems: [['book'], ['flute', 'titans_mitt', 'mirror']]
        }
      ],
      bossLocations: [
        {
          name: 'Lanmolas',
          left: '3.8',
          top: '78.4',
          hoverText: 'Lanmolas',
          imageNumber: 12,
          availableItems: [
            ['boots', 'book'],
            ['boots', 'flute', 'titans_mitt', 'mirror']
          ],
          possibleItems: [['book'], ['flute', 'titans_mitt', 'mirror']]
        }
      ]
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
        typeof location.availableItems === 'undefined' ||
        location.availableItems.some(this.haveItems)
      ) {
        return 'available'
      } else if (
        typeof location.possibleItems !== 'undefined' &&
        location.possibleItems.some(this.haveItems)
      ) {
        return 'possible'
      }

      return 'unavailable'
    },
    haveItems(items) {
      return items.every(item => {
        return this.$store.state.game[item]
      })
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
