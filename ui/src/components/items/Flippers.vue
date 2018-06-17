<template>
  <div class="item flippers" :class="flippersState"></div>
</template>

<script>
export default {
  name: 'Flippers',
  computed: {
    flippersState () {
      return this.$store.state.game.flippers ? 'true' : 'false'
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
