<template>
  <div
    class="border border-slate-200 rounded-xl p-6 space-y-6 bg-white shadow-md hover:shadow-lg transition-shadow duration-200"
  >
    <!-- Command Header -->
    <div
      class="flex items-center justify-between pb-4 border-b border-slate-200 -mx-6 px-6"
    >
      <div class="flex items-center space-x-3">
        <div
          class="w-8 h-8 bg-blue-100 flex items-center justify-center rounded-lg"
        >
          <LightningSmallIcon />
        </div>
        <h4 class="font-semibold text-slate-900">
          Command {{ index + 1 }}{{ command.name ? ` - ${command.name}` : '' }}
        </h4>
      </div>
      <div class="flex items-center space-x-1">
        <CustomButton
          variant="ghost"
          size="sm"
          :disabled="index === 0"
          @click="$emit('move', index, -1)"
        >
          <ChevronUpIcon />
        </CustomButton>
        <CustomButton
          variant="ghost"
          size="sm"
          :disabled="index === (parentCommands?.length || 0) - 1"
          @click="$emit('move', index, 1)"
        >
          <ChevronDownIcon />
        </CustomButton>
        <CustomButton
          variant="danger"
          size="sm"
          @click="$emit('remove', index)"
        >
          <TrashIcon />
        </CustomButton>
      </div>
    </div>

    <!-- Basic Command Fields -->
    <div class="grid gap-6 grid-cols-1 md:grid-cols-2">
      <div class="flex items-start gap-3">
        <div class="w-16">
          <IconSelector
            v-model="commandIcon"
            label="Icon"
            placeholder="emoji"
            input-class="px-2.5 py-1.5"
            @update:modelValue="handleIconChange"
          />
        </div>
        <div class="flex-1">
          <ValidatedField
            :model-value="command.name"
            :rules="fieldRules.commandName"
            :hide-error="true"
            @update:model-value="handleNameChange"
          >
            <template #default="{ value, error, updateValue }">
              <Input
                :model-value="value"
                label="Name"
                placeholder="Command name"
                required
                :error="error"
                @update:model-value="updateValue"
              />
            </template>
          </ValidatedField>
        </div>
      </div>

      <HotkeyInput
        v-model="command.hotkey"
        label="Hotkey"
        placeholder="Click to record"
      />
    </div>

    <!-- Commands Section -->
    <div class="space-y-6">
      <div class="space-y-4">
        <div class="flex items-center justify-between">
          <label class="block text-sm font-semibold text-slate-700"
            >Commands</label
          >
          <CustomButton
            variant="ghost"
            size="sm"
            @click="handleAddMultipleCommand"
          >
            <AddIcon />
            Add Command
          </CustomButton>
        </div>

        <div
          v-if="command.commands && command.commands.length > 0"
          class="space-y-3"
        >
          <div
            v-for="(_, cmdIndex) in command.commands"
            :key="cmdIndex"
            class="group"
          >
            <div class="flex items-start gap-2">
              <div class="flex-1">
                <CommandInput
                  v-model="command.commands[cmdIndex]"
                  :placeholder="`Enter command ${cmdIndex + 1}`"
                  :configId="configId"
                />
              </div>

              <CustomButton
                variant="danger"
                size="sm"
                class="flex-shrink-0"
                @click="handleRemoveMultipleCommand(cmdIndex)"
              >
                <XIcon />
              </CustomButton>
            </div>
          </div>
        </div>
      </div>



      <!-- Advanced Options Spoiler -->
      <CollapsibleSection
        title="Advanced Options"
        :summary="getAdvancedOptionsSummary()"
      >
        <!-- Background Execution Option -->
        <div class="space-y-4">
          <div class="flex items-center space-x-3">
            <input
              id="background"
              v-model="command.background"
              type="checkbox"
              class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 focus:ring-2"
            />
            <label for="background" class="text-sm font-medium text-slate-700">
              Execute in background (ConsolePool)
            </label>
          </div>
          <p class="text-xs text-slate-500">
            When enabled, commands will be executed in background using
            ConsolePool. When disabled, commands will be executed through normal
            terminal execution.
          </p>
        </div>

        <!-- Divider -->
        <div class="border-t-2 border-slate-200/70 my-8 -mx-6"></div>
        <!-- Monitor Field -->
        <div class="space-y-1">
          <label class="block text-sm font-medium text-slate-700"
            >Monitor Command (optional)</label
          >
          <CommandInput
            :model-value="command.monitor || ''"
            placeholder="Command to get display value for monitoring (e.g., echo 'CPU: 45%')"
            :configId="configId"
            @update:model-value="(value: string) => (command.monitor = value)"
          />
        </div>

        <!-- Divider -->
        <div class="border-t-2 border-slate-200/70 my-8 -mx-6"></div>

        <!-- Inputs Section -->
        <div class="space-y-4">
          <div class="flex items-center justify-between">
            <label class="block text-sm font-semibold text-slate-700"
              >Inputs</label
            >
            <CustomButton
              variant="ghost"
              size="sm"
              class="text-blue-600 hover:text-blue-700 hover:bg-blue-50"
              @click="handleAddInput"
            >
              <AddIcon />
              Add Field
            </CustomButton>
          </div>

          <div
            v-if="command.inputs && Object.keys(command.inputs).length > 0"
            class="space-y-2"
          >
            <!-- Table Headers -->
            <div class="flex items-center gap-2 py-1 px-1 rounded-lg">
              <div class="flex-1">
                <span
                  class="text-xs font-semibold text-slate-700 uppercase tracking-wide"
                  >Key</span
                >
              </div>
              <div class="flex-1">
                <span
                  class="text-xs font-semibold text-slate-700 uppercase tracking-wide"
                  >Default Value</span
                >
              </div>
              <div class="w-8"></div>
            </div>
            <!-- Table Rows -->
            <div
              v-for="(_, key) in command.inputs"
              :key="key"
              class="flex items-center gap-2 py-1"
            >
              <div class="flex-1">
                <ValidatedField
                  :model-value="isRootLevel && inputKeys[index] ? inputKeys[index][key] : key"
                  :rules="fieldRules.inputKey"
                  :hide-error="true"
                  @update:model-value="(value: string) => handleInputKeyChange(key, value)"
                >
                  <template #default="{ value, error, updateValue }">
                    <Input
                      :model-value="value"
                      placeholder="Key"
                      size="sm"
                      input-class="border border-slate-300 bg-white rounded px-2 py-1 focus:border-blue-400 focus:ring-0"
                      :error="error"
                      @update:model-value="updateValue"
                    />
                  </template>
                </ValidatedField>
              </div>
              <div class="flex-1">
                <ValidatedField
                  :model-value="command.inputs?.[key] || ''"
                  :rules="fieldRules.inputValue"
                  :hide-error="true"
                  @update:model-value="(value: string) => handleInputValueChange(key, value)"
                >
                  <template #default="{ value, error, updateValue }">
                    <Input
                      :model-value="value"
                      placeholder="Default value"
                      size="sm"
                      input-class="border border-slate-300 bg-white rounded px-2 py-1 focus:border-blue-400 focus:ring-0"
                      :error="error"
                      @update:model-value="updateValue"
                    />
                  </template>
                </ValidatedField>
              </div>
              <CustomButton
                variant="danger"
                size="sm"
                class="flex-shrink-0 w-8"
                @click="() => handleRemoveInput(key)"
              >
                <XIcon />
              </CustomButton>
            </div>
          </div>
        </div>

        <!-- Scheduler Configuration -->
        <SchedulerInput v-model="command.scheduler" />
      </CollapsibleSection>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { computed, onMounted } from 'vue';
