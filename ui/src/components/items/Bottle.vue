<template>
  <div class="item" :class="bottleState"></div>
</template>

<script lang="ts">
export default {
  name: 'Bottle',
}
</script>
<script setup lang="ts">
import { computed } from 'vue'
import { useStore } from '../../store'

const store = useStore()
const bottleState = computed(() => 'bottle' + store?.state?.game?.bottleCount)
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
