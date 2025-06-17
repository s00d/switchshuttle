<template>
  <header class="fixed top-0 left-0 right-0 z-50 bg-white border-b border-slate-200" data-tauri-drag-region>
    <div class="container mx-auto px-4">
      <div class="flex items-center justify-between h-16">
        <!-- Logo and Title -->
        <div class="flex items-center space-x-3" data-tauri-drag-region>
          <img src="/logo.svg" alt="SwitchShuttle" class="w-8 h-8" />
          <h1 class="text-xl font-bold text-slate-900">SwitchShuttle</h1>
        </div>
        
        <!-- Actions -->
        <div class="flex items-center space-x-2">
          <!-- Close Button -->
          <Button variant="danger" size="sm" @click="closeApp">
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
            </svg>
          </Button>
        </div>
      </div>
    </div>
  </header>
</template>

<script setup lang="ts">
import { getCurrentWindow } from '@tauri-apps/api/window';
import Button from './Button.vue';

const closeApp = async () => {
  try {
    const window = getCurrentWindow();
    await window.hide();
  } catch (error) {
    console.error('Failed to close app:', error);
  }
};
</script> 