<template>
  <div class="item" :class="shieldState"></div>
</template>

<script>
export default {
  name: 'Shield',
  computed: {
    shieldState() {
      if (!this.$store.state.game.shieldLevel) {
        return
      }
      if (this.$store.state.game.shieldLevel === 'None') {
        return 'noshield'
      }
      return this.$store.state.game.shieldLevel.toLowerCase()
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
