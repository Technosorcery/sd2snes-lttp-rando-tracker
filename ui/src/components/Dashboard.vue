<script setup lang="ts">
import { ref } from 'vue'
import ItemTracker from '@/components/ItemTracker.vue'
import MapTracker from '@/components/MapTracker.vue'
import SettingsModal from '@/components/SettingsModal.vue'

const showSettings = ref(false)
</script>

<template>
  <div class="dashboard">
    <ItemTracker />
    <MapTracker />
    <SettingsModal v-if="showSettings" @close="showSettings = false"></SettingsModal>
    <button id="show-settings" class="fixed bottom-0 right-0 rounded-lg p-1 m-2 border-4" @click="showSettings = true">Settings</button>
  </div>
</template>

<style scoped>
</style>
