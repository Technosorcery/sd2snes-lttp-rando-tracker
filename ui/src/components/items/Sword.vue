<template>
  <div class="item" :class="swordState"></div>
</template>

<script>
export default {
  name: 'Sword',
  computed: {
    swordState() {
      if (!this.$store.state.game.sword_level) {
        return
      }
      if (this.$store.state.game.sword_level === 'None') {
        return 'nosword'
      }
      return this.$store.state.game.sword_level.toLowerCase()
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
