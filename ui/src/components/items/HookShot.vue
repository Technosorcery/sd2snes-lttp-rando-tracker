<template>
  <div class="item hookshot" :class="hookshotState"></div>
</template>

<script lang="ts">
export default {
  name: 'HookShot',
}
</script>
<script setup lang="ts">
import { computed } from 'vue'
import { useStore } from '../../store'

const store = useStore()
const hookshotState = computed(() => store?.state?.game?.hookShot ? 'true' : 'false')
</script>

<style scoped>
</style>
