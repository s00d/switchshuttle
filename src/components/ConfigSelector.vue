<template>
  <div class="space-y-4">
    <div class="flex items-center justify-between">
      <h3 class="text-lg font-semibold text-slate-900">Configuration Selection</h3>
      <div class="flex items-center space-x-2">
        <Button @click="refreshConfigs" variant="ghost" size="sm">
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
          </svg>
        </Button>
        <Button @click="$emit('showAddConfigModal')" variant="secondary" size="sm">
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6" />
          </svg>
          Add
        </Button>
        <Button @click="$emit('showDeleteConfigModal')" variant="danger" size="sm">
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
          </svg>
          Delete
        </Button>
      </div>
    </div>
    
    <div v-if="configFiles.length === 0" class="text-center py-8">
      <div class="w-16 h-16 bg-slate-100 flex items-center justify-center mx-auto mb-4">
        <svg class="w-8 h-8 text-slate-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
        </svg>
      </div>
      <p class="text-slate-500 mb-2">No configurations found</p>
      <p class="text-sm text-slate-400">Create your first configuration to get started</p>
    </div>
    
    <div v-else class="space-y-2">
      <div
        v-for="file in configFiles"
        :key="file.path"
        class="flex items-center justify-between p-3 border border-slate-200 hover:border-slate-300 transition-colors"
        :class="[
          currentConfig === file.path 
            ? 'bg-blue-50 border-blue-300' 
            : 'bg-white'
        ]"
      >
        <div class="flex items-center space-x-3">
          <div class="w-8 h-8 bg-blue-100 flex items-center justify-center">
            <svg class="w-4 h-4 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
            </svg>
          </div>
          <div>
            <h4 class="font-medium text-slate-900">{{ file.name }}</h4>
            <p class="text-sm text-slate-500">{{ file.path }}</p>
          </div>
        </div>
        
        <div class="flex items-center space-x-2">
          <Button
            @click="selectConfig(file.path)"
            variant="ghost"
            size="sm"
            :class="[
              currentConfig === file.path 
                ? 'text-blue-600 bg-blue-100' 
                : ''
            ]"
          >
            {{ currentConfig === file.path ? 'Selected' : 'Select' }}
          </Button>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref, onMounted, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { ConfigFile, Config } from '../types';
import Button from './Button.vue';

const configFiles = ref<ConfigFile[]>([]);
const currentConfig = ref<string>('');

defineProps<{
  modelValue: Config;
}>();

const emit = defineEmits<{
  (e: 'showAddConfigModal'): void;
  (e: 'showDeleteConfigModal'): void;
  (e: 'update:modelValue', value: Config): void;
}>();

async function loadConfigs() {
  try {
    // Trying to load configurations via Tauri API
    const configs = await invoke('get_config_files') as ConfigFile[];
    configFiles.value = configs;
    
    if (configFiles.value.length > 0) {
      currentConfig.value = configFiles.value[0].path;
      loadConfig();
    }
  } catch (error) {
    console.error('Failed to load configs:', error);
    // If API is unavailable, create test data
    configFiles.value = [
      {
        path: '/test/config1.json',
        name: 'Test Configuration'
      },
      {
        path: '/test/config2.json',
        name: 'Second Configuration'
      }
    ];
    
    if (configFiles.value.length > 0) {
      currentConfig.value = configFiles.value[0].path;
      loadConfig();
    }
  }
}

async function loadConfig() {
  try {
    const config = await invoke('load_config', { path: currentConfig.value }) as Config;
    emit('update:modelValue', config);
  } catch (error) {
    console.error('Failed to load config:', error);
    // Create a test configuration
    const testConfig: Config = {
      terminal: 'iterm',
      launch_in: 'current',
      theme: 'Homebrew',
      title: 'Test Configuration',
      commands: [
        {
          name: 'Open Terminal',
          command: 'open -a iTerm',
          hotkey: undefined,
          submenu: null,
          commands: []
        }
      ],
      menu_hotkey: 'Cmd+Shift+S'
    };
    emit('update:modelValue', testConfig);
  }
}

const selectConfig = (path: string) => {
  currentConfig.value = path;
  loadConfig();
};

const refreshConfigs = () => {
  loadConfigs();
};

watch(currentConfig, () => {
  if (currentConfig.value) {
    loadConfig();
  }
});

onMounted(loadConfigs);
</script>
