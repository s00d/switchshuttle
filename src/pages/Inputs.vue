<template>
  <div class="w-full h-full overflow-x-auto flex flex-col items-center px-8 py-8 bg-slate-50">
    <div class="w-full max-w-2xl">
      <div class="bg-white rounded-xl shadow-sm border border-slate-200 p-8">
        <h1 class="text-2xl font-bold text-slate-800 mb-6">Input Parameters</h1>
        
        <form id="inputForm" class="w-full flex flex-col gap-6">
          <div v-for="(value, key) in inputs" :key="key" class="flex flex-col gap-2">
            <label :for="key" class="text-sm font-semibold text-slate-700 bg-slate-50 px-3 py-2 rounded-lg border border-slate-200">
          {{ key }}
        </label>
            <Input
              v-model="inputs[key]"
              :id="key"
              :name="key"
              :placeholder="`Enter ${key.toLowerCase()}`"
        />
      </div>
    </form>

        <div v-if="errorMessage" class="text-red-600 text-sm mt-6 bg-red-50 border border-red-200 rounded-lg px-4 py-3">{{ errorMessage }}</div>

        <div class="mt-8 flex justify-center gap-4">
          <Button
            variant="primary"
          @click="submitForm"
      >
            Execute Command
          </Button>
          <Button
            variant="secondary"
          @click="onClose"
      >
        Cancel
          </Button>
        </div>
      </div>
    </div>
  </div>
</template>


<script setup>
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { useRouter, useRoute } from 'vue-router';
import Button from '../components/Button.vue';
import Input from '../components/Input.vue';

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