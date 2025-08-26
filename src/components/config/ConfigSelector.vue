<template>
  <div class="space-y-4">
    <div class="flex items-center justify-between">
      <h3 class="text-lg font-semibold text-slate-900">
        Configuration Selection
      </h3>
      <div class="flex items-center space-x-2">
        <CustomButton variant="ghost" size="sm" @click="refreshConfigs">
          <SpinnerIcon class="w-4 h-4" />
        </CustomButton>
        <CustomButton
          variant="secondary"
          size="sm"
          @click="$emit('showAddConfigModal')"
        >
          <AddIcon class="w-4 h-4" />
          Add
        </CustomButton>
        <CustomButton
          variant="danger"
          size="sm"
          @click="$emit('showDeleteConfigModal')"
        >
          <TrashIcon class="w-4 h-4" />
          Delete
        </CustomButton>
      </div>
    </div>

    <div v-if="configFiles.length === 0" class="text-center py-8">
      <div
        class="w-16 h-16 bg-slate-100 flex items-center justify-center mx-auto mb-4"
      >
        <DocumentIcon class="w-8 h-8 text-slate-400" />
      </div>
      <p class="text-slate-500 mb-2">No configurations found</p>
      <p class="text-sm text-slate-400">
        Create your first configuration to get started
      </p>
    </div>

    <div v-else class="space-y-2">
      <div
        v-for="file in configFiles"
        :key="file.path"
        class="flex items-center justify-between p-3 border border-slate-200 hover:border-slate-300 transition-colors"
        :class="[
          currentConfig === file.path
            ? 'bg-blue-50 border-blue-300'
            : 'bg-white',
        ]"
      >
        <div class="flex items-center space-x-3">
          <div class="w-8 h-8 bg-blue-100 flex items-center justify-center">
            <DocumentIcon class="w-4 h-4 text-blue-600" />
          </div>
          <div>
            <h4 class="font-medium text-slate-900">{{ file.name }}</h4>
            <p class="text-sm text-slate-500">{{ file.path }}</p>
          </div>
        </div>

        <div class="flex items-center space-x-2">
          <CustomButton
            variant="ghost"
            size="sm"
            :class="[currentConfig === file.path ? 'selected' : '']"
            @click="selectConfig(file.path)"
          >
            {{ currentConfig === file.path ? 'Selected' : 'Select' }}
          </CustomButton>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref, onMounted, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { ConfigFile, Config } from '../../types';
import CustomButton from '../ui/CustomButton.vue';
import SpinnerIcon from '../icons/SpinnerIcon.vue';
import AddIcon from '../icons/AddIcon.vue';
import TrashIcon from '../icons/TrashIcon.vue';
import DocumentIcon from '../icons/DocumentIcon.vue';

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
    const configs = (await invoke('get_config_files')) as ConfigFile[];
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
        name: 'Test Configuration',
      },
      {
        path: '/test/config2.json',
        name: 'Second Configuration',
      },
    ];

    if (configFiles.value.length > 0) {
      currentConfig.value = configFiles.value[0].path;
      loadConfig();
    }
  }
}

async function loadConfig() {
  try {
    const config = (await invoke('load_config', {
      path: currentConfig.value,
    })) as Config;
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
          commands: [],
        },
      ],
      menu_hotkey: 'Cmd+Shift+S',
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
