<template>
  <div class="flex flex-col items-center justify-center min-h-screen bg-slate-50 px-6 text-center">
    <div class="bg-white rounded-2xl shadow-xl border border-slate-200 p-8 max-w-md w-full">
      <h1 class="text-3xl font-bold text-slate-800 mb-6">
      {{ url ? 'Update Available' : 'Update Not Available' }}
    </h1>

      <div v-if="loading" class="w-16 h-16 border-4 border-slate-200 border-t-blue-500 rounded-full animate-spin mb-6 mx-auto shadow-lg"></div>
      <div v-if="loading" class="text-lg text-slate-600 mb-6 font-medium">Checking for updates...</div>

      <p v-if="!loading" class="text-base text-slate-700 mb-8 leading-relaxed">{{ message }}</p>

    <div v-if="!loading" class="flex gap-4 justify-center">
        <Button
          v-if="url"
          variant="primary"
          @click="update"
        >
          Download Update
        </Button>
        <Button
          variant="secondary"
          @click="onClose"
      >
        Close
        </Button>
      </div>
    </div>
  </div>
</template>


<script setup>
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-shell';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { useRouter } from 'vue-router';
import Button from '../components/Button.vue';

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