<template>
  <div class="item flute" :class="fluteState"></div>
</template>

<script>
export default {
  name: 'Flute',
  computed: {
    fluteState() {
      return this.$store.state.game.flute ? 'true' : 'false'
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
