<template>
  <div :class="ui.root()">
    <div :class="ui.header()">
      <h3 :class="ui.title()">Commands</h3>
      <div :class="ui.actions()">
        <CustomButton variant="primary" size="sm" @click="showTemplatesModal">
          <TemplatesIcon />
          Templates
        </CustomButton>
        <CustomButton variant="secondary" size="sm" @click="addCommand">
          <AddIcon />
          Add Command
        </CustomButton>
        <CustomButton variant="secondary" size="sm" @click="addGroup">
          <AddIcon />
          Add Group
        </CustomButton>
      </div>
    </div>

    <div v-if="commands.length === 0" :class="ui.empty()">
      <div :class="ui.emptyIconWrap()">
        <LightningIcon />
      </div>
      <p :class="ui.emptyTitle()">No commands added</p>
      <p :class="ui.emptyHint()">Add commands for quick access</p>
    </div>

    <div v-else :class="ui.list()">
      <template v-for="(command, index) in commands" :key="index">
        <CommandItem
          v-if="!command.submenu || command.submenu.length === 0"
          :command="command"
          :index="index"
          :input-keys="inputKeys"
          :level="0"
          :parent-commands="commands"
          :configId="configId"
          @update:command="(cmd: Command) => updateCommand(index, cmd)"
          @remove="removeCommand"
          @move="moveCommand"
          @add-input="addInput"
          @remove-input="removeInput"
          @update-input-key="updateInputKey"
          @add-multiple-command="addMultipleCommand"
          @remove-multiple-command="removeMultipleCommand"
        />

        <CommandSubmenu
          v-else
          :command="command"
          :index="index"
          :level="0"
          :parent-commands="commands"
          @update:command="(cmd: Command) => updateCommand(index, cmd)"
          @remove="removeCommand"
          @move="moveCommand"
        />
      </template>
    </div>

    <div :class="ui.footer()">
      <div>
        <h4 :class="ui.footerTitle()">Add New Command</h4>
        <p :class="ui.footerHint()">
          Create additional commands for your configuration
        </p>
      </div>
      <div :class="ui.actions()">
        <CustomButton variant="primary" size="sm" @click="showTemplatesModal">
          <TemplatesIcon :class="ui.btnIcon()" />
          Templates
        </CustomButton>
        <CustomButton variant="secondary" size="sm" @click="addCommand">
          <AddIcon :class="ui.btnIcon()" />
          Add Command
        </CustomButton>
        <CustomButton variant="secondary" size="sm" @click="addGroup">
          <AddIcon :class="ui.btnIcon()" />
          Add Group
        </CustomButton>
      </div>
    </div>

    <TemplateCommandsModal
      :is-open="isTemplatesModalOpen"
      @close="closeTemplatesModal"
      @commands-selected="handleCommandsSelected"
    />
  </div>
</template>

<script lang="ts" setup>
import { computed, ref, watch } from 'vue';
import { PropType } from 'vue';
import { tv } from '@/lib/tv';
import { Command } from '../../types';
import CustomButton from '../ui/CustomButton.vue';
import CommandItem from './CommandItem.vue';
import CommandSubmenu from './CommandSubmenu.vue';
import TemplateCommandsModal from '../modals/TemplateCommandsModal.vue';
import TemplatesIcon from '../icons/TemplatesIcon.vue';
import AddIcon from '../icons/AddIcon.vue';
import LightningIcon from '../icons/LightningIcon.vue';

defineOptions({ name: 'CommandsTable' });

const props = defineProps({
  commands: {
    type: Array as PropType<Command[]>,
    required: true,
  },
  configId: {
    type: String,
    default: '',
  },
});

const emit = defineEmits<{
  (e: 'update:commands', value: Command[]): void;
}>();

