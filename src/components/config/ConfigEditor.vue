<template>
  <div class="space-y-6">
    <!-- Basic Settings -->
    <div class="space-y-4">
      <h2
        class="text-lg font-semibold text-slate-900 border-b border-slate-200 pb-2"
      >
        Basic Settings
      </h2>

      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <ValidatedField
          v-model="config.terminal"
          :rules="fieldRules.terminal"
          :hide-error="true"
        >
          <template #default="{ value, error, updateValue }">
            <div>
              <label class="block text-sm font-medium text-slate-700 mb-2">Terminal</label>
              <CustomSelect
                :model-value="value"
                :options="terminalOptionsArray"
                placeholder="Loading terminals..."
                required
                :disabled="loadingTerminals"
                :error="error"
                @update:model-value="updateValue"
              />
            </div>
          </template>
        </ValidatedField>

        <ValidatedField
          v-model="config.launch_in"
          :rules="fieldRules.launchIn"
          :hide-error="true"
        >
          <template #default="{ value, error, updateValue }">
            <div>
              <label class="block text-sm font-medium text-slate-700 mb-2">Launch in</label>
              <CustomSelect
                :model-value="value"
                :options="launchOptions"
                placeholder="Select launch option"
                required
                :error="error"
                @update:model-value="updateValue"
              />
            </div>
          </template>
        </ValidatedField>
      </div>

      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <ValidatedField
          v-model="config.title"
          :rules="fieldRules.title"
          :hide-error="true"
        >
          <template #default="{ value, error, updateValue }">
            <Input
              :model-value="value"
              label="Title"
              placeholder="Enter title"
              required
              :error="error"
              @update:model-value="updateValue"
            />
          </template>
        </ValidatedField>

        <ValidatedField
          v-model="config.menu_hotkey"
          :rules="fieldRules.hotkey"
          :hide-error="true"
        >
          <template #default="{ value, error, updateValue }">
            <HotkeyInput
              :model-value="value"
              label="Menu Hotkey"
              placeholder="Click to record"
              hint="Leave empty to disable"
              :error="error"
              @update:model-value="updateValue"
            />
          </template>
        </ValidatedField>
      </div>

      <!-- Enable/Disable Configuration -->
      <div class="p-4 bg-slate-50 rounded-lg">
        <Toggle
          :model-value="config.enabled ?? true"
          label="Configuration Status"
          description="Enable or disable this configuration. Disabled configurations will be ignored."
          @update:model-value="(value: boolean) => (config.enabled = value)"
        />
      </div>
    </div>

    <!-- Commands Section -->
    <div class="space-y-4">
      <div class="flex items-center justify-between">
        <h3 class="text-md font-medium text-slate-900">Commands</h3>
        <ValidatedField
          v-model="config.commands"
          :rules="fieldRules.commands"
          :hide-error="true"
        >
          <template #default="{ error }">
            <div v-if="error" class="text-red-500 text-sm">{{ error }}</div>
          </template>
        </ValidatedField>
      </div>
      <CommandsTable
        :commands="config.commands"
        :configId="config.title"
        @update:commands="config.commands = $event"
      />
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref, computed } from 'vue';
import CommandsTable from '../commands/CommandsTable.vue';
import Input from '../ui/Input.vue';
import HotkeyInput from '../forms/HotkeyInput.vue';
import CustomSelect from '../forms/CustomSelect.vue';
import Toggle from '../ui/Toggle.vue';
import ValidatedField from '../ui/ValidatedField.vue';
import { fieldRules } from '../../lib/validation-rules';
import { Command, Config } from '../../types';
import { TerminalConfig } from '../../lib/tauri-commands';

// Опции запуска (одинаковые для всех ОС)
const launchOptions = [
  { value: 'current', label: 'Current Window', icon: '📍' },
  { value: 'new_tab', label: 'New Tab', icon: '📑' },
  { value: 'new_window', label: 'New Window', icon: '🪟' },
];

const props = defineProps<{
  config: Config;
  commands: Command[];
  terminalOptions: Record<string, TerminalConfig>;
  loadingTerminals: boolean;
}>();

const config = ref<Config>(props.config);

// Убеждаемся, что enabled имеет значение по умолчанию
if (config.value.enabled === undefined) {
  config.value.enabled = true;
}

// Преобразуем Record<string, TerminalConfig> в массив для CustomSelect
const terminalOptionsArray = computed(() => {
  return Object.entries(props.terminalOptions).map(([key, config]) => ({
    value: key,
    label: config.name,
    icon: config.icon,
  }));
});
</script>
