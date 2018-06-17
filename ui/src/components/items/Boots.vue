<template>
  <div class="item boots" :class="bootsState"></div>
</template>

<script>
export default {
  name: 'Boots',
  computed: {
    bootsState () {
      return this.$store.state.game.boots ? 'true' : 'false'
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
