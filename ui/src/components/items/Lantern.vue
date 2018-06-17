<template>
  <div class="item lantern" :class="lanternState"></div>
</template>

<script>
export default {
  name: 'Lantern',
  computed: {
    lanternState () {
      return this.$store.state.game.lantern ? 'true' : 'false'
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
