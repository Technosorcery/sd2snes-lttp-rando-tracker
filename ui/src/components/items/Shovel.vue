<template>
  <div class="item shovel" :class="shovelState"></div>
</template>

<script>
export default {
  name: 'Shovel',
  computed: {
    shovelState() {
      return this.$store.state.game.shovel ? 'true' : 'false'
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
