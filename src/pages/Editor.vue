<template>
  <div class="container">
    <h1>Config Editor</h1>

    <div id="notification" :class="notificationClass" class="alert" v-if="notificationMessage" v-text="notificationMessage"></div>

    <ConfigSelector v-model="config" @showAddConfigModal="showAddConfigModal" @showDeleteConfigModal="showDeleteConfigModal"/>
    <ConfigEditor :config="config" :commands="config.commands" @saveConfig="saveConfig" @onClose="onClose" />

    <Modal :show="addConfigModal" title="Add New Config" @close="closeAddConfigModal">
      <div class="form-group">
        <label for="new-config-name">Config Name</label>
        <input type="text" id="new-config-name" class="form-control" v-model="newConfigName">
      </div>
      <template #footer>
        <button type="button" class="button-blue" @click="addNewConfig">Add</button>
        <button type="button" class="cancel-button" @click="closeAddConfigModal">Cancel</button>
      </template>
    </Modal>

    <Modal :show="deleteConfigModal" title="Confirm Delete" @close="closeDeleteConfigModal">
      <p>Are you sure you want to delete this config?</p>
      <template #footer>
        <button type="button" class="cancel-button" @click="deleteConfig">Delete</button>
        <button type="button" class="button-blue" @click="closeDeleteConfigModal">Cancel</button>
      </template>
    </Modal>
  </div>
</template>

<script lang="ts" setup>
import { ref, onMounted } from 'vue';
import { writeTextFile, readTextFile, exists, readDir, removeFile } from '@tauri-apps/api/fs';
import { appWindow } from '@tauri-apps/api/window';
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
    path: file.path,
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
    await removeFile(currentConfig.value);
    showNotification('Config deleted successfully', 'success');
    closeDeleteConfigModal();
    loadConfigs();
  } catch (error) {
    showNotification('Failed to delete config', 'error');
  }
}

function onClose() {
  router.push('/').catch(() => {});
  appWindow.hide();
}

onMounted(loadConfigs);
</script>

<style>
/* Add styles from editor.html */
.container {
  padding: 40px;
  display: flex;
  flex-direction: column;
  justify-content: flex-start;
  overflow-x: scroll;
  max-width: 100%;
  height: 100%;
}

h1 {
  margin-top: 40px;
  font-size: 24px;
  color: #333;
}

#notification {
  width: 80%;
  margin: 20px auto;
  padding: 10px;
  border-radius: 6px;
}


.alert-success {
  background-color: #d4edda;
  color: #155724;
}

.alert-error {
  background-color: #f8d7da;
  color: #721c24;
}
</style>
