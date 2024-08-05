<template>
  <div class="container">
    <img src="/logo.svg" alt="Logo" class="logo">
    <div id="message" v-html="message" @click="handleLinkClick"></div>
    <button class="button-blue" @click="onClose">OK</button>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { useRouter } from 'vue-router';
import { open } from '@tauri-apps/plugin-shell';

const router = useRouter();
const message = ref('');

async function fetchMessage() {
  message.value = await invoke('about_message');
}

function onClose() {
  router.push('/').catch((error) => {});
  getCurrentWindow().hide();
}

async function handleLinkClick(event) {
  if (event.target.tagName === 'A') {
    event.preventDefault();
    await open(event.target.href);
  }
}

onMounted(fetchMessage);
</script>

<style scoped>
#app {
  height: 100vh;
  background-color: #f3f3f3;
  display: flex;
  align-items: center;
  justify-content: center;
}

.container {
  background: white;
  padding: 20px;
  border-radius: 10px;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
  text-align: center;
  position: relative;
}

.logo {
  width: 128px;
  height: 128px;
  display: block;
  margin: 20px auto 0;
}

#message {
  margin-bottom: 20px;
  font-size: 16px;
  color: #333;
}

button {
  padding: 10px 20px;
  font-size: 14px;
  border: none;
  border-radius: 5px;
  background-color: #007bff;
  color: white;
  cursor: pointer;
}

button:hover {
  background-color: #0056b3;
}
</style>
