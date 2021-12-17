<template>
  <div class="item shovel" :class="shovelState"></div>
</template>

<script lang="ts">
export default {
  name: 'Shovel',
}
</script>
<script setup lang="ts">
import { computed } from 'vue'
import { useStore } from '../../store'

const store = useStore()
const shovelState = computed(() => store?.state?.game?.shovel ? 'true' : 'false')
</script>

<style scoped>
</style>
