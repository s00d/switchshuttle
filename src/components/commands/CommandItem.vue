<template>
  <div :class="ui.root()">
    <div :class="ui.header()">
      <div :class="ui.headerLeft()">
        <div :class="ui.iconWrap()">
          <LightningSmallIcon />
        </div>
        <h4 :class="ui.title()">
          Command {{ index + 1 }}{{ command.name ? ` - ${command.name}` : '' }}
        </h4>
      </div>
      <div :class="ui.actions()">
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

    <div :class="ui.fieldsGrid()">
      <div :class="ui.fieldsRow()">
        <div :class="ui.iconField()">
          <IconSelector
            v-model="commandIcon"
            label="Icon"
            placeholder="emoji"
            input-class="px-2.5 py-1.5"
            @update:modelValue="handleIconChange"
          />
        </div>
        <div :class="ui.nameField()">
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

    <div :class="ui.section()">
      <div :class="ui.sectionInner()">
        <div :class="ui.sectionHeader()">
          <label :class="ui.sectionLabel()">Commands</label>
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
          :class="ui.commandList()"
        >
          <div
            v-for="(_, cmdIndex) in command.commands"
            :key="cmdIndex"
          >
            <div :class="ui.commandRow()">
              <div :class="ui.commandField()">
                <CommandInput
                  v-model="command.commands[cmdIndex]"
                  :placeholder="`Enter command ${cmdIndex + 1}`"
                  :configId="configId"
                />
              </div>

              <CustomButton
                variant="danger"
                size="sm"
                :class="ui.shrinkBtn()"
                @click="handleRemoveMultipleCommand(cmdIndex)"
              >
                <XIcon />
              </CustomButton>
            </div>
          </div>
        </div>
      </div>

      <CollapsibleSection
        title="Advanced Options"
        :summary="getAdvancedOptionsSummary()"
      >
        <div :class="ui.sectionInner()">
          <div :class="ui.checkboxRow()">
            <input
              id="background"
              v-model="command.background"
              type="checkbox"
              :class="ui.checkbox()"
            />
            <label for="background" :class="ui.checkboxLabel()">
              Execute in background (tracked in tray)
            </label>
          </div>
          <p :class="ui.hint()">
            When enabled, commands run as app-managed processes and appear under
            Running in the system tray (Stop to kill). When disabled, commands
            open in your configured terminal (not stoppable from the tray).
          </p>
        </div>

        <div :class="ui.divider()"></div>

        <div :class="ui.fieldStack()">
          <label :class="ui.fieldLabel()">Monitor Command (optional)</label>
          <CommandInput
            :model-value="command.monitor || ''"
            placeholder="Command to get display value for monitoring (e.g., echo 'CPU: 45%')"
            :configId="configId"
            @update:model-value="(value: string) => (command.monitor = value)"
          />
        </div>

        <div :class="ui.divider()"></div>

        <div :class="ui.sectionInner()">
          <div :class="ui.sectionHeader()">
            <label :class="ui.sectionLabel()">Inputs</label>
            <CustomButton
              variant="ghost"
              size="sm"
              :class="ui.ghostAccent()"
              @click="handleAddInput"
            >
              <AddIcon />
              Add Field
            </CustomButton>
          </div>

          <div
            v-if="command.inputs && Object.keys(command.inputs).length > 0"
            :class="ui.inputsList()"
          >
            <div :class="ui.inputsHeader()">
              <div :class="ui.inputsCol()">
                <span :class="ui.inputsColLabel()">Key</span>
              </div>
              <div :class="ui.inputsCol()">
                <span :class="ui.inputsColLabel()">Default Value</span>
              </div>
              <div :class="ui.inputsSpacer()"></div>
            </div>
            <div
              v-for="(_, key) in command.inputs"
              :key="key"
              :class="ui.inputsRow()"
            >
              <div :class="ui.inputsCol()">
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
                      :input-class="ui.inputCompact()"
                      :error="error"
                      @update:model-value="updateValue"
                    />
                  </template>
                </ValidatedField>
              </div>
              <div :class="ui.inputsCol()">
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
                      :input-class="ui.inputCompact()"
                      :error="error"
                      @update:model-value="updateValue"
                    />
                  </template>
                </ValidatedField>
              </div>
              <CustomButton
                variant="danger"
                size="sm"
                :class="ui.removeBtn()"
                @click="() => handleRemoveInput(key)"
              >
                <XIcon />
              </CustomButton>
            </div>
          </div>
        </div>

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
import { commandCardTv } from './commandCardTheme';

defineOptions({ name: 'CommandItem' });

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

const ui = commandCardTv({ tone: 'command' });

onMounted(() => {
  if (!props.command.commands) {
    props.command.commands = [''];
    emit('update:command', props.command);
  }
});

const isRootLevel = computed(() => props.level === 0);

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

const handleIconChange = (value: string) => {
  if (value.trim() === '') {
    props.command.icon = null;
  } else {
    props.command.icon = value;
  }
  emit('update:command', props.command);
};

const handleNameChange = (value: string) => {
  props.command.name = value;
  emit('update:command', props.command);
};

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
