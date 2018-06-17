<template>
  <div class="item ether" :class="etherState"></div>
</template>

<script>
export default {
  name: 'Ether',
  computed: {
    etherState () {
      return this.$store.state.game.ether_medallion ? 'true' : 'false'
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
