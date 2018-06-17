<template>
  <div class="item" :class="bottleState"></div>
</template>

<script>
export default {
  name: 'Bottle',
  computed: {
    bottleState () {
      return 'bottle' + this.$store.state.game.bottle_count
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
