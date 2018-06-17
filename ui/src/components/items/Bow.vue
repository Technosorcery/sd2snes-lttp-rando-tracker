<template>
  <div class="item bow" :class="bowState"></div>
</template>

<script>
export default {
  name: 'Bow',
  computed: {
    bowState () {
      return this.$store.state.game.bow ? 'true' : 'false'
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
