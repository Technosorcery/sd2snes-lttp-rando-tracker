<template>
  <span>
    <span
      class="mapspan dungeon"
      :class="[
        dungeonAvailability(),
        activeClass()
      ]"
      :style="dungeonStyle()"
      @mouseover="updateCaption(); hover = true"
      @mouseleave="clearCaption(); hover = false"
    ></span>
    <span
      class="mapspan boss"
      :class="bossAvailability()"
      :style="bossStyle()"
      @mouseover="updateBossCaption(); hover = true"
      @mouseleave="clearCaption(); hover = false"
    ></span>
  </span>
</template>

<script lang="ts">
export default {
  name: "DungeonMapView",
}
</script>
<script setup lang="ts">
import { computed, ref } from 'vue'
import { useStore } from '../../store'
import type { Dungeon } from "../../server_types/Dungeon"

const store = useStore()

interface Props {
  location: Dungeon,
}

const props = defineProps<Props>()
const hover = ref(false)

const dungeonCleared = computed(() => remainingChests.value === 0)
const remainingChests = computed(() => props.location.totalChests - props.location.foundChests)
const locationLeft = computed(() => props.location?.position?.horizontal.left)
const locationTop = computed(() => props.location?.position?.horizontal.top)

function dungeonStyle() {
  return (
    "left: " + locationLeft.value + "%; top: " + locationTop.value + "%;"
  );
}

function bossStyle() {
  return (
    'background-image: url("/image/boss' +
    props.location?.boss?.imageNumber +
    '.png"); left: ' +
    locationLeft.value +
    "%; top: " +
    locationTop.value +
    "%;"
  );
}

function dungeonAvailability() {
  if (dungeonCleared.value) {
    return "opened";
  }

  return props.location.dungeonAvailability;
}
function bossAvailability() {
  if (props.location.cleared) {
    return "opened";
  }

  return props.location.bossAvailability;
}

function updateCaption() {
  store.dispatch("updateCaption", props.location.hoverText);
}

function clearCaption() {
  store.dispatch("updateCaption", "&nbsp;");
}

function updateBossCaption() {
  store.dispatch("updateCaption", props.location?.boss?.hoverText);
}

function activeClass() {
  return hover.value ? "active" : "inactive";
}
</script>

<style scoped>
.dungeon {
  width: 48px;
  height: 48px;
  background-size: 100% 100%;
  position: absolute;
  margin-left: -24px;
  margin-top: -24px;
  background-image: url("/image/poi.png");
}

.inactive {
  background-image: url("/image/poi.png");
}

.active {
  background-image: url("/image/highlighted.png");
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

.glitchAvailable {
  background-color: rgb(0, 127, 0);
}

.glitchPossible {
  background-color: rgb(192, 192, 0);
}

.agahnim {
  background-color: rgb(0, 255, 255);
}

.glitchAgahnim {
  background-color: rgb(0, 160, 160);
}
</style>
