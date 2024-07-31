<template>
  <div class="container">
    <h1>Config Editor</h1>

    <div id="notification" :class="notificationClass" class="alert" v-if="notificationMessage" v-text="notificationMessage"></div>

    <div class="form-group">
      <label for="config-select">Select Config</label>
      <div class="d-flex gap-2">
        <select id="config-select" class="form-control" v-model="currentConfig" @change="loadConfig">
          <option v-for="file in configFiles" :value="file.path" :key="file.path" v-text="file.name"></option>
        </select>
        <button class="button-blue" @click="showAddConfigModal">+</button>
        <button class="cancel-button" @click="showDeleteConfigModal">x</button>
      </div>
    </div>

    <div class="form-group">
      <label for="terminal">Terminal</label>
      <select id="terminal" class="form-control" v-model="config.terminal">
        <option value="iterm">iTerm</option>
        <option value="terminal">Terminal</option>
        <option value="warp">Warp</option>
        <option value="hyper">Hyper</option>
        <option value="alacritty">Alacritty</option>
      </select>
    </div>

    <div class="form-group">
      <label for="launch-in">Launch In</label>
      <select id="launch-in" class="form-control" v-model="config.launch_in">
        <option value="current">Current</option>
        <option value="new_tab">New Tab</option>
        <option value="new_window">New Window</option>
      </select>
    </div>

    <div class="form-group">
      <label for="theme">Theme</label>
      <input type="text" id="theme" class="form-control" v-model="config.theme">
    </div>

    <div class="form-group">
      <label for="title">Title</label>
      <input type="text" id="title" class="form-control" v-model="config.title">
    </div>

    <div class="form-group">
      <label for="menu-hotkey">Menu Hotkey</label>
      <input type="text" id="menu-hotkey" class="form-control" v-model="config.menu_hotkey" @keydown="hotkeyHandlerMenu">
    </div>

    <table id="commands-table">
      <thead>
      <tr>
        <th>Command Name</th>
        <th>Single Command</th>
        <th>Commands</th>
        <th>Hotkey</th>
        <th>Actions</th>
      </tr>
      </thead>
      <tbody>
      <template v-for="(command, index) in config.commands"  :key="index">
        <tr :data-index="index">
          <td><input type="text" class="form-control" v-model="command.name"></td>
          <td><input v-if="!command.submenu" type="text" class="form-control" v-model="command.command" :disabled="command.commands && command.commands.length"></td>
          <td>
            <div v-if="!command.submenu" v-for="(cmd, cmdIndex) in command.commands" :key="cmdIndex" class="d-flex align-items-center">
              <input type="text" class="form-control" v-model="command.commands[cmdIndex]">
              <button class="cancel-button" @click="deleteInnerCommand(index, cmdIndex)">Delete</button>
            </div>
            <button v-if="!command.submenu" class="button-blue" @click="addInnerCommand(index)">+</button>
          </td>
          <td>
            <div class="d-flex align-items-center">
              <input type="checkbox" class="mr-2" :checked="command.hotkey !== null" @change="toggleHotkeyInput(index, command.hotkey)">
              <input type="text" class="form-control" v-model="command.hotkey" v-if="command.hotkey !== null" @keydown="hotkeyHandler($event, index)" :disabled="command.submenu">
            </div>
          </td>
          <td>
            <button class="button-blue" @click="addSubCommand(index)">+</button>
            <button class="cancel-button" @click="deleteCommand(index)">x</button>
          </td>
        </tr>
        <tr v-if="command.submenu" :key="'sub' + index">
          <td colspan="5">
            <SubcommandTable :subcommands="command.submenu" :commandIndex="index" @addInnerSubcommand="addInnerSubCommand" @deleteInnerSubcommand="deleteInnerSubCommand" @toggleHotkeyInput="toggleHotkeyInput" @hotkeyHandler="hotkeyHandler" @deleteSubcommand="deleteSubCommand" @addSubcommand="addSubCommand"></SubcommandTable>
          </td>
        </tr>
      </template>

      </tbody>
    </table>
    <div class="table-actions text-center">
      <button class="button-blue" @click="addCommand">Add Command</button>
    </div>

    <div class="button-group d-flex justify-content-between mt-4">
      <button class="button-blue" @click="saveConfig">Save</button>
      <button class="cancel-button" @click="onClose">Cancel</button>
    </div>

    <div id="add-config-modal" v-if="addConfigModal" class="modal">
      <div class="modal-dialog">
        <div class="modal-content">
          <div class="modal-header">
            <h5 class="modal-title">Add New Config</h5>
            <button type="button" class="close" @click="closeAddConfigModal">
              <span aria-hidden="true">&times;</span>
            </button>
          </div>
          <div class="modal-body">
            <div class="form-group">
              <label for="new-config-name">Config Name</label>
              <input type="text" id="new-config-name" class="form-control" v-model="newConfigName">
            </div>
          </div>
          <div class="modal-footer">
            <button type="button" class="button-blue" @click="addNewConfig">Add</button>
            <button type="button" class="cancel-button" @click="closeAddConfigModal">Cancel</button>
          </div>
        </div>
      </div>
    </div>

    <div id="delete-config-modal" v-if="deleteConfigModal" class="modal">
      <div class="modal-dialog">
        <div class="modal-content">
          <div class="modal-header">
            <h5 class="modal-title">Confirm Delete</h5>
            <button type="button" class="close" @click="closeDeleteConfigModal">
              <span aria-hidden="true">&times;</span>
            </button>
          </div>
          <div class="modal-body">
            <p>Are you sure you want to delete this config?</p>
          </div>
          <div class="modal-footer">
            <button type="button" class="cancel-button" @click="deleteConfig">Delete</button>
            <button type="button" class="button-blue" @click="closeDeleteConfigModal">Cancel</button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import { writeTextFile, readTextFile, exists, readDir, removeFile } from '@tauri-apps/api/fs';
