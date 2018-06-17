<template>
  <div class="item bombos" :class="bombosState"></div>
</template>

<script>
export default {
  name: 'Bombos',
  computed: {
    bombosState () {
      return this.$store.state.game.bombos_medallion ? 'true' : 'false'
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
