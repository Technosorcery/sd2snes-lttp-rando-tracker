<template>
  <div class="item-tracker">
    <div class="items">
      <div v-if="showItems" v-for="(row, row_index) in items" class="tracker-row" :key="row_index">
        <div class="row-spacer" :style="spacerStyle(row, items)"></div>
        <component v-for="item in row" :key="item.name" :is="item"></component>
      </div>
    </div>
    <div class="dungeons">
      <div v-if="showMap" v-for="(row, index) in dungeons" class="tracker-row" :key="index">
        <div class="row-spacer" :style="spacerStyle(row, items)"></div>
        <Dungeon
          v-for="dungeon in dungeonsForRow(row)"
          :key="dungeon.dungeonCode"
          :dungeon="dungeon"
        ></Dungeon>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
export default {
  name: 'ItemTracker',
}
</script>
<script setup lang="ts">
import { computed } from 'vue'
import { useStore } from "../store";
import Armor from "@/components/items/Armor.vue";
import Blank from "@/components/items/Blank.vue";
import Bombos from "@/components/items/Bombos.vue";
import Book from "@/components/items/Book.vue";
import Boots from "@/components/items/Boots.vue";
import Bottle from "@/components/items/Bottle.vue";
import Bow from "@/components/items/Bow.vue";
import Cape from "@/components/items/Cape.vue";
import Dungeon from "@/components/items/Dungeon.vue";
import Ether from "@/components/items/Ether.vue";
import FireRod from "@/components/items/FireRod.vue";
import Flippers from "@/components/items/Flippers.vue";
import Flute from "@/components/items/Flute.vue";
import Glove from "@/components/items/Glove.vue";
import Hammer from "@/components/items/Hammer.vue";
import HookShot from "@/components/items/HookShot.vue";
import IceRod from "@/components/items/IceRod.vue";
import Lantern from "@/components/items/Lantern.vue";
import Mirror from "@/components/items/Mirror.vue";
import MoonPearl from "@/components/items/MoonPearl.vue";
import Mushroom from "@/components/items/Mushroom.vue";
import Powder from "@/components/items/Powder.vue";
import Quake from "@/components/items/Quake.vue";
import Shield from "@/components/items/Shield.vue";
import Shovel from "@/components/items/Shovel.vue";
import SilverArrows from "@/components/items/SilverArrows.vue";
import Somaria from "@/components/items/Somaria.vue";
import Sword from "@/components/items/Sword.vue";

const components = {
  Armor,
  Blank,
  Bombos,
  Book,
  Boots,
  Bottle,
  Bow,
  Cape,
  Dungeon,
  Ether,
  FireRod,
  Flippers,
  Flute,
  Glove,
  Hammer,
  HookShot,
  IceRod,
  Lantern,
  Mirror,
  MoonPearl,
  Mushroom,
  Powder,
  Quake,
  Shield,
  Shovel,
  SilverArrows,
  Somaria,
  Sword,
}

const store = useStore()

const showItems = computed(() => !!store.state.game)
const showMap = computed(() => store.getters.mappableDungeons.length > 0 && store.getters.mappableLocations.length > 0)

const items = [
  [
    Bow,
    HookShot,
    Hammer,
    FireRod,
    Glove,
    MoonPearl,
  ],
  [
    Somaria,
    Lantern,
    Flute,
    Book,
    Boots,
    Flippers,
    Mirror,
  ],
  [
    Sword,
    Armor,
    Shield,
    Blank,
    Bombos,
    Ether,
    Quake,
  ],
  [
    Shovel,
    Mushroom,
    Powder,
    Bottle,
    Cape,
    IceRod,
    SilverArrows,
  ],
]

const dungeons = [
  ["PoD", "SP", "SW", "TT", "IP", "MM", "TR"],
  ["EP", "DP", "ToH", "Aga"],
]

function spacerStyle(currentRow: any[], allRows: any[]) {
  let maxRowLen = Math.max.apply(
    Math,
    allRows.map(function (item) {
      return item.length;
    })
  );
  let padding = (64 * maxRowLen - 64 * currentRow.length) / 2;

  return "width: " + padding + "px;";
}

function dungeonForCode(code: string) {
  return store.getters.getDungeonByDungeonCode(code);
}

function dungeonsForRow(row: string[]) {
  let foundDungeons = [];
  for (var i = 0, numDungeons = row.length; i < numDungeons; i++) {
    let dungeon = dungeonForCode(row[i]);
    if (dungeon) {
      foundDungeons.push(dungeon);
    }
  }

  return foundDungeons;
}
</script>

<style scoped>
.item-tracker {
  left: 0;
  position: relative;
  box-sizing: border-box;
}

.item-tracker .row-spacer {
  display: inline-block;
}
</style>