const ui = tv({
  slots: {
    root: 'space-y-3',
    header: 'flex items-center justify-between gap-2',
    title: 'text-base font-semibold text-slate-900',
    actions: 'flex items-center gap-1.5',
    empty: 'text-center py-6',
    emptyIconWrap:
      'w-12 h-12 bg-slate-100 rounded-md flex items-center justify-center mx-auto mb-3',
    emptyTitle: 'text-sm text-slate-500 mb-1',
    emptyHint: 'text-xs text-slate-400',
    list: 'space-y-3',
    footer: [
      'flex items-center justify-between gap-3 pt-3 pb-1.5 border-t border-slate-200',
      'bg-slate-50/50 rounded-md px-3 -mx-1',
    ],
    footerTitle: 'text-sm font-semibold text-slate-800',
    footerHint: 'text-xs text-slate-500 mt-0.5',
    btnIcon: 'mr-1.5',
  },
})();

const commands = computed({
  get: () => props.commands,
  set: value => emit('update:commands', value),
});

// Track input keys for UI
const inputKeys = ref<Record<number, Record<string, string>>>({});
const isTemplatesModalOpen = ref(false);

// Initialize input keys
watch(
  () => props.commands,
  newCommands => {
    inputKeys.value = {};
    newCommands.forEach((cmd, index) => {
      if (cmd.inputs) {
        inputKeys.value[index] = { ...cmd.inputs };
      }
    });
  },
  { immediate: true }
);

const addCommand = () => {
  const newCommand: Command = {
    name: '',
    command: undefined,
    hotkey: undefined,
    submenu: null,
    commands: [''], // По умолчанию создаем команду с commands
    inputs: null,
  };
  commands.value.push(newCommand);
};

const addGroup = () => {
  const newGroup: Command = {
    name: 'Group',
    command: undefined,
    hotkey: undefined,
    submenu: [
      {
        name: 'Command',
        command: undefined,
        hotkey: undefined,
        submenu: null,
        commands: [''],
        inputs: null,
      },
    ],
    commands: null,
    inputs: null,
  };
  commands.value.push(newGroup);
};

const updateCommand = (index: number, command: Command) => {
  commands.value[index] = command;
};

const removeCommand = (index: number) => {
  commands.value.splice(index, 1);
  delete inputKeys.value[index];
};

const moveCommand = (index: number, direction: number) => {
  const newIndex = index + direction;
  if (newIndex >= 0 && newIndex < commands.value.length) {
    const command = commands.value[index];
    commands.value.splice(index, 1);
    commands.value.splice(newIndex, 0, command);
  }
};

const addInput = (commandIndex: number) => {
  const command = commands.value[commandIndex];
  if (!command.inputs) {
    command.inputs = {};
  }
  if (!inputKeys.value[commandIndex]) {
    inputKeys.value[commandIndex] = {};
  }

  const key = `key${Object.keys(command.inputs).length + 1}`;
  command.inputs[key] = '';
  inputKeys.value[commandIndex][key] = key;
};

const removeInput = (commandIndex: number, key: string) => {
  const command = commands.value[commandIndex];
  if (command.inputs) {
    delete command.inputs[key];
    delete inputKeys.value[commandIndex][key];
  }
};

const updateInputKey = (
  commandIndex: number,
  oldKey: string,
  newKey: string
) => {
  const command = commands.value[commandIndex];
  if (command.inputs && command.inputs[oldKey] !== undefined) {
    const value = command.inputs[oldKey];
    delete command.inputs[oldKey];
    command.inputs[newKey] = value;
    delete inputKeys.value[commandIndex][oldKey];
    inputKeys.value[commandIndex][newKey] = newKey;
  }
};

const addMultipleCommand = (commandIndex: number) => {
  const command = commands.value[commandIndex];
  if (!command.commands) {
    command.commands = [];
  }
  command.commands.push('');
};

const removeMultipleCommand = (commandIndex: number, cmdIndex: number) => {
  const command = commands.value[commandIndex];
  if (command.commands && command.commands.length > 1) {
    command.commands.splice(cmdIndex, 1);
  }
};

const showTemplatesModal = () => {
  isTemplatesModalOpen.value = true;
};

const closeTemplatesModal = () => {
  isTemplatesModalOpen.value = false;
};

const handleCommandsSelected = (selectedCommands: Command[]) => {
  // Add selected commands to current commands list
  commands.value.push(...selectedCommands);
  closeTemplatesModal();
};
</script>
