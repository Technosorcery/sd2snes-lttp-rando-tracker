<template>
  <div class="item cape" :class="capeState"></div>
</template>

<script lang="ts">
export default {
  name: 'Cape',
}
</script>
<script setup lang="ts">
import { computed } from 'vue'
import { useStore } from '../../store'

const store = useStore()
const capeState = computed(() => store?.state?.game?.cape ? 'true' : 'false')
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
