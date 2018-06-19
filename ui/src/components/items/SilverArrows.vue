<template>
  <div class="item silvers" :class="silversState"></div>
</template>

<script>
export default {
  name: 'SilverArrows',
  computed: {
    silversState() {
      return this.$store.state.game.silvers ? 'true' : 'false'
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
