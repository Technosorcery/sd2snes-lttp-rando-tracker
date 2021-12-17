<template>
  <div class="item powder" :class="powderState"></div>
</template>

<script lang="ts">
export default {
  name: 'Powder',
}
</script>
<script setup lang="ts">
import { computed } from 'vue'
import { useStore } from '../../store'

const store = useStore()
const powderState = computed(() => store?.state?.game?.powder ? 'true' : 'false')
</script>

<style scoped>
</style>
