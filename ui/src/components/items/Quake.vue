<template>
  <div class="item quake" :class="quakeState"></div>
</template>

<script lang="ts">
export default {
  name: 'Quake',
}
</script>
<script setup lang="ts">
import { computed } from 'vue'
import { useStore } from '../../store'

const store = useStore()
const quakeState = computed(() => store?.state?.game?.quakeMedallion ? 'true' : 'false')
</script>

<style scoped>
</style>
