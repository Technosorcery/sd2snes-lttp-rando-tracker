<template>
  <div class="item icerod" :class="icerodState"></div>
</template>

<script lang="ts">
export default {
  name: 'IceRod',
}
</script>
<script setup lang="ts">
import { computed } from 'vue'
import { useStore } from '../../store'

const store = useStore()
const icerodState = computed(() => store?.state?.game?.iceRod ? 'true' : 'false')
</script>

<style scoped>
</style>
