<template>
  <div
    class="flex flex-col items-center justify-center h-screen w-screen bg-slate-50"
  >
    <div
      class="w-16 h-16 border-4 border-slate-200 border-t-blue-500 rounded-full animate-spin mb-6 shadow-lg"
    ></div>
    <div class="text-lg text-slate-600 font-medium">Loading...</div>
    <div class="text-sm text-slate-500 mt-2">
      Please wait while we prepare your application
    </div>
  </div>
</template>

<script setup>
import { onMounted } from 'vue';
import { getCurrentWindow } from '@tauri-apps/api/window';

// Function to close the window (currently unused)
// function onClose() {
//   getCurrentWindow().close();
// }

onMounted(() => {
  // Add any initialization logic if needed
});
</script>
