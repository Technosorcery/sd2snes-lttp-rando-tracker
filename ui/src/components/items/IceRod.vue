<template>
  <div class="item icerod" :class="icerodState"></div>
</template>

<script>
export default {
  name: 'IceRod',
  computed: {
    icerodState() {
      return this.$store.state.game.iceRod ? 'true' : 'false'
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
