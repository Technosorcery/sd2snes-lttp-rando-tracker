<template>
  <div class="item lantern" :class="lanternState"></div>
</template>

<script lang="ts">
export default {
  name: 'Lantern',
}
</script>
<script setup lang="ts">
import { computed } from 'vue'
import { useStore } from '../../store'

const store = useStore()
const lanternState = computed(() => store?.state?.game?.lantern ? 'true' : 'false')
</script>

<style scoped>
</style>
