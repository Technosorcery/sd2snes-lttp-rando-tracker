<template>
  <div class="item" :class="swordState"></div>
</template>

<script lang="ts">
export default {
  name: 'Sword',
}
</script>
<script setup lang="ts">
import { computed } from 'vue'
import { useStore } from '../../store'

const store = useStore()
const swordState = computed(() => {
  if (!store?.state?.game?.swordLevel) {
    return
  }
  if (store?.state?.game?.swordLevel === 'None') {
    return 'nosword'
  }
  return store?.state?.game?.swordLevel.toLowerCase()
})
</script>

<style scoped>
</style>
