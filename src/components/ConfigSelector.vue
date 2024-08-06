<template>
  <div class="form-group">
    <label for="config-select">Select Config</label>
    <div class="d-flex gap-2">
      <select id="config-select" class="form-control" v-model="currentConfig" @change="loadConfig">
        <option v-for="file in configFiles" :value="file.path" :key="file.path" v-text="file.name"></option>
      </select>
      <button class="button-blue" @click="$emit('showAddConfigModal')">+</button>
      <button class="cancel-button" @click="$emit('showDeleteConfigModal')">x</button>
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
  const configFilesList = await readDir(`${configDir}/switch-shuttle`);
  configFiles.value = configFilesList.filter(file => file.name?.endsWith('.json')).map(file => ({
    path: `${configDir}/switch-shuttle`,
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

<style scoped>
.form-group {
  display: flex;
  flex-direction: column;
  align-items: center;
  margin-bottom: 20px;
  width: 100%;
}

.form-group label {
  font-size: 16px;
  color: #333;
  margin-bottom: 5px;
  width: 100%;
  text-align: left;
}

.d-flex {
  display: flex;
  width: 100%;
  justify-content: center;
}

.gap-2 {
  gap: 8px;
}

.form-control {
  width: calc(100% - 20px);
  margin: 0 10px 0 0;
  height: 32px; /* Ensure the height is set to 20px */
  padding: 0; /* Remove default padding */
  border: 1px solid #ccc; /* Optional: Customize border */
  box-sizing: border-box; /* Ensure padding and border are included in the height */
  font-size: 14px; /* Adjust the font size as needed */
}


</style>
