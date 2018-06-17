<template>
  <div class="item" :class="gloveState"></div>
</template>

<script>
export default {
  name: 'Glove',
  computed: {
    gloveState () {
      if (!this.$store.state.game.gloves) { return }
      if (this.$store.state.game.gloves === 'None') { return 'nogloves' }
      return this.$store.state.game.gloves.toLowerCase()
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
