<template>
  <div class="item silvers" :class="silversState"></div>
</template>

<script lang="ts">
export default {
  name: 'SilverArrows',
}
</script>
<script setup lang="ts">
import { computed } from 'vue'
import { useStore } from '../../store'

const store = useStore()
const silversState = computed(() => store?.state?.game?.silvers ? 'true' : 'false')
</script>

<style scoped>
</style>
