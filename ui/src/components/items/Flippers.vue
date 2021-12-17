<template>
  <div class="item flippers" :class="flippersState"></div>
</template>

<script lang="ts">
export default {
  name: 'Flippers',
}
</script>
<script setup lang="ts">
import { computed } from 'vue'
import { useStore } from '../../store'

const store = useStore()
const flippersState = computed(() => store?.state?.game?.flippers ? 'true' : 'false')
</script>

<style scoped>
</style>
