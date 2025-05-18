<!-- src/components/ConfigEditor.vue -->
<template>
  <div class="space-y-6">
    <div class="space-y-2">
      <label for="terminal" class="block text-sm font-medium text-gray-700">Terminal</label>
      <select
          id="terminal"
          v-model="config.terminal"
          class="w-full border border-gray-300 px-3 py-2 text-sm bg-white text-gray-800 focus:outline-none focus:ring-2 focus:ring-blue-500"
      >
        <option value="iterm">iTerm</option>
        <option value="terminal">Terminal</option>
        <option value="warp">Warp</option>
        <option value="hyper">Hyper</option>
        <option value="alacritty">Alacritty</option>
      </select>
    </div>

    <div class="space-y-2">
      <label for="launch-in" class="block text-sm font-medium text-gray-700">Launch In</label>
      <select
          id="launch-in"
          v-model="config.launch_in"
          class="w-full border border-gray-300 px-3 py-2 text-sm bg-white text-gray-800 focus:outline-none focus:ring-2 focus:ring-blue-500"
      >
        <option value="current">Current</option>
        <option value="new_tab">New Tab</option>
        <option value="new_window">New Window</option>
      </select>
    </div>

    <div class="space-y-2">
      <label for="theme" class="block text-sm font-medium text-gray-700">Theme</label>
      <input
          type="text"
          id="theme"
          v-model="config.theme"
          class="w-full border border-gray-300 px-3 py-2 text-sm text-gray-800 focus:outline-none focus:ring-2 focus:ring-blue-500"
      />
    </div>

    <div class="space-y-2">
      <label for="title" class="block text-sm font-medium text-gray-700">Title</label>
      <input
          type="text"
          id="title"
          v-model="config.title"
          class="w-full border border-gray-300 px-3 py-2 text-sm text-gray-800 focus:outline-none focus:ring-2 focus:ring-blue-500"
      />
    </div>

    <div class="space-y-2">
      <label for="menu-hotkey" class="block text-sm font-medium text-gray-700">Menu Hotkey</label>
      <input
          type="text"
          id="menu-hotkey"
          v-model="config.menu_hotkey"
          @keydown="handleHotkey(config, 'menu_hotkey', $event)"
          class="w-full border border-gray-300 px-3 py-2 text-sm text-gray-800 focus:outline-none focus:ring-2 focus:ring-blue-500"
      />
    </div>

    <CommandsTable
        :commands="commands"
        @addCommand="addCommand"
        @deleteCommand="deleteCommand"
        @addInnerCommand="addInnerCommand"
        @deleteInnerCommand="deleteInnerCommand"
        @toggleHotkeyInput="toggleHotkeyInput"
        @hotkeyHandler="handleHotkey"
    />

    <div class="flex justify-between gap-4 pt-4">
      <button
          @click="$emit('saveConfig')"
          class="w-full bg-blue-600 text-white px-4 py-2 text-sm hover:bg-blue-700 transition"
      >
        Save
      </button>
      <button
          @click="$emit('onClose')"
          class="w-full bg-gray-300 text-black px-4 py-2 text-sm hover:bg-gray-400 transition"
      >
        Cancel
      </button>
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