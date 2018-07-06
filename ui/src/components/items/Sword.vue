<template>
  <div class="item" :class="swordState"></div>
</template>

<script>
export default {
  name: 'Sword',
  computed: {
    swordState() {
      if (!this.$store.state.game.swordLevel) {
        return
      }
      if (this.$store.state.game.swordLevel === 'None') {
        return 'nosword'
      }
      return this.$store.state.game.swordLevel.toLowerCase()
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
