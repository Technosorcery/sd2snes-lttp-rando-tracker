<template>
  <div class="item flute" :class="fluteState"></div>
</template>

<script>
export default {
  name: 'Flute',
  computed: {
    fluteState() {
      if (this.$store.state.game.fluteActivated) {
        return 'true'
      } else if (this.$store.state.game.flute) {
        return 'unactivated'
      }

      return 'false'
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
