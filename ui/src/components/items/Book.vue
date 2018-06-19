<template>
  <div class="item book" :class="bookState"></div>
</template>

<script>
export default {
  name: 'Book',
  computed: {
    bookState() {
      return this.$store.state.game.book ? 'true' : 'false'
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
