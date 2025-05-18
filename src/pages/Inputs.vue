<template>
  <div class="w-full h-full overflow-x-auto flex flex-col items-center px-6 py-8 bg-white">
    <form id="inputForm" class="w-full max-w-lg flex flex-col gap-4">
      <div v-for="(value, key) in inputs" :key="key" class="flex flex-col gap-1">
        <label :for="key" class="text-sm font-medium text-gray-700 bg-gray-100 px-2 py-1 rounded">
          {{ key }}
        </label>
        <input
            type="text"
            :name="key"
            :id="key"
            v-model="inputs[key]"
            class="w-full border border-gray-300 px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
      </div>
    </form>

    <div v-if="errorMessage" class="text-red-600 text-sm mt-4">{{ errorMessage }}</div>

    <div class="mt-6 flex justify-center gap-4">
      <button
          type="button"
          class="bg-blue-600 text-white px-6 py-2 text-sm hover:bg-blue-700 transition"
          @click="submitForm"
      >
        OK
      </button>
      <button
          type="button"
          class="bg-gray-300 text-black px-6 py-2 text-sm hover:bg-gray-400 transition"
          @click="onClose"
      >
        Cancel
      </button>
    </div>
  </div>
</template>


<script setup>
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { useRouter, useRoute } from 'vue-router';

const router = useRouter();
const route = useRoute();

const command = ref('');
const inputs = ref({});
const errorMessage = ref('');

function submitForm() {
  invoke('execute_command_with_inputs', { inputs: inputs.value, command: command.value }).then(() => {
    onClose();
  }).catch((error) => {
    errorMessage.value = error;
  });
}

function fetchInputData() {
  // Implement your logic to fetch input data using the ID
  invoke('fetch_input_data', { command: command.value }).then((data) => {
    console.log(111, data);
    inputs.value = JSON.parse(data);
  }).catch((error) => {
    errorMessage.value = error;
  });
}

function onClose() {
  router.push('/').catch((error) => {});
  getCurrentWindow().hide();
}

onMounted(() => {
  command.value = route.params.id; // Get the ID from the route parameters
  fetchInputData(); // Fetch the input data using the ID
});
</script>