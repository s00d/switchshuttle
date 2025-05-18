<template>
  <div class="flex flex-col px-10 py-6 overflow-x-auto h-full w-full bg-white">
    <h1 class="text-2xl font-semibold text-gray-800 mb-6">Config Editor</h1>

    <div v-if="notificationMessage" :class="notificationClass" class="w-4/5 mx-auto mb-4 rounded px-4 py-3 text-sm font-medium text-center">
      {{ notificationMessage }}
    </div>

    <ConfigSelector
        v-model="config"
        @showAddConfigModal="showAddConfigModal"
        @showDeleteConfigModal="showDeleteConfigModal"
    />

    <ConfigEditor
        :config="config"
        :commands="config.commands"
        @saveConfig="saveConfig"
        @onClose="onClose"
    />

    <!-- Add Config Modal -->
    <Modal :show="addConfigModal" title="Add New Config" @close="closeAddConfigModal">
      <div class="flex flex-col gap-2 mb-4">
        <label for="new-config-name" class="text-sm font-medium text-gray-700">Config Name</label>
        <input
            type="text"
            id="new-config-name"
            v-model="newConfigName"
            class="border border-gray-300 px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
      </div>
      <template #footer>
        <button class="bg-blue-600 text-white px-4 py-2 text-sm hover:bg-blue-700" @click="addNewConfig">Add</button>
        <button class="bg-gray-300 text-black px-4 py-2 text-sm hover:bg-gray-400" @click="closeAddConfigModal">Cancel</button>
      </template>
    </Modal>

    <!-- Delete Config Modal -->
    <Modal :show="deleteConfigModal" title="Confirm Delete" @close="closeDeleteConfigModal">
      <p class="text-sm text-gray-800 mb-4">Are you sure you want to delete this config?</p>
      <template #footer>
        <button class="bg-red-600 text-white px-4 py-2 text-sm hover:bg-red-700" @click="deleteConfig">Delete</button>
        <button class="bg-gray-300 text-black px-4 py-2 text-sm hover:bg-gray-400" @click="closeDeleteConfigModal">Cancel</button>
      </template>
    </Modal>
  </div>
</template>

<script lang="ts" setup>
import { ref, onMounted } from 'vue';
import { writeTextFile, readTextFile, exists, readDir, remove } from '@tauri-apps/plugin-fs';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { path as tauriPath } from '@tauri-apps/api';
import { useRouter } from 'vue-router';

import ConfigSelector from '../components/ConfigSelector.vue';
import ConfigEditor from '../components/ConfigEditor.vue';
import Modal from '../components/Model.vue';

import { ConfigFile, Config } from '../types';

const router = useRouter();
const configFiles = ref<ConfigFile[]>([]);
const currentConfig = ref<string>('');
const config = ref<Config>({
  terminal: 'iterm',
  launch_in: 'current',
  theme: '',
  title: '',
  commands: [],
  menu_hotkey: ''
});
const newConfigName = ref<string>('');
const notificationMessage = ref<string>('');
const notificationClass = ref<string>('');
const addConfigModal = ref<boolean>(false);
const deleteConfigModal = ref<boolean>(false);

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
    config.value = JSON.parse(configContent);
  }
}

async function saveConfig() {
  try {
    await writeTextFile(currentConfig.value, JSON.stringify(config.value, null, 2));
    showNotification('Config saved successfully', 'success');
  } catch (error) {
    showNotification('Failed to save config', 'error');
  }
}

function showNotification(message: string, type: string) {
  notificationMessage.value = message;
  notificationClass.value = `alert-${type}`;
  setTimeout(() => {
    notificationMessage.value = '';
    notificationClass.value = '';
  }, 3000);
}

function showAddConfigModal() {
  addConfigModal.value = true;
}

function closeAddConfigModal() {
  addConfigModal.value = false;
}

function showDeleteConfigModal() {
  deleteConfigModal.value = true;
}

function closeDeleteConfigModal() {
  deleteConfigModal.value = false;
}

async function addNewConfig() {
  if (!newConfigName.value) {
    showNotification('Config name cannot be empty', 'error');
    return;
  }
  const configDir = await tauriPath.configDir();
  const newConfigPath = `${configDir}/switch-shuttle/${newConfigName.value}.json`;
  const defaultConfig: Config = {
    terminal: "iterm",
    launch_in: "current",
    theme: "Homebrew",
    title: "New tab",
    commands: [],
    menu_hotkey: ''
  };
  try {
    await writeTextFile(newConfigPath, JSON.stringify(defaultConfig, null, 2));
    showNotification('New config added successfully', 'success');
    closeAddConfigModal();
    loadConfigs();
  } catch (error) {
    showNotification('Failed to add new config', 'error');
  }
}

async function deleteConfig() {
  if (!currentConfig.value) {
    showNotification('No config selected to delete', 'error');
    return;
  }
  try {
    await remove(currentConfig.value);
    showNotification('Config deleted successfully', 'success');
    closeDeleteConfigModal();
    loadConfigs();
  } catch (error) {
    showNotification('Failed to delete config', 'error');
  }
}

function onClose() {
  router.push('/').catch(() => {});
  getCurrentWindow().hide()
}

onMounted(loadConfigs);
</script>

