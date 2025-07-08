<template>
  <div class="min-h-screen bg-gradient-to-br from-slate-50 to-slate-100 flex items-start justify-center pt-3 p-3">
    <div class="w-full max-w-sm">
      <!-- Form Card -->
      <div class="bg-white rounded-xl shadow-sm border border-slate-200/60 backdrop-blur-sm">
        <form id="inputForm" class="p-4 space-y-3" @submit.prevent="submitForm">
          <div v-for="(_, key) in inputs" :key="key" class="flex items-center">
            <label :for="key" class="text-sm font-medium text-slate-700 bg-slate-50 border border-r-0 border-slate-200 px-3 py-2 rounded-l-lg">
              {{ key }}
            </label>
            <div class="relative flex-1">
              <input
                v-model="inputs[key]"
                :id="key"
                :name="key"
                :placeholder="`Enter ${String(key).toLowerCase()}`"
                class="w-full px-3 py-2 bg-white border-l-0 border border-slate-200 rounded-r-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all duration-200 hover:border-slate-300 focus:outline-none"
                @keydown.enter="submitForm"
              />
              <div class="absolute inset-y-0 right-0 flex items-center pr-2">
                <div class="w-1.5 h-1.5 bg-blue-500 rounded-full"></div>
              </div>
            </div>
          </div>
        </form>

        <!-- Error Message -->
        <div v-if="errorMessage" class="mx-4 mb-3 p-3 bg-red-50 border border-red-200 rounded-lg">
          <div class="flex items-center gap-2">
            <svg class="w-3.5 h-3.5 text-red-500 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
            </svg>
            <span class="text-xs text-red-700">{{ errorMessage }}</span>
          </div>
        </div>

        <!-- Action Buttons -->
        <div class="p-4 pt-0 flex gap-2">
          <Button
            variant="secondary"
            @click="onClose"
            class="flex-1 px-3 py-2 text-sm font-medium text-slate-700 bg-slate-100 hover:bg-slate-200 rounded-lg transition-colors duration-200"
          >
            Cancel
          </Button>
          <Button
            variant="primary"
            @click="submitForm"
            class="flex-1 px-3 py-2 text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 rounded-lg transition-colors duration-200 shadow-sm hover:shadow-md"
          >
            <svg class="w-3.5 h-3.5 mr-1.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"></path>
            </svg>
            Execute
          </Button>
        </div>
      </div>

      <!-- Footer -->
      <div class="text-center mt-3">
        <p class="text-xs text-slate-400">
          <kbd class="px-1.5 py-0.5 text-xs font-semibold text-slate-500 bg-slate-100 border border-slate-200 rounded">Enter</kbd> to submit • 
          <kbd class="px-1.5 py-0.5 text-xs font-semibold text-slate-500 bg-slate-100 border border-slate-200 rounded">Esc</kbd> to cancel
        </p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch, nextTick, inject } from 'vue';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { useRouter, useRoute } from 'vue-router';
import Button from '../components/Button.vue';
import type { TauriInjectionKey } from '../lib/tauri-commands-plugin';

const router = useRouter();
const route = useRoute();

// Получаем доступ к командам через плагин
const tauri = inject('tauri') as TauriInjectionKey['tauri'];

const command = ref('');
const inputs = ref({});
const errorMessage = ref('');

async function submitForm() {
  try {
    await tauri.execute_command_with_inputs(inputs.value, command.value);
    onClose();
  } catch (error) {
    errorMessage.value = error as string;
  }
}

function handleKeydown(event: KeyboardEvent) {
  // Submit form on Enter key (but not when typing in input fields)
  if (event.key === 'Enter' && (event.target as HTMLElement).tagName !== 'INPUT') {
    event.preventDefault();
    submitForm();
  }
  // Close on Escape key
  if (event.key === 'Escape') {
    event.preventDefault();
    onClose();
  }
}

async function fetchInputData() {
  try {
    const data = await tauri.fetch_input_data(command.value);
    inputs.value = data;
  } catch (error) {
    errorMessage.value = error as string;
  }
}

function onClose() {
  // Reset form state
  inputs.value = {};
  errorMessage.value = '';
  command.value = '';
  
  // Navigate to home and hide window
  router.push('/').then(() => {
    getCurrentWindow().hide();
  }).catch(() => {
    // If navigation fails, just hide the window
    getCurrentWindow().hide();
  });
}

onMounted(() => {
  // Check if route params are valid
  if (!route.params.id) {
    console.warn('No command ID provided, redirecting to home');
    router.push('/').catch(() => {});
    return;
  }
  
  command.value = route.params.id as string; // Get the ID from the route parameters
  fetchInputData(); // Fetch the input data using the ID
  
  // Add global keyboard listener
  document.addEventListener('keydown', handleKeydown);
});

// Watch for inputs to be loaded and focus first field
watch(inputs, (newInputs) => {
  if (newInputs && Object.keys(newInputs).length > 0) {
    // Focus on first input field after a short delay to ensure DOM is ready
    nextTick(() => {
      const firstInput = document.querySelector('input');
      if (firstInput) {
        firstInput.focus();
      }
    });
  }
}, { immediate: true });

onUnmounted(() => {
  // Clean up keyboard listener
  document.removeEventListener('keydown', handleKeydown);
});
</script>