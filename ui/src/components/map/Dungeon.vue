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
  name: "Dungeon",
}
</script>
<script setup lang="ts">
import { computed, ref } from 'vue'
import { useStore } from '../../store'

const store = useStore()

interface Location {
  name: string,
  hover_text: string,
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
