<template>
  <div class="item hookshot" :class="hookshotState"></div>
</template>

<script>
export default {
  name: 'HookShot',
  computed: {
    hookshotState() {
      return this.$store.state.game.hookShot ? 'true' : 'false'
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
