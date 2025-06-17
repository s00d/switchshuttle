<!-- src/components/ConfigEditor.vue -->
<template>
  <div class="space-y-6">
    <!-- Basic Settings -->
    <div class="space-y-4">
      <h2 class="text-lg font-semibold text-slate-900 border-b border-slate-200 pb-2">
        Basic Settings
      </h2>
      
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <Input
          v-model="config.terminal"
          label="Terminal"
          type="select"
          :options="terminalOptions"
          required
        />
        
        <Input
          v-model="config.launch_in"
          label="Launch in"
          type="select"
          :options="launchOptions"
          required
        />
      </div>
      
      <div class="grid grid-cols-1 gap-4">
        <Input
          v-model="config.title"
          label="Title"
          placeholder="Enter title"
          required
        />
      </div>
      
      <HotkeyInput
        v-model="config.menu_hotkey"
        label="Menu Hotkey"
        placeholder="Click to record"
        hint="Leave empty to disable"
      />
    </div>

    <!-- Commands Section -->
    <div class="space-y-4">
      <CommandsTable :commands="config.commands" @update:commands="config.commands = $event" />
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref } from 'vue';
import CommandsTable from './CommandsTable.vue';
import Input from './Input.vue';
import HotkeyInput from './HotkeyInput.vue';
import { Command, Config } from '../types';

const props = defineProps<{
  config: Config;
  commands: Command[];
}>();

const config = ref<Config>(props.config);

const terminalOptions = [
  { value: 'iterm', label: 'iTerm2' },
  { value: 'terminal', label: 'Terminal.app' },
  { value: 'alacritty', label: 'Alacritty' },
  { value: 'hyper', label: 'Hyper' },
  { value: 'warp', label: 'Warp' }
];

const launchOptions = [
  { value: 'current', label: 'Current Window' },
  { value: 'tab', label: 'New Tab' },
  { value: 'window', label: 'New Window' }
];
</script>