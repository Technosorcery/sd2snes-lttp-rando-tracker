<template>
  <div class="item-tracker">
    <div class="items">
      <div
        v-for="(row, index) in items"
        class="tracker-row"
        :key="index">
        <div
          class="row-spacer"
          :style="spacerStyle(row, items)"></div>
        <component
          v-for="item in row"
          :key="item.name"
          :is="item.name"></component>
      </div>
    </div>
    <div class="dungeons">
      <div
        v-for="(row, index) in dungeons"
        class="tracker-row"
        :key="index">
        <div
          class="row-spacer"
          :style="spacerStyle(row, items)"></div>
        <Dungeon
          v-for="dungeon in row"
          :key="dungeon"
          :name="dungeon"></Dungeon>
      </div>
    </div>
  </div>
</template>

<script>
import Armor from '@/components/items/Armor'
import Blank from '@/components/items/Blank'
import Bombos from '@/components/items/Bombos'
import Book from '@/components/items/Book'
import Boots from '@/components/items/Boots'
import Bottle from '@/components/items/Bottle'
import Bow from '@/components/items/Bow'
import Cape from '@/components/items/Cape'
import Dungeon from '@/components/items/Dungeon'
import Ether from '@/components/items/Ether'
import FireRod from '@/components/items/FireRod'
import Flippers from '@/components/items/Flippers'
import Flute from '@/components/items/Flute'
import Glove from '@/components/items/Glove'
import Hammer from '@/components/items/Hammer'
import HookShot from '@/components/items/HookShot'
import IceRod from '@/components/items/IceRod'
import Lantern from '@/components/items/Lantern'
import Mirror from '@/components/items/Mirror'
import MoonPearl from '@/components/items/MoonPearl'
import Mushroom from '@/components/items/Mushroom'
import Powder from '@/components/items/Powder'
import Quake from '@/components/items/Quake'
import Shield from '@/components/items/Shield'
import Shovel from '@/components/items/Shovel'
import SilverArrows from '@/components/items/SilverArrows'
import Somaria from '@/components/items/Somaria'
import Sword from '@/components/items/Sword'

export default {
  name: 'ItemTracker',
  components: {
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
    Sword
  },

  data() {
    return {
      items: [
        [
          { name: Bow.name },
          { name: HookShot.name },
          { name: Hammer.name },
          { name: FireRod.name },
          { name: Glove.name },
          { name: MoonPearl.name }
        ],
        [
          { name: Somaria.name },
          { name: Lantern.name },
          { name: Flute.name },
          { name: Book.name },
          { name: Boots.name },
          { name: Flippers.name },
          { name: Mirror.name }
        ],
        [
          { name: Sword.name },
          { name: Armor.name },
          { name: Shield.name },
          { name: Blank.name },
          { name: Bombos.name },
          { name: Ether.name },
          { name: Quake.name }
        ],
        [
          { name: Shovel.name },
          { name: Mushroom.name },
          { name: Powder.name },
          { name: Bottle.name },
          { name: Cape.name },
          { name: IceRod.name },
          { name: SilverArrows.name }
        ]
      ],
      dungeons: [
        ['PoD', 'SP', 'SW', 'TT', 'IP', 'MM', 'TR'],
        ['EP', 'DP', 'ToH', 'Aga']
      ]
    }
  },

  methods: {
    spacerStyle(currentRow, allRows) {
      let maxRowLen = Math.max.apply(Math, allRows.map(function (item) { return item.length }))
      let padding = ((64 * maxRowLen) - (64 * currentRow.length)) / 2

      return 'width: ' + padding + 'px;'
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
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
