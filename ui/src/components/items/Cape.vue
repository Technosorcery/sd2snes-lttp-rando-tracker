<template>
  <div class="item cape" :class="capeState"></div>
</template>

<script>
export default {
  name: 'Cape',
  computed: {
    capeState () {
      return this.$store.state.game.cape ? 'true' : 'false'
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
