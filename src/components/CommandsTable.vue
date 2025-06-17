<!-- src/components/CommandsTable.vue -->
<template>
  <div class="space-y-4">
    <div class="flex items-center justify-between">
      <h3 class="text-lg font-semibold text-slate-900">Commands</h3>
      <Button @click="addCommand" variant="secondary" size="sm">
        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6" />
        </svg>
        Add Command
      </Button>
    </div>
    
    <div v-if="commands.length === 0" class="text-center py-8">
      <div class="w-16 h-16 bg-slate-100 flex items-center justify-center mx-auto mb-4">
        <svg class="w-8 h-8 text-slate-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
        </svg>
      </div>
      <p class="text-slate-500 mb-2">No commands added</p>
      <p class="text-sm text-slate-400">Add commands for quick access</p>
    </div>
    
    <div v-else class="space-y-6">
      <CommandItem
        v-for="(command, index) in commands"
        :key="index"
        :command="command"
        :index="index"
        :input-keys="inputKeys"
        :level="0"
        :parent-commands="commands"
        @update:command="updateCommand"
        @remove="removeCommand"
        @move="moveCommand"
        @update-command-type="updateCommandType"
        @add-input="addInput"
        @remove-input="removeInput"
        @update-input-key="updateInputKey"
        @add-multiple-command="addMultipleCommand"
        @remove-multiple-command="removeMultipleCommand"
        @add-submenu-command="addSubmenuCommand"
        @remove-submenu-command="removeSubmenuCommand"
      />
    </div>
    
    <!-- Add Command Button at Bottom -->
    <div class="flex items-center justify-between pt-6 pb-2 border-t border-slate-200 bg-slate-50/50 rounded-lg px-4 -mx-4">
      <div>
        <h4 class="text-sm font-semibold text-slate-800">Add New Command</h4>
        <p class="text-xs text-slate-500 mt-0.5">Create additional commands for your configuration</p>
      </div>
      <Button @click="addCommand" variant="secondary" size="sm" class="shadow-sm">
        <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6" />
        </svg>
        Add Command
      </Button>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { computed, ref, watch } from 'vue';
import { PropType } from 'vue';
import { Command } from '../types';
import Input from './Input.vue';
import Button from './Button.vue';
import HotkeyInput from './HotkeyInput.vue';
import CommandItem from './CommandItem.vue';

const props = defineProps({
  commands: {
    type: Array as PropType<Command[]>,
    required: true
  }
});

const emit = defineEmits<{
  (e: 'update:commands', value: Command[]): void;
}>();

const commands = computed({
  get: () => props.commands,
  set: (value) => emit('update:commands', value)
});

// Track input keys for UI
const inputKeys = ref<Record<number, Record<string, string>>>({});

// Initialize input keys
watch(() => props.commands, (newCommands) => {
  inputKeys.value = {};
  newCommands.forEach((cmd, index) => {
    if (cmd.inputs) {
      inputKeys.value[index] = { ...cmd.inputs };
    }
  });
}, { immediate: true });

const addCommand = () => {
  const newCommand: Command = {
    name: '',
    command: undefined,
    hotkey: undefined,
    submenu: null,
    commands: null,
    inputs: null
  };
  commands.value.push(newCommand);
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

const updateCommandType = (index: number, type: string) => {
  const command = commands.value[index];
  
  if (type === 'single') {
    command.submenu = null;
    command.commands = null;
    command.inputs = null;
  } else if (type === 'multiple') {
    command.command = undefined;
    command.submenu = null;
    command.commands = command.commands || [];
    command.inputs = command.inputs || {};
  } else if (type === 'submenu') {
    command.command = undefined;
    command.commands = null;
    command.inputs = null;
    command.submenu = command.submenu || [];
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

const updateInputKey = (commandIndex: number, oldKey: string, newKey: string) => {
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
  if (command.commands) {
    command.commands.splice(cmdIndex, 1);
  }
};

const addSubmenuCommand = (commandIndex: number) => {
  const command = commands.value[commandIndex];
  if (!command.submenu) {
    command.submenu = [];
  }
  command.submenu.push({
    name: '',
    command: undefined,
    hotkey: undefined,
    submenu: null,
    commands: null,
    inputs: null
  });
};

const removeSubmenuCommand = (commandIndex: number, subIndex: number) => {
  const command = commands.value[commandIndex];
  if (command.submenu) {
    command.submenu.splice(subIndex, 1);
  }
};
</script>