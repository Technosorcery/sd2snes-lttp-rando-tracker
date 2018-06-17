<template>
  <div class="item mirror" :class="mirrorState"></div>
</template>

<script>
export default {
  name: 'Mirror',
  computed: {
    mirrorState () {
      return this.$store.state.game.mirror ? 'true' : 'false'
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
