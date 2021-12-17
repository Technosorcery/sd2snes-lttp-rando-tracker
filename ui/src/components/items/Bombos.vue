<template>
  <div class="item bombos" :class="bombosState"></div>
</template>

<script lang="ts">
export default {
  name: 'Bombos'
}
</script>
<script setup lang="ts">
import { computed } from 'vue'
import { useStore } from '../../store'

const store = useStore()
const bombosState = computed(() => store?.state?.game?.bombosMedallion ? 'true' : 'false')
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
