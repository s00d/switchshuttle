<template>
  <div class="min-h-screen w-screen flex items-center justify-center bg-white">
    <div class="bg-white p-6 rounded text-center w-full max-w-md">
      <img src="/logo.svg" alt="Logo" class="w-32 h-32 mx-auto mb-4" />
      <div id="message" class="text-gray-800 text-base mb-6" v-html="message" @click="handleLinkClick"></div>
      <button @click="onClose" class="px-6 py-2 text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 transition-colors">
        OK
      </button>
    </div>
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

<style>
#message {
  padding: 0.25rem;
  border: 1px solid #d1d5db;       /* border-gray-300 */
  background-color: #f9fafb;       /* bg-gray-50 */
  color: #1f2937;                  /* text-gray-800 */
  font-family: system-ui, sans-serif;
  font-size: 0.95rem;
  line-height: 1.6;
  text-align: center;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.05);
}

/* Заголовок */
#message h2 {
  font-size: 1.5rem;               /* text-2xl */
  font-weight: 600;
  margin-bottom: 0.75rem;
  color: #111827;                 /* text-gray-900 */
  border-bottom: 1px solid #e5e7eb; /* border-gray-200 */
  padding-bottom: 0.5rem;
}

/* Абзацы */
#message p {
  margin-bottom: 0.75rem;
  font-size: 0.95rem;
  color: #374151;                 /* text-gray-700 */
}

/* Последний абзац без нижнего отступа */
#message p:last-child {
  margin-bottom: 0;
}

/* Ссылки */
#message a {
  color: #2563eb;                 /* text-blue-600 */
  text-decoration: underline;
  font-weight: 500;
  transition: color 0.2s ease;
}

#message a:hover {
  color: #1d4ed8;                 /* text-blue-700 */
  text-decoration: underline;
}

/* Тонкие элементы (©, версия и т.д.) */
#message p small {
  font-size: 0.8rem;
  color: #6b7280;                 /* text-gray-500 */
}

</style>
