<template>
  <div class="item somaria" :class="somariaState"></div>
</template>

<script>
export default {
  name: 'Somaria',
  computed: {
    somariaState () {
      return this.$store.state.game.cane_somaria ? 'true' : 'false'
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