import { PropType } from 'vue';
import { Command } from '../../types';
import Input from '../ui/Input.vue';
import CustomButton from '../ui/CustomButton.vue';
import HotkeyInput from '../forms/HotkeyInput.vue';
import IconSelector from '../forms/IconSelector.vue';
import CollapsibleSection from '../ui/CollapsibleSection.vue';
import SchedulerInput from '../forms/SchedulerInput.vue';
import CommandInput from './CommandInput.vue';
import LightningSmallIcon from '../icons/LightningSmallIcon.vue';
import ChevronUpIcon from '../icons/ChevronUpIcon.vue';
import ChevronDownIcon from '../icons/ChevronDownIcon.vue';
import TrashIcon from '../icons/TrashIcon.vue';
import AddIcon from '../icons/AddIcon.vue';
import XIcon from '../icons/XIcon.vue';
import ValidatedField from '../ui/ValidatedField.vue';
import { fieldRules } from '../../lib/validation-rules';

const props = defineProps({
  command: {
    type: Object as PropType<Command>,
    required: true,
  },
  index: {
    type: Number,
    required: true,
  },
  inputKeys: {
    type: Object as PropType<Record<number, Record<string, string>>>,
    required: true,
  },
  level: {
    type: Number,
    default: 0,
  },
  parentCommands: {
    type: Array as PropType<Command[]>,
    default: null,
  },
  configId: {
    type: String,
    default: '',
  },
});



