<template>
  <div class="container">
    <h1 v-if="url">Update Available</h1>
    <h1 v-else>Update Not Available</h1>
    <div v-if="loading" class="loader"></div>
    <div v-if="loading" id="loader-text">loading...</div>
    <p v-if="!loading" id="message">{{ message }}</p>
    <button v-if="!loading && url" id="update-button" class="button-blue" @click="update">Update</button>
    <button v-if="!loading" id="close-button" @click="onClose">Close</button>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api';
import { open } from '@tauri-apps/api/shell';
import { appWindow } from '@tauri-apps/api/window';
import { useRouter } from 'vue-router';

const router = useRouter();
const message = ref('');
const loading = ref(true);
let url = '';

async function checkForUpdates() {
  try {
    const [msg, updateUrl] = await invoke('check_for_updates');
    message.value = msg;
    url = updateUrl;
  } catch (error) {
    message.value = error.message;
  } finally {
    loading.value = false;
  }
}

function update() {
  if (url) {
    open(url).then(() => {
      onClose();
    });
  }
}

function onClose() {
  router.push('/').catch((error) => {});
  appWindow.hide();
}

onMounted(checkForUpdates);
</script>

<style scoped>
/* Add styles from update.html */
h1 {
  margin: 20px 0;
  font-size: 24px;
  color: #333;
}

#message {
  margin: 20px 0;
  font-size: 16px;
  color: #333;
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

.button-blue {
  border: 1px solid #007aff;
  background: linear-gradient(to bottom, #007aff, #005bb5);
  color: white;
}

.button-blue:hover {
  background: linear-gradient(to bottom, #005bb5, #004a99);
  border-color: #005bb5;
}

.button-blue:active {
  background: linear-gradient(to bottom, #004a99, #003d7a);
  border-color: #004a99;
}
</style>