import { appWindow } from '@tauri-apps/api/window';
import { path as tauriPath } from '@tauri-apps/api';
import SubcommandTable from '../components/SubcommandTable.vue';

const configFiles = ref([]);
const currentConfig = ref('');
const config = ref({
  terminal: 'iterm',
  launch_in: 'current',
  theme: '',
  title: '',
  commands: [],
  menu_hotkey: ''
});
const newConfigName = ref('');
const notificationMessage = ref('');
const notificationClass = ref('');
const addConfigModal = ref(false);
const deleteConfigModal = ref(false);

async function loadConfigs() {
  const configDir = await tauriPath.configDir();
  const configFilesList = await readDir(`${configDir}/switch-shuttle`);
  configFiles.value = configFilesList.filter(file => file.name.endsWith('.json')).map(file => ({
    path: file.path,
    name: file.name.replace('.json', '')
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
    cancel();
  } catch (error) {
    showNotification('Failed to save config', 'error');
  }
}

function addCommand() {
  config.value.commands.push({ name: '', command: '', hotkey: null, hotkeyEnabled: false, submenu: [], commands: [] });
}

function deleteCommand(index) {
  config.value.commands.splice(index, 1);
}

function addInnerCommand(index) {
  config.value.commands[index].commands = config.value.commands[index].commands || [];
  config.value.commands[index].commands.push('');
}

function deleteInnerCommand(index, cmdIndex) {
  config.value.commands[index].commands.splice(cmdIndex, 1);
}

function addSubCommand(index, subIndex = null) {
  if (subIndex === null) {
    config.value.commands[index].submenu = config.value.commands[index].submenu || [];
    config.value.commands[index].submenu.push({ name: '', command: '', hotkey: null, submenu: [], commands: [] });
  } else {
    config.value.commands[index].submenu[subIndex].submenu = config.value.commands[index].submenu[subIndex].submenu || [];
    config.value.commands[index].submenu[subIndex].submenu.push({ name: '', command: '', hotkey: null, submenu: [], commands: [] });
  }
}

function deleteSubCommand(index, subIndex) {
  config.value.commands[index].submenu.splice(subIndex, 1);
}

function addInnerSubCommand(index, subIndex) {
  config.value.commands[index].submenu[subIndex].commands = config.value.commands[index].submenu[subIndex].commands || [];
  config.value.commands[index].submenu[subIndex].commands.push('');
}

function deleteInnerSubCommand(index, subIndex, cmdIndex) {
  config.value.commands[index].submenu[subIndex].commands.splice(cmdIndex, 1);
}

function toggleHotkeyInput(index, isEnabled, isSub = false, subIndex = null) {
  if (isSub) {
    const subCommand = config.value.commands[index].submenu[subIndex];
    subCommand.hotkey = subCommand.hotkey ? null : '';
  } else {
    const command = config.value.commands[index];
    command.hotkey = command.hotkey ? null : '';
  }
}

function hotkeyHandler(event, index, subIndex = null) {
  event.preventDefault();
  let hotkey = [];
  if (event.ctrlKey) hotkey.push('Ctrl');
  if (event.shiftKey) hotkey.push('Shift');
  if (event.altKey) hotkey.push('Alt');
  if (event.metaKey) hotkey.push('Command');
  if (event.key && !['Control', 'Shift', 'Alt', 'Meta'].includes(event.key)) hotkey.push(event.key);

  if (subIndex !== null) {
    config.value.commands[index].submenu[subIndex].hotkey = hotkey.join('+');
  } else {
    config.value.commands[index].hotkey = hotkey.join('+');
  }
}

function hotkeyHandlerMenu(event) {
  event.preventDefault();
  let hotkey = [];
  if (event.ctrlKey) hotkey.push('Ctrl');
  if (event.shiftKey) hotkey.push('Shift');
  if (event.altKey) hotkey.push('Alt');
  if (event.metaKey) hotkey.push('Command');
  if (event.key && !['Control', 'Shift', 'Alt', 'Meta'].includes(event.key)) hotkey.push(event.key);
  config.value.menu_hotkey = hotkey.join('+');
}

function showNotification(message, type) {
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
  const defaultConfig = {
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
  appWindow.hide();
}

onMounted(loadConfigs);
</script>

<style>
/* Add styles from editor.html */
.container {
  border-radius: 12px;
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
  width: 80%;
  text-align: left;
}

.form-group input, .form-group select {
  padding: 10px;
  font-size: 16px;
  border: 1px solid #ccc;
  border-radius: 6px;
  width: 100%;
  box-shadow: inset 0 1px 3px rgba(0, 0, 0, 0.1);
  transition: border-color 0.2s, box-shadow 0.2s;
}

.form-group input:focus, .form-group select:focus {
  border-color: #007aff;
  box-shadow: 0 0 3px 2px rgba(0, 123, 255, 0.25);
  outline: none;
}

.buttons {
  display: flex;
  justify-content: center;
  margin-top: 2px;
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
  margin: 0 10px;
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

table {
  width: 100%;
  border-collapse: collapse;
}

th, td {
  padding: 10px;
  border: 1px solid #ccc;
  text-align: left;
}

th {
  background: #f0f0f5;
}

.modal {
  display: block;
  background: rgba(0, 0, 0, 0.5);
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  justify-content: center;
  align-items: center;
  z-index: 3;
}

.modal-dialog {
  background: #fff;
  border-radius: 12px;
  box-shadow: 0 4px 15px rgba(0, 0, 0, 0.1);
  padding: 20px;
  width: 80%;
  max-width: 400px;
  margin: auto;
  margin-top: 50px;
}

.modal-header, .modal-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.modal-header {
  border-bottom: 1px solid #e0e0e0;
  padding-bottom: 10px;
  margin-bottom: 20px;
}

.modal-footer {
  border-top: 1px solid #e0e0e0;
  padding-top: 10px;
  margin-top: 20px;
}

.modal-title {
  font-size: 18px;
  color: #333;
}

.close {
  background: none;
  border: none;
  font-size: 24px;
  cursor: pointer;
}
</style>
