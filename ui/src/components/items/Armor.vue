<template>
  <div class="item" :class="armorState"></div>
</template>

<script>
export default {
  name: 'Armor',
  computed: {
    armorState () {
      if (!this.$store.state.game.armor_level) { return }
      return this.$store.state.game.armor_level.toLowerCase()
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
