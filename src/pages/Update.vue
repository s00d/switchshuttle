<template>
  <div class="flex flex-col items-center justify-center min-h-screen bg-white px-4 text-center">
    <h1 class="text-2xl font-semibold text-gray-800 mb-6">
      {{ url ? 'Update Available' : 'Update Not Available' }}
    </h1>

    <div v-if="loading" class="w-10 h-10 border-4 border-gray-200 border-t-blue-500 rounded-full animate-spin mb-4"></div>
    <div v-if="loading" class="text-sm text-gray-600 mb-4">loading...</div>

    <p v-if="!loading" class="text-base text-gray-700 mb-6">{{ message }}</p>

    <div v-if="!loading" class="flex gap-4 justify-center">
      <button
          v-if="url"
          @click="update"
          class="bg-blue-600 text-white px-5 py-2 text-sm hover:bg-blue-700 transition"
      >
        Update
      </button>
      <button
          @click="onClose"
          class="bg-gray-300 text-black px-5 py-2 text-sm hover:bg-gray-400 transition"
      >
        Close
      </button>
    </div>
  </div>
</template>


<script setup>
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-shell';
import { getCurrentWindow } from '@tauri-apps/api/window';
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
  getCurrentWindow().hide();
}

onMounted(checkForUpdates);
</script>