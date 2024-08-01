<template>
  <div class="container">
    <p id="message">{{ message }}</p>
    <button @click="onClose">OK</button>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api';
import { appWindow } from '@tauri-apps/api/window';
import { useRouter } from 'vue-router';

const router = useRouter();
const message = ref('');

async function fetchMessage() {
  message.value = await invoke('about_message');
}

function onClose() {
  router.push('/').catch((error) => {});
  appWindow.hide();
}

onMounted(fetchMessage);
</script>

<style scoped>
/* Add styles from about.html */
#app {
  height: 100vh;
  background-color: #f3f3f3;
  border-radius: 5px;
}

#message {
  margin-bottom: 20px;
  font-size: 16px;
  color: #333;
}

.container {
  height: 100%;
}
</style>
