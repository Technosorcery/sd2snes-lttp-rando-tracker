<script setup lang="ts">
import ItemTracker from '@/components/ItemTracker.vue'
import MapTracker from '@/components/MapTracker.vue'

import { useStore } from '../store'

const store = useStore()
</script>

<template>
  <div class="dashboard">
    <ItemTracker />
    <MapTracker />
  </div>
</template>

<style scoped>
</style>
