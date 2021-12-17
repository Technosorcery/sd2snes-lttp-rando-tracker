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

<script lang="ts">
export default {
  name: 'Dungeon',
}
</script>
<script setup lang="ts">
import { computed } from 'vue'
import { useStore } from '../../store'

interface DungeonBoss {
  name: string,
  hoverText: string,
  imageNumber: string,
}

enum DungeonReward {
  Unknown = "Unknown",
  GreenPendant = "GreenPendant",
  RedBluePendant = "RedBluePendant",
  Crystal = "Crystal",
  RedCrystal = "RedCrystal",
}

enum Medallion {
  Unknown = "Unknown",
  Bombos = "Bombos",
  Ether = "Ether",
  Quake = "Quake",
}

interface LocationPosition {
  horizontal: LocationCoordinates,
  vertical: LocationCoordinates,
}

interface LocationCoordinates {
  left: number,
  top: number,
}

enum DungeonAvailability {
  DesertPalace = "DesertPalace",
  EasternPalace = "EasternPalace",
  GanonsTower = "GanonsTower",
  IcePalace = "IcePalace",
  MiseryMire = "MiseryMire",
  PalaceOfDarkness = "PalaceOfDarkness",
  SkullWoods = "SkullWoods",
  SwampPalace = "SwampPalace",
  ThievesTown = "ThievesTown",
  TowerOfHera = "TowerOfHera",
  TurtleRock = "TurtleRock",
}

enum Availability {
  Unavailable = "Unavailable",
  GlitchPossible = "GlitchPossible",
  Possible = "Possible",
  GlitchAgahnim = "GlitchAgahnim",
  Agahnim = "Agahnim",
  GlitchAvailable = "GlitchAvailable",
  Available = "Available",
}

interface Dungeon {
  name: string,
  dungeonCode: string,
  hoverText: string,
  totalChests: number,
  clearedImage: string,
  defaultImage: string,
  hasReward: boolean,
  position?: LocationPosition,
  boss?: DungeonBoss,
  foundChests: number,
  reward: DungeonReward,
  medallion: Medallion,
  cleared: boolean,
  dungeonAvailability: Availability,
  bossAvailability: Availability,
  logic?: DungeonAvailability,
}

interface Props {
  dungeon: Dungeon,
}

const props = defineProps<Props>()
const store = useStore()
const sequences = {
  medallion: ['Unknown', 'Bombos', 'Ether', 'Quake'],
  reward: [
    'Unknown',
    'GreenPendant',
    'RedBluePendant',
    'Crystal',
    'RedCrystal'
  ],
}

const sequenceNumbers = computed((): {medallion: number, reward: number} => {
  return {
    medallion: sequences["medallion"].indexOf(props.dungeon.medallion),
    reward: sequences["reward"].indexOf(props.dungeon.reward),
  }
})
const getSequenceNumber = <T extends object, U extends keyof T>(obj: T) => (key: U) => obj[key]

const dungeonClass = computed(() => {
  if (props.dungeon.name === 'Aga' && props.dungeon.cleared) {
    return 'dungeon false'
  } else {
    return 'dungeon'
  }
})
const displayStyle = computed(() => 'background-image: url(' + displayImage.value + ')')
const chestsStyle = computed(() => 'background-image: url(' + chestImage.value + ')')
const rewardStyle = computed(() => 'background-image: url(' + rewardImage.value + ')')
const medallionStyle = computed(() => 'background-image: url(' + medallionImage.value + ')')
const chestImage = computed(() => '/image/chest' + (props.dungeon.totalChests - props.dungeon.foundChests) + '.png')
const medallionImage = computed(() => '/image/medallion' + sequenceNumbers.value.medallion + '.png')
const rewardImage = computed(() => '/image/dungeon' + sequenceNumbers.value.reward + '.png')
const displayImage = computed(() => {
  if (props.dungeon.cleared) {
    return '/image/' + props.dungeon.clearedImage
  } else {
    return '/image/' + props.dungeon.defaultImage
  }
})
const medallionRequired = computed(() => {
  switch (props.dungeon.dungeonCode) {
    case 'MM':
    case 'TR':
      return true
    default:
      return false
  }
})
const title = computed(() => {
  if (props.dungeon.dungeonCode === 'Aga') {
    return false
  } else {
    return props.dungeon.dungeonCode
  }
})

function cycleMedallion(event: MouseEvent) {
  event.stopPropagation()

  cycleSequence('medallion')
}

function openChest(event: MouseEvent) {
  event.stopPropagation()

  let newFoundChests = props.dungeon.foundChests + 1
  if (newFoundChests > props.dungeon.totalChests) {
    newFoundChests = 0
  }

  let data = { foundChests: newFoundChests }
  updateServerState(data)
}

function cycleReward(event: MouseEvent) {
  event.stopPropagation()

  cycleSequence('reward')
}

function cycleSequence(sequence: "medallion" | "reward") {
  let newIndex = sequenceNumbers.value[sequence] + 1
  if (newIndex >= sequences[sequence].length) {
    newIndex = 0
  }

  let update_data: { medallion?: string, reward?: string } = {}
  update_data[sequence] = sequences[sequence][newIndex]
  updateServerState(update_data)
}

function toggleCleared(event: MouseEvent) {
  event.stopPropagation()

  let cleared = !props.dungeon.cleared
  let data = { cleared: cleared }
  updateServerState(data)
}

function updateServerState(data: object) {
  let host = window.location.hostname + ':' + store?.state?.serverConfig?.apiPort

  var xhr = new XMLHttpRequest()
  xhr.open('POST', 'http://' + host + '/api/dungeon_state/' + props.dungeon.dungeonCode, true)
  xhr.setRequestHeader('Content-Type', 'application/json')
  xhr.send(JSON.stringify(data))
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
