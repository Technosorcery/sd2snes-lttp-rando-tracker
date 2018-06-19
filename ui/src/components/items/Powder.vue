<template>
  <div class="item powder" :class="powderState"></div>
</template>

<script>
export default {
  name: 'Powder',
  computed: {
    powderState() {
      return this.$store.state.game.powder ? 'true' : 'false'
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
