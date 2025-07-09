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
      <div class="p-4 bg-slate-50 rounded-lg">
        <Toggle
          :model-value="config.enabled ?? true"
          label="Configuration Status"
          description="Enable or disable this configuration. Disabled configurations will be ignored."
          @update:model-value="(value: boolean) => config.enabled = value"
        />
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
import Toggle from './Toggle.vue';

import { Command, Config } from '../types';
import { getTerminalOptions, launchOptions } from '../lib/terminals';

const props = defineProps<{
  config: Config;
  commands: Command[];
}>();

const config = ref<Config>(props.config);

// Убеждаемся, что enabled имеет значение по умолчанию
if (config.value.enabled === undefined) {
  config.value.enabled = true;
}

const terminalOptions = getTerminalOptions();
</script>