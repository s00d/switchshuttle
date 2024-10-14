<template>
  <div>
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
      <input type="text" id="menu-hotkey" class="form-control" v-model="config.menu_hotkey" @keydown="handleHotkey(config, 'menu_hotkey', $event)">
    </div>
    <CommandsTable :commands="commands" @addCommand="addCommand" @deleteCommand="deleteCommand" @addInnerCommand="addInnerCommand" @deleteInnerCommand="deleteInnerCommand" @toggleHotkeyInput="toggleHotkeyInput" @hotkeyHandler="handleHotkey" />
    <div class="button-group d-flex justify-content-between mt-4">
      <button class="button-blue" @click="$emit('saveConfig')">Save</button>
      <button class="cancel-button" @click="$emit('onClose')">Cancel</button>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref } from 'vue';
import CommandsTable from './CommandsTable.vue';
import { Command, Config } from '../types';
import { handleHotkey } from '../utils';

const props = defineProps<{
  config: Config;
  commands: Command[];
}>();

defineEmits<{
  (e: 'saveConfig'): void;
  (e: 'onClose'): void;
}>();

const config = ref<Config>(props.config);

function addCommand() {
  config.value.commands.push({ name: '', command: '', hotkey: null, submenu: null, commands: [] });
}

function deleteCommand(index: number) {
  config.value.commands.splice(index, 1);
}

function addInnerCommand(index: number) {
  const command = config.value.commands[index];
  command.commands = command.commands || [];
  command.commands.push('');
}

function deleteInnerCommand(index: number, cmdIndex: number) {
  const command = config.value.commands[index];
  command.commands.splice(cmdIndex, 1);
}

function toggleHotkeyInput(index: number) {
  const command = config.value.commands[index];
  command.hotkey = command.hotkey ? null : '';
}
</script>

<style scoped>
.form-control {
  height: 32px; /* Ensure the height is set to 20px */
  padding: 0; /* Remove default padding */
  border: 1px solid #ccc; /* Optional: Customize border */
  box-sizing: border-box; /* Ensure padding and border are included in the height */
  font-size: 14px; /* Adjust the font size as needed */
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
  width: 100%;
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

.d-flex {
  display: flex;
  width: 100%;
  justify-content: center;
}

.button-group {
  width: 100%;
}
</style>
