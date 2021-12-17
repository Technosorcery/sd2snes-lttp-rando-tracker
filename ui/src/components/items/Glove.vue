<template>
  <div class="item" :class="gloveState"></div>
</template>

<script lang="ts">
export default {
  name: 'Glove',
}
</script>
<script setup lang="ts">
import { computed } from 'vue'
import { useStore } from '../../store'

const store = useStore()
const gloveState = computed(() => {
  if (!store?.state?.game?.gloves) {
    return
  }
  if (store?.state?.game?.gloves === 'None') {
    return 'nogloves'
  }
  return store?.state?.game?.gloves.toLowerCase()
})
</script>

<style scoped>
</style>
