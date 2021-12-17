<template>
  <div class="map-tracker">
    <div id="map" class="map">
      <Location v-for="poi in locations" :key="poi.name" :location="poi"></Location>
      <Dungeon
        v-for="dungeon in dungeons"
        :key="dungeon.name"
        :location="dungeon"
      ></Dungeon>
    </div>
    <div id="caption">
      <span v-html="caption"></span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useStore } from '../store';
import Dungeon from "@/components/map/Dungeon.vue";
import Location from "@/components/map/Location.vue";

const store = useStore()

const caption = computed(() => store.state.caption)
const locations = computed(() => store.getters.mappableLocations)
const dungeons = computed(() => store.getters.mappableDungeons)
</script>

<style scoped>
.map-tracker .map {
  background-size: 100% 100%;
  background: url(/image/map.png) no-repeat;
  position: relative;
  width: 884px;
  height: 442px;
  left: 0;
}
</style>
