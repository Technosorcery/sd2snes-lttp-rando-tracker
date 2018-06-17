<template>
  <div class="item firerod" :class="firerodState"></div>
</template>

<script>
export default {
  name: 'FireRod',
  computed: {
    firerodState () {
      return this.$store.state.game.fire_rod ? 'true' : 'false'
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