const emit = defineEmits<{
  (e: 'update:command', value: Command): void;
  (e: 'remove', index: number): void;
  (e: 'move', index: number, direction: number): void;
  (e: 'add-input', index: number): void;
  (e: 'remove-input', index: number, key: string): void;
  (e: 'update-input-key', index: number, oldKey: string, newKey: string): void;
  (e: 'add-multiple-command', index: number): void;
  (e: 'remove-multiple-command', index: number, cmdIndex: number): void;
}>();

// Initialize commands if they don't exist
onMounted(() => {
  if (!props.command.commands) {
    props.command.commands = [''];
    emit('update:command', props.command);
  }
});



// Computed properties for determining logic
const isRootLevel = computed(() => props.level === 0);

// Computed property for icon handling
const commandIcon = computed({
  get: () => props.command.icon || '',
  set: (value: string) => {
    if (value.trim() === '') {
      props.command.icon = null;
    } else {
      props.command.icon = value;
    }
    emit('update:command', props.command);
  },
});

// Method for handling icon changes
const handleIconChange = (value: string) => {
  if (value.trim() === '') {
    props.command.icon = null;
  } else {
    props.command.icon = value;
  }
  emit('update:command', props.command);
};

// Method for handling name changes
const handleNameChange = (value: string) => {
  props.command.name = value;
  emit('update:command', props.command);
};

// Functions for handling inputs
const handleAddInput = () => {
  if (isRootLevel.value) {
    emit('add-input', props.index);
  }
};

const handleRemoveInput = (key: string) => {
  if (isRootLevel.value) {
    emit('remove-input', props.index, key);
  }
};

const handleInputKeyChange = (key: string, value: string) => {
  if (isRootLevel.value) {
    emit('update-input-key', props.index, key, value);
  }
};

const handleInputValueChange = (key: string, value: string) => {
  if (props.command.inputs) {
    props.command.inputs[key] = value;
    emit('update:command', props.command);
  }
};

// Functions for handling multiple commands
const handleAddMultipleCommand = () => {
  if (isRootLevel.value) {
    emit('add-multiple-command', props.index);
  }
};

const handleRemoveMultipleCommand = (cmdIndex: number) => {
  if (isRootLevel.value) {
    emit('remove-multiple-command', props.index, cmdIndex);
  }
};

// Function to get brief information about extended options
const getAdvancedOptionsSummary = () => {
  const cmd = props.command;
  const parts = [];

  if (cmd.background) {
    parts.push('Background');
  }

  if (cmd.monitor && cmd.monitor.trim()) {
    parts.push('Monitor');
  }

  if (cmd.inputs && Object.keys(cmd.inputs).length > 0) {
    parts.push(
      `${Object.keys(cmd.inputs).length} input${Object.keys(cmd.inputs).length > 1 ? 's' : ''}`
    );
  }

  if (cmd.scheduler && cmd.scheduler.trim()) {
    parts.push('Scheduler');
  }

  return parts.length > 0 ? parts.join(', ') : 'None configured';
};
</script>
