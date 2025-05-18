<template>
  <div class="flex min-h-screen w-screen items-center justify-center bg-white">
    <div class="w-full max-w-md bg-white p-6 rounded text-center">
      <h1 class="text-xl font-semibold text-gray-800 mb-6">Create New Config</h1>
      <input
          type="text"
          id="fileName"
          name="fileName"
          placeholder="Enter file name (without .json)"
          v-model="fileName"
          required
          class="w-full px-4 py-2 border border-gray-300 text-sm rounded-none focus:outline-none focus:ring-2 focus:ring-blue-500"
      />
      <div v-if="errorMessage" class="mt-2 text-red-600 text-sm">{{ errorMessage }}</div>
      <div class="mt-6 flex justify-between gap-4">
        <button
            type="submit"
            @click="onCreate"
            class="w-full bg-blue-600 text-white text-sm py-2 hover:bg-blue-700 transition"
        >
          Create
        </button>
        <button
            type="button"
            @click="onClose"
            class="w-full bg-red-500 text-white text-sm py-2 hover:bg-red-600 transition"
        >
          Cancel
        </button>
      </div>
    </div>
  </div>
</template>


<script setup>
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { useRouter } from 'vue-router';

const router = useRouter();
const fileName = ref('');
const errorMessage = ref('');

function onCreate() {
  invoke('create_new_config', { fileName: fileName.value }).then(() => {
    onClose();
  }).catch((error) => {
    errorMessage.value = error;
  });
}

function onClose() {
  router.push('/').catch((error) => {});
  getCurrentWindow().hide();
}
</script>
