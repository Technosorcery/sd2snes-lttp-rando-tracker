<template>
  <div class="item quake" :class="quakeState"></div>
</template>

<script>
export default {
  name: 'Quake',
  computed: {
    quakeState() {
      return this.$store.state.game.quakeMedallion ? 'true' : 'false'
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
