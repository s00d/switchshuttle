<template>
  <div class="flex flex-col items-center justify-center h-screen w-screen bg-white">
    <div class="w-10 h-10 border-4 border-gray-200 border-t-blue-500 rounded-full animate-spin mb-4"></div>
    <div class="text-sm text-gray-600">loading...</div>
  </div>
</template>

<script setup>
import { onMounted } from 'vue';

onMounted(() => {
  // Add any initialization logic if needed
});
</script>
