<template>
  <div class="item ether" :class="etherState"></div>
</template>

<script lang="ts">
export default {
  name: 'Ether',
}
</script>
<script setup lang="ts">
import { computed } from 'vue'
import { useStore } from '../../store'

const store = useStore()
const etherState = computed(() => store?.state?.game?.etherMedallion ? 'true' : 'false')
</script>

<style scoped>
</style>
