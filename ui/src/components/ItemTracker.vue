<template>
  <div class="item-tracker">
    <div class="items">
      <div v-for="(row, index) in items"
        class="tracker-row"
        :key="index">
        <div v-for="item in row"
             :key="item.name"
             :class="item.classFn"
             class="item"></div>
      </div>
      <div class="tracker-row">
        <div class="item bow" :class="bowState"></div>
        <div class="item hookshot" :class="hookshotState"></div>
        <div class="item hammer" :class="hammerState"></div>
        <div class="item firerod" :class="firerodState"></div>
        <div class="item" :class="gloveState"></div>
        <div class="item moonpearl" :class="moonpearlState"></div>
      </div>
      <div class="tracker-row">
        <div class="item somaria" :class="somariaState"></div>
        <div class="item lantern" :class="lanternState"></div>
        <div class="item flute" :class="fluteState"></div>
        <div class="item book" :class="bookState"></div>
        <div class="item boots" :class="bootsState"></div>
        <div class="item flippers" :class="flippersState"></div>
        <div class="item mirror" :class="mirrorState"></div>
      </div>
      <div class="tracker-row">
        <div class="item" :class="swordState"></div>
        <div class="item" :class="armorState"></div>
        <div class="item" :class="shieldState"></div>
        <div class="item"></div>
        <div class="item bombos" :class="bombosState"></div>
        <div class="item ether" :class="etherState"></div>
        <div class="item quake" :class="quakeState"></div>
      </div>
      <div class="tracker-row">
        <div class="item shovel" :class="shovelState"></div>
        <div class="item mushroom" :class="mushroomState"></div>
        <div class="item powder" :class="powderState"></div>
        <div class="item" :class="bottleState"></div>
        <div class="item cape" :class="capeState"></div>
        <div class="item icerod" :class="icerodState"></div>
        <div class="item silvers" :class="silversState"></div>
      </div>
    </div>
    <div class="dungeons">
      <div class="tracker-row">
        <img src="../assets/boss32.png">
        <img src="../assets/boss42.png">
        <img src="../assets/boss52.png">
        <img src="../assets/boss62.png">
        <img src="../assets/boss72.png">
        <img src="../assets/boss82.png">
        <img src="../assets/boss92.png">
      </div>
      <div class="tracker-row">
        <img src="../assets/boss02.png">
        <img src="../assets/boss12.png">
        <img src="../assets/boss22.png">
        <img src="../assets/agahnim0.png">
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: 'ItemTracker',
  data () {
    return {
      items: [
        [
          { name: 'bow', classFn: this.bowClass },
          { name: 'hookshot', classFn: 'hookshotClass' },
          { name: 'hammer', classFn: 'hammerClass' },
          { name: 'firerod', classFn: 'firerodClass' },
          { name: 'glove', classFn: 'gloveClass' },
          { name: 'moonpearl', classFn: 'moonpearlClass' }
        ],
        [
          { name: 'somaria', classFn: 'somariaClass' },
          { name: 'lantern', classFn: 'lanternClass' },
          { name: 'flute', classFn: 'fluteClass' },
          { name: 'book', classFn: 'bookClass' },
          { name: 'boots', classFn: 'bootsClass' },
          { name: 'flippers', classFn: 'flippersClass' },
          { name: 'mirror', classFn: 'mirrorClass' }
        ]
      ]
    }
  },
  computed: {
    bowState () {
      return this.$store.state.game.bow ? 'true' : 'false'
    },
    bowClass () {
      return 'bow ' + this.bowState()
    },

    hookshotState () {
      return this.$store.state.game.hook_shot ? 'true' : 'false'
    },
    hookshotClass () {
      return 'hookshot ' + this.hookshotState()
    },

    hammerState () {
      return this.$store.state.game.hammer ? 'true' : 'false'
    },
    hammerClass () {
      return 'hammer ' + this.hammerState()
    },

    firerodState () {
      return this.$store.state.game.fire_rod ? 'true' : 'false'
    },

    gloveState () {
      if (!this.$store.state.game.gloves) { return }
      if (this.$store.state.game.gloves === 'None') { return 'nogloves' }
      return this.$store.state.game.gloves.toLowerCase()
    },

    moonpearlState () {
      return this.$store.state.game.moon_pearl ? 'true' : 'false'
    },

    somariaState () {
      return this.$store.state.game.cane_somaria ? 'true' : 'false'
    },

    lanternState () {
      return this.$store.state.game.lantern ? 'true' : 'false'
    },

    fluteState () {
      return this.$store.state.game.flute ? 'true' : 'false'
    },

    bookState () {
      return this.$store.state.game.book ? 'true' : 'false'
    },

    bootsState () {
      return this.$store.state.game.boots ? 'true' : 'false'
    },

    flippersState () {
      return this.$store.state.game.flippers ? 'true' : 'false'
    },

    mirrorState () {
      return this.$store.state.game.mirror ? 'true' : 'false'
    },

    swordState () {
      if (!this.$store.state.game.sword_level) { return }
      if (this.$store.state.game.sword_level === 'None') { return 'nosword' }
      return this.$store.state.game.sword_level.toLowerCase()
    },

    armorState () {
      if (!this.$store.state.game.armor_level) { return }
      return this.$store.state.game.armor_level.toLowerCase()
    },

    shieldState () {
      if (!this.$store.state.game.shield_level) { return }
      if (this.$store.state.game.shield_level === 'None') { return 'noshield' }
      return this.$store.state.game.shield_level.toLowerCase()
    },

    bombosState () {
      return this.$store.state.game.bombos_medallion ? 'true' : 'false'
    },

    etherState () {
      return this.$store.state.game.ether_medallion ? 'true' : 'false'
    },

    quakeState () {
      return this.$store.state.game.quake_medallion ? 'true' : 'false'
    },

    shovelState () {
      return this.$store.state.game.shovel ? 'true' : 'false'
    },

    mushroomState () {
      return this.$store.state.game.mushroom ? 'true' : 'false'
    },

    powderState () {
      return this.$store.state.game.powder ? 'true' : 'false'
    },

    bottleState () {
      return 'bottle' + this.$store.state.game.bottle_count
    },

    capeState () {
      return this.$store.state.game.cape ? 'true' : 'false'
    },

    icerodState () {
      return this.$store.state.game.ice_rod ? 'true' : 'false'
    },

    silversState () {
      return this.$store.state.game.silvers ? 'true' : 'false'
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
