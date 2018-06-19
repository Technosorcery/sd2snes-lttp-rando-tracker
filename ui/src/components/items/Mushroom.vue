<template>
  <div class="item mushroom" :class="mushroomState"></div>
</template>

<script>
export default {
  name: 'Mushroom',
  computed: {
    mushroomState() {
      return this.$store.state.game.mushroom ? 'true' : 'false'
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
