<template>
  <div class="item firerod" :class="firerodState"></div>
</template>

<script lang="ts">
export default {
  name: 'FireRod',
}
</script>
<script setup lang="ts">
import { computed } from 'vue'
import { useStore } from '../../store'

const store = useStore()
const firerodState = computed(() => store?.state?.game?.fireRod ? 'true' : 'false')
</script>

<style scoped>
</style>
