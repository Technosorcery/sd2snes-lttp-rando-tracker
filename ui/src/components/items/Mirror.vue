<template>
  <div class="item mirror" :class="mirrorState"></div>
</template>

<script lang="ts">
export default {
  name: 'Mirror',
}
</script>
<script setup lang="ts">
import { computed } from 'vue'
import { useStore } from '../../store'

const store = useStore()
const mirrorState = computed(() => store?.state?.game?.mirror ? 'true' : 'false')
</script>

<style scoped>
</style>
