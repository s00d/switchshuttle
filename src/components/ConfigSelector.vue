<template>
  <div class="flex flex-col gap-2 w-full">
    <label for="config-select" class="text-sm font-medium text-gray-700">Select Config</label>
    <div class="flex gap-2 w-full">
      <select
          id="config-select"
          class="flex-1 border border-gray-300 px-3 py-2 text-sm text-gray-800 bg-white focus:outline-none focus:ring-2 focus:ring-blue-500"
          v-model="currentConfig"
          @change="loadConfig"
      >
        <option
            v-for="file in configFiles"
            :key="file.path"
            :value="file.path"
            v-text="file.name"
        ></option>
      </select>
      <button
          class="bg-blue-600 text-white text-sm px-3 py-2 hover:bg-blue-700 transition"
          @click="$emit('showAddConfigModal')"
      >
        +
      </button>
      <button
          class="bg-red-500 text-white text-sm px-3 py-2 hover:bg-red-600 transition"
          @click="$emit('showDeleteConfigModal')"
      >
        x
      </button>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref, onMounted, watch } from 'vue';
import { exists, readDir, readTextFile } from '@tauri-apps/plugin-fs';
import { path as tauriPath } from '@tauri-apps/api';
import { ConfigFile, Config } from '../types';

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
  const configDir = await tauriPath.configDir();
  const dirPath = `${configDir}/switch-shuttle`;
  const configFilesList = await readDir(dirPath);
  configFiles.value = configFilesList
      .filter(file => file.name?.endsWith('.json'))
      .map(file => ({
        path: `${dirPath}/${file.name}`, // полный путь к файлу
        name: file.name?.replace('.json', '') ?? ''
      }));

  if (configFiles.value.length > 0) {
    currentConfig.value = configFiles.value[0].path;
    loadConfig();
  }
}

async function loadConfig() {
  if (await exists(currentConfig.value)) {
    const configContent = await readTextFile(currentConfig.value);
    const config = JSON.parse(configContent) as Config;
    emit('update:modelValue', config);
  }
}

watch(currentConfig, () => {
  if (currentConfig.value) {
    loadConfig();
  }
});

onMounted(loadConfigs);
</script>
