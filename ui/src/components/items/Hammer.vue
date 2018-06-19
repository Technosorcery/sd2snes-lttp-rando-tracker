<template>
  <div class="item hammer" :class="hammerState"></div>
</template>

<script>
export default {
  name: 'Hammer',
  computed: {
    hammerState() {
      return this.$store.state.game.hammer ? 'true' : 'false'
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
