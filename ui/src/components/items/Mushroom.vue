<template>
  <div class="item mushroom" :class="mushroomState"></div>
</template>

<script lang="ts">
export default {
  name: 'Mushroom',
}
</script>
<script setup lang="ts">
import { computed } from 'vue'
import { useStore } from '../../store'

const store = useStore()
const mushroomState = computed(() => store?.state?.game?.mushroom ? 'true' : 'false')
</script>

<style scoped>
</style>
