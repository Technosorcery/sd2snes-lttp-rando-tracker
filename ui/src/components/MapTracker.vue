<template>
  <div class="map-tracker">
    <div id="map" class="map">
      <Location
        v-for="poi in locations"
        :key="poi.name"
        :location="poi"></Location>
      <Dungeon
        v-for="dungeon in dungeons"
        :key="dungeon.name"
        :location="dungeon"></Dungeon>
    </div>
    <div id="caption">
      <span v-html="caption"></span>
    </div>
  </div>
</template>

<script>
import { mapGetters } from 'vuex'
import Dungeon from '@/components/map/Dungeon.vue'
import Location from '@/components/map/Location.vue'

export default {
  name: 'MapTracker',
  components: {
    Location,
    Dungeon
  },
  computed: {
    caption() {
      return this.$store.state.caption
    },
    ...mapGetters({
      locations: 'mappableLocations',
      dungeons: 'mappableDungeons',
    }
    )
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
