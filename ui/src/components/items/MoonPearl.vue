<template>
  <div class="item moonpearl" :class="moonpearlState"></div>
</template>

<script lang="ts">
export default {
  name: 'MoonPearl',
}
</script>
<script setup lang="ts">
import { computed } from 'vue'
import { useStore } from '../../store'

const store = useStore()
const moonpearlState = computed(() => store?.state?.game?.moonPearl ? 'true' : 'false')
</script>

<style scoped>
</style>
