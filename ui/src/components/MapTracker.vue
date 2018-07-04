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
      <Dungeon
        v-for="dungeon in dungeonLocations"
        :key="dungeon.name"
        :location="dungeon"></Dungeon>
    </div>
  </div>
</template>

<script>
import Dungeon from '@/components/map/Dungeon.vue'
import Location from '@/components/map/Location.vue'

import poiLocations from '@/poiLocations.js'
import dungeonLocations from '@/dungeonLocations.js'

export default {
  name: 'MapTracker',
  components: {
    Location,
    Dungeon
  },
  data() {
    return {
      poiLocations,
      dungeonLocations
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
</style>
