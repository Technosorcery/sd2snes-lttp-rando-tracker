<template>
  <div class="item bow" :class="bowState"></div>
</template>

<script lang="ts">
export default {
  name: 'Bow',
}
</script>
<script setup lang="ts">
import { computed } from 'vue'
import { useStore } from '../../store'

const store = useStore()
const bowState = computed(() => store?.state?.game?.bow ? 'true' : 'false')
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
