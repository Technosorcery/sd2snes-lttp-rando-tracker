<template>
  <div class="item" :class="armorState"></div>
</template>

<script>
export default {
  name: 'Armor',
  computed: {
    armorState() {
      if (!this.$store.state.game.armorLevel) return

      return this.$store.state.game.armorLevel.toLowerCase()
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
