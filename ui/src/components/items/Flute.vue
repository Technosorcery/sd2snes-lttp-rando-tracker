<template>
  <div class="item flute" :class="fluteState"></div>
</template>

<script lang="ts">
export default {
  name: 'Flute',
}
</script>
<script setup lang="ts">
import { computed } from 'vue'
import { useStore } from '../../store'

const store = useStore()
const fluteState = computed(() => {
  if (store?.state?.game?.fluteActivated) {
    return 'true'
  } else if (store?.state?.game?.flute) {
    return 'unactivated'
  }

  return 'false'
})
</script>

<style scoped>
</style>
