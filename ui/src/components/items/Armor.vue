<template>
  <div class="item" :class="armorState"></div>
</template>

<script lang="ts">
export default {
  name: 'Armor',
}
</script>
<script setup lang="ts">
import { computed } from 'vue'
import { useStore } from '../../store'

const store = useStore()
const armorState = computed(() => store?.state?.game?.armorLevel.toLowerCase())
</script>

<style scoped>
</style>
