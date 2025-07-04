<!-- src/components/ConfigEditor.vue -->
<template>
  <div class="space-y-6">
    <!-- Basic Settings -->
    <div class="space-y-4">
      <h2 class="text-lg font-semibold text-slate-900 border-b border-slate-200 pb-2">
        Basic Settings
      </h2>
      
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <div>
          <label class="block text-sm font-medium text-slate-700 mb-2">Terminal</label>
          <CustomSelect
            v-model="config.terminal"
            :options="terminalOptions"
            placeholder="Select terminal"
            required
          />
        </div>
        
        <div>
          <label class="block text-sm font-medium text-slate-700 mb-2">Launch in</label>
          <CustomSelect
            v-model="config.launch_in"
            :options="launchOptions"
            placeholder="Select launch option"
            required
          />
        </div>
      </div>
      
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <Input
          v-model="config.title"
          label="Title"
          placeholder="Enter title"
          required
        />
        
        <HotkeyInput
          v-model="config.menu_hotkey"
          label="Menu Hotkey"
          placeholder="Click to record"
          hint="Leave empty to disable"
        />
      </div>

      <!-- Enable/Disable Configuration -->
      <div class="flex items-center justify-between p-4 bg-slate-50 rounded-lg">
        <div>
          <h3 class="text-sm font-medium text-slate-900">Configuration Status</h3>
          <p class="text-sm text-slate-600">
            {{ config.enabled ? 'Configuration is enabled and will be loaded' : 'Configuration is disabled and will be ignored' }}
          </p>
        </div>
        <div class="flex items-center">
          <button
            @click="toggleEnabled"
            :class="[
              'relative inline-flex h-6 w-11 items-center rounded-full transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2',
              config.enabled ? 'bg-blue-600' : 'bg-slate-200'
            ]"
            :aria-checked="config.enabled"
            role="switch"
            type="button"
          >
            <span
              :class="[
                'inline-block h-4 w-4 transform rounded-full bg-white transition-transform',
                config.enabled ? 'translate-x-6' : 'translate-x-1'
              ]"
            />
          </button>
          <span class="ml-3 text-sm font-medium text-slate-900">
            {{ config.enabled ? 'Enabled' : 'Disabled' }}
          </span>
        </div>
      </div>
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
import CustomSelect from './CustomSelect.vue';

import { Command, Config } from '../types';

const props = defineProps<{
  config: Config;
  commands: Command[];
}>();

const config = ref<Config>(props.config);

// Убеждаемся, что enabled имеет значение по умолчанию
if (config.value.enabled === undefined) {
  config.value.enabled = true;
}

const toggleEnabled = () => {
  config.value.enabled = !config.value.enabled;
};

const terminalOptions = [
  { value: 'iterm', label: 'iTerm2', icon: '🖥️' },
  { value: 'terminal', label: 'Terminal.app', icon: '💻' },
  { value: 'alacritty', label: 'Alacritty', icon: '⚡' },
  { value: 'hyper', label: 'Hyper', icon: '🚀' },
  { value: 'warp', label: 'Warp', icon: '⚡' }
];

const launchOptions = [
  { value: 'current', label: 'Current Window', icon: '🪟' },
  { value: 'tab', label: 'New Tab', icon: '📑' },
  { value: 'window', label: 'New Window', icon: '🪟' }
];
</script>