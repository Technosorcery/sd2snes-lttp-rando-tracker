<template>
  <div class="item hammer" :class="hammerState"></div>
</template>

<script lang="ts">
export default {
  name: 'Hammer',
}
</script>
<script setup lang="ts">
import { computed } from 'vue'
import { useStore } from '../../store'

const store = useStore()
const hammerState = computed(() => store?.state?.game?.hammer ? 'true' : 'false')
</script>

<style scoped>
</style>
