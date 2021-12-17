<template>
  <div class="item book" :class="bookState"></div>
</template>

<script lang="ts">
export default {
  name: 'Book'
}
</script>
<script setup lang="ts">
import { computed } from 'vue'
import { useStore } from '../../store'

const store = useStore()
const bookState = computed(() => store?.state?.game?.book ? 'true' : 'false')
</script>

<style scoped>
</style>
