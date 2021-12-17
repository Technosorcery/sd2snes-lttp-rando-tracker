<template>
  <div class="item" :class="shieldState"></div>
</template>

<script lang="ts">
export default {
  name: 'Shield',
}
</script>
<script setup lang="ts">
import { computed } from 'vue'
import { useStore } from '../../store'

const store = useStore()
const shieldState = computed(() => {
  if (!store?.state?.game?.shieldLevel) {
    return
  }
  if (store?.state?.game?.shieldLevel === 'None') {
    return 'noshield'
  }
  return store?.state?.game?.shieldLevel.toLowerCase()
})
</script>

<style scoped>
</style>
