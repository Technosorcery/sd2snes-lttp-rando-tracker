<template>
  <div class="item boots" :class="bootsState"></div>
</template>

<script lang="ts">
export default {
  name: 'Boots',
}
</script>
<script setup lang="ts">
import { computed } from 'vue'
import { useStore } from '../../store'

const store = useStore()
const bootsState = computed(() => store?.state?.game?.boots ? 'true' : 'false')
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
