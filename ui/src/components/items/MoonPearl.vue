<template>
  <div class="item moonpearl" :class="moonpearlState"></div>
</template>

<script>
export default {
  name: 'MoonPearl',
  computed: {
    moonpearlState() {
      return this.$store.state.game.moon_pearl ? 'true' : 'false'
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
