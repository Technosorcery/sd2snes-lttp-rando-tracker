<template>
  <span
    class="mapspan chest"
    :class="locationAvailability()"
    :style="locationStyle()"
    @click="toggleLocation"
    @mouseover="updateCaption()"
    @mouseleave="clearCaption()"></span>
</template>

<script lang="ts">
export default {
  name: 'Location',
}
</script>
<script setup lang="ts">
import { computed } from 'vue'
import { useStore } from '../../store'
import type { Location } from "../../server_types/Location"

interface Props {
  location: Location,
}

const props = defineProps<Props>()
const store = useStore()

const locationLeft = computed(() => props.location.position.horizontal.left)
const locationTop = computed(() => props.location.position.horizontal.top)

function locationStyle() {
  return (
    'left: ' +
    locationLeft.value +
    '%; top: ' +
    locationTop.value +
    '%;'
  )
}

function locationAvailability() {
  if (props.location.cleared) {
    return 'opened'
  }

  return props.location.availability
}

function toggleLocation(event: MouseEvent) {
  event.stopPropagation()

  let cleared = !props.location.cleared
  let data = { cleared: cleared }
  updateLocationState(data)
}

function updateLocationState(data: any) {
  let host = window.location.hostname + ':' + store?.state?.serverConfig?.apiPort

  var xhr = new XMLHttpRequest()
  xhr.open('POST', 'http://' + host + '/api/location_state/' + props.location.name, true)
  xhr.setRequestHeader('Content-Type', 'application/json')
  xhr.send(JSON.stringify(data))
}

function updateCaption() {
  store.dispatch("updateCaption", props.location.hoverText)
}

function clearCaption() {
  store.dispatch("updateCaption", "&nbsp;")
}
</script>

<style scoped>
.chest {
  width: 24px;
  height: 24px;
  background-size: 100% 100%;
  position: absolute;
  margin-left: -12px;
  margin-top: -12px;
  background-image: url("/image/poi.png");
}

.chest:hover {
  background-image: url("/image/highlighted.png");
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
