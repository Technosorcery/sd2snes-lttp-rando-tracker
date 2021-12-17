<template>
  <div class="item somaria" :class="somariaState"></div>
</template>

<script lang="ts">
export default {
  name: 'Somaria',
}
</script>
<script setup lang="ts">
import { computed } from 'vue'
import { useStore } from '../../store'

const store = useStore()
const somariaState = computed(() => store?.state?.game?.caneSomaria ? 'true' : 'false')
</script>

<style scoped>
</style>
