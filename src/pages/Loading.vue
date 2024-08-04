<template>
  <div class="container">
        <div id="loader" class="loader"></div>
        <div id="loader-text">loading...</div>
      </div>
</template>

<script setup>
import { onMounted } from 'vue';
import { getCurrentWindow } from '@tauri-apps/api/window';

function onClose() {
  getCurrentWindow().close();
}

onMounted(() => {
  // Add any initialization logic if needed
});
</script>

<style scoped>
/* Add styles from loading.html */
.container {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100vh;
  background-color: #f0f0f5;
}

.loader {
  border: 4px solid #f3f3f3;
  border-radius: 50%;
  border-top: 4px solid #3498db;
  width: 40px;
  height: 40px;
  animation: spin 2s linear infinite;
  margin: 20px auto;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}
</style>
