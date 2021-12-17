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

interface Location {
  name: string,
  hoverText: string,
  position: LocationPosition,
  cleared: boolean,
  logic: LocationAvailability,
  availability: Availability,
}

interface LocationPosition {
  horizontal: LocationCoordinates,
  vertical: LocationCoordinates,
}

interface LocationCoordinates {
  left: number,
  top: number,
}

enum LocationAvailability {
  AginahsCave = "AginahsCave",
  BombableHut = "BombableHut",
  BombFairy = "BombFairy",
  BombosTablet = "BombosTablet",
  BottleVendor = "BottleVendor",
  BugKid = "BugKid",
  BumperCave = "BumperCave",
  BuriedItem = "BuriedItem",
  ByrnaSpikeCave = "ByrnaSpikeCave",
  CastleSecretEntrance = "CastleSecretEntrance",
  Catfish = "Catfish",
  CaveUnderRockBottomChest = "CaveUnderRockBottomChest",
  CaveUnderRockThreeTopChests = "CaveUnderRockThreeTopChests",
  CheckerboardCave = "CheckerboardCave",
  ChickenHouse = "ChickenHouse",
  CHouse = "CHouse",
  DarkWorldDeathMountain = "DarkWorldDeathMountain",
  DeathMountainEast = "DeathMountainEast",
  DesertWestLedge = "DesertWestLedge",
  DigGame = "DigGame",
  EscapeSewer = "EscapeSewer",
  EtherTablet = "EtherTablet",
  FloatingIsland = "FloatingIsland",
  ForestHideout = "ForestHideout",
  FugitiveUnderTheBridge = "FugitiveUnderTheBridge",
  GraveyardCliffCave = "GraveyardCliffCave",
  Hammers = "Hammers",
  HypeCave = "HypeCave",
  HyruleCastle = "HyruleCastle",
  IceRodCave = "IceRodCave",
  KakarikoWell = "KakarikoWell",
  KingsTomb = "KingsTomb",
  KingZora = "KingZora",
  LakeHyliaIsland = "LakeHyliaIsland",
  Library = "Library",
  LightWorldSwamp = "LightWorldSwamp",
  LinksHouse = "LinksHouse",
  LostOldMan = "LostOldMan",
  LumberjackTree = "LumberjackTree",
  MadBatter = "MadBatter",
  MasterSwordPedestal = "MasterSwordPedestal",
  MimicCave = "MimicCave",
  MinimoldormCave = "MinimoldormCave",
  Mushroom = "Mushroom",
  PurpleChest = "PurpleChest",
  Pyramid = "Pyramid",
  RaceMinigame = "RaceMinigame",
  Sahasrahla = "Sahasrahla",
  SahasrahlasHut = "SahasrahlasHut",
  Sanctuary = "Sanctuary",
  SouthOfGrove = "SouthOfGrove",
  SpectacleRock = "SpectacleRock",
  SpectacleRockCave = "SpectacleRockCave",
  SpiralCave = "SpiralCave",
  StumpKid = "StumpKid",
  TakeTheFrogHome = "TakeTheFrogHome",
  Tavern = "Tavern",
  ThievesHut = "ThievesHut",
  TreasureChestMinigame = "TreasureChestMinigame",
  WaterfallOfTheWishing = "WaterfallOfTheWishing",
  WestOfMire = "WestOfMire",
  WestOfSanctuary = "WestOfSanctuary",
  Witch = "Witch",
  ZoraRiverLedge = "ZoraRiverLedge",
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
