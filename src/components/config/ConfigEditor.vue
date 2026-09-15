<template>
  <div :class="ui.root()">
    <div :class="ui.section()">
      <h2 :class="ui.sectionTitle()">Basic Settings</h2>

      <div :class="ui.grid()">
        <ValidatedField
          v-model="config.terminal"
          :rules="fieldRules.terminal"
          :hide-error="true"
        >
          <template #default="{ value, error, updateValue }">
            <div>
              <label :class="ui.label()">Terminal</label>
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
              <label :class="ui.label()">Launch in</label>
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

      <div :class="ui.grid()">
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

      <div :class="ui.toggleWrap()">
        <Toggle
          :model-value="config.enabled ?? true"
          label="Configuration Status"
          description="Enable or disable this configuration. Disabled configurations will be ignored."
          @update:model-value="(value: boolean) => (config.enabled = value)"
        />
      </div>
    </div>

    <div :class="ui.section()">
      <div :class="ui.commandsHeader()">
        <h3 :class="ui.commandsTitle()">Commands</h3>
        <ValidatedField
          v-model="config.commands"
          :rules="fieldRules.commands"
          :hide-error="true"
        >
          <template #default="{ error }">
            <div v-if="error" :class="ui.error()">{{ error }}</div>
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
import { tv } from '@/lib/tv';
import CommandsTable from '../commands/CommandsTable.vue';
import Input from '../ui/Input.vue';
import HotkeyInput from '../forms/HotkeyInput.vue';
import CustomSelect from '../forms/CustomSelect.vue';
import Toggle from '../ui/Toggle.vue';
import ValidatedField from '../ui/ValidatedField.vue';
import { fieldRules } from '../../lib/validation-rules';
import { Command, Config } from '../../types';
import { TerminalConfig } from '../../lib/tauri-commands';

defineOptions({ name: 'ConfigEditor' });

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

if (config.value.enabled === undefined) {
  config.value.enabled = true;
}

const configEditorTv = tv({
  slots: {
    root: 'space-y-3',
    section: 'space-y-2.5',
    sectionTitle:
      'text-base font-semibold text-slate-900 border-b border-slate-200 pb-1.5',
    grid: 'grid grid-cols-1 md:grid-cols-2 gap-3',
    label: 'block text-sm font-medium text-slate-700 mb-1',
    toggleWrap: 'p-3 bg-slate-50 rounded-md',
    commandsHeader: 'flex items-center justify-between gap-2',
    commandsTitle: 'text-sm font-medium text-slate-900',
    error: 'text-red-500 text-xs',
  },
});

const ui = configEditorTv();

const terminalOptionsArray = computed(() => {
  return Object.entries(props.terminalOptions).map(([key, terminalConfig]) => ({
    value: key,
    label: terminalConfig.name,
    icon: terminalConfig.icon,
  }));
});
</script>
