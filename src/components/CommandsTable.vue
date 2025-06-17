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
    
    <div v-else class="space-y-4">
      <div
        v-for="(command, index) in commands"
        :key="index"
        class="border border-slate-200 rounded-lg p-4 space-y-4"
      >
        <!-- Command Header -->
        <div class="flex items-center justify-between">
          <div class="flex items-center space-x-2">
            <svg class="w-5 h-5 text-slate-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
            </svg>
            <h4 class="font-medium text-slate-900">Command {{ index + 1 }}</h4>
          </div>
          <div class="flex items-center space-x-2">
            <Button
              @click="moveCommand(index, -1)"
              variant="ghost"
              size="sm"
              :disabled="index === 0"
            >
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 15l7-7 7 7" />
              </svg>
            </Button>
            <Button
              @click="moveCommand(index, 1)"
              variant="ghost"
              size="sm"
              :disabled="index === commands.length - 1"
            >
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
              </svg>
            </Button>
            <Button
              @click="removeCommand(index)"
              variant="danger"
              size="sm"
            >
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
              </svg>
            </Button>
          </div>
        </div>

        <!-- Basic Command Fields -->
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <Input
            v-model="command.name"
            label="Name"
            placeholder="Command name"
            required
          />
          
          <HotkeyInput
            v-model="command.hotkey"
            label="Hotkey"
            placeholder="Click to record"
          />
        </div>

        <!-- Command Type Selection -->
        <div class="space-y-3">
          <label class="block text-sm font-medium text-slate-700">Command Type</label>
          <div class="flex space-x-4">
            <label class="flex items-center">
              <input
                type="radio"
                :value="'single'"
                v-model="commandTypes[index]"
                class="mr-2"
                @change="updateCommandType(index, 'single')"
              />
              <span class="text-sm">Single Command</span>
            </label>
            <label class="flex items-center">
              <input
                type="radio"
                :value="'multiple'"
                v-model="commandTypes[index]"
                class="mr-2"
                @change="updateCommandType(index, 'multiple')"
              />
              <span class="text-sm">Multiple Commands</span>
            </label>
            <label class="flex items-center">
              <input
                type="radio"
                :value="'submenu'"
                v-model="commandTypes[index]"
                class="mr-2"
                @change="updateCommandType(index, 'submenu')"
              />
              <span class="text-sm">Submenu</span>
            </label>
          </div>
        </div>

        <!-- Single Command -->
        <div v-if="commandTypes[index] === 'single'" class="space-y-4">
          <Input
            v-model="command.command"
            label="Command"
            placeholder="Enter command to execute"
            type="textarea"
            rows="3"
          />
          
          <!-- Inputs Section -->
          <div class="space-y-3">
            <div class="flex items-center justify-between">
              <label class="block text-sm font-medium text-slate-700">Inputs</label>
              <Button @click="addInput(index)" variant="ghost" size="sm">
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6" />
                </svg>
                Add Field
              </Button>
            </div>
            
            <div v-if="command.inputs && Object.keys(command.inputs).length > 0" class="space-y-2">
              <div
                v-for="(_, key) in command.inputs"
                :key="key"
                class="flex items-center space-x-2"
              >
                <Input
                  v-model="inputKeys[index][key]"
                  placeholder="Key"
                  size="sm"
                  @input="updateInputKey(index, key, $event)"
                />
                <Input
                  v-model="command.inputs[key]"
                  placeholder="Default value"
                  size="sm"
                />
                <Button @click="removeInput(index, key)" variant="danger" size="sm">
                  <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                  </svg>
                </Button>
              </div>
            </div>
          </div>
        </div>

        <!-- Multiple Commands -->
        <div v-if="commandTypes[index] === 'multiple'" class="space-y-4">
          <div class="space-y-3">
            <div class="flex items-center justify-between">
              <label class="block text-sm font-medium text-slate-700">Commands</label>
              <Button @click="addMultipleCommand(index)" variant="ghost" size="sm">
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6" />
                </svg>
                Add Command
              </Button>
            </div>
            
            <div v-if="command.commands && command.commands.length > 0" class="space-y-2">
              <div
                v-for="(_, cmdIndex) in command.commands"
                :key="cmdIndex"
                class="flex items-center space-x-2"
              >
                <Input
                  v-model="command.commands[cmdIndex]"
                  placeholder="Enter command"
                  size="sm"
                />
                <Button @click="removeMultipleCommand(index, cmdIndex)" variant="danger" size="sm">
                  <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                  </svg>
                </Button>
              </div>
            </div>
          </div>
          
          <!-- Inputs for multiple commands -->
          <div class="space-y-3">
            <div class="flex items-center justify-between">
              <label class="block text-sm font-medium text-slate-700">Inputs</label>
              <Button @click="addInput(index)" variant="ghost" size="sm">
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6" />
                </svg>
                Add Field
              </Button>
            </div>
            
            <div v-if="command.inputs && Object.keys(command.inputs).length > 0" class="space-y-2">
              <div
                v-for="(_, key) in command.inputs"
                :key="key"
                class="flex items-center space-x-2"
              >
                <Input
                  v-model="inputKeys[index][key]"
                  placeholder="Key"
                  size="sm"
                  @input="updateInputKey(index, key, $event)"
                />
                <Input
                  v-model="command.inputs[key]"
                  placeholder="Default value"
                  size="sm"
                />
                <Button @click="removeInput(index, key)" variant="danger" size="sm">
                  <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                  </svg>
                </Button>
              </div>
            </div>
          </div>
        </div>

        <!-- Submenu -->
        <div v-if="commandTypes[index] === 'submenu'" class="space-y-4">
          <div class="space-y-3">
            <div class="flex items-center justify-between">
              <label class="block text-sm font-medium text-slate-700">Submenu</label>
              <Button @click="addSubmenuCommand(index)" variant="ghost" size="sm">
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6" />
                </svg>
                Add Submenu Command
              </Button>
            </div>
            
            <div v-if="command.submenu && command.submenu.length > 0" class="space-y-2">
              <div
                v-for="(subCmd, subIndex) in command.submenu"
                :key="subIndex"
                class="border border-slate-200 rounded p-3 space-y-2"
              >
                <div class="flex items-center justify-between">
                  <h5 class="font-medium text-sm">Subcommand {{ subIndex + 1 }}</h5>
                  <Button @click="removeSubmenuCommand(index, subIndex)" variant="danger" size="sm">
                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                    </svg>
                  </Button>
                </div>
                
                <div class="grid grid-cols-1 md:grid-cols-2 gap-2">
                  <Input
                    v-model="subCmd.name"
                    placeholder="Name"
                    size="sm"
                  />
                  <HotkeyInput
                    v-model="subCmd.hotkey"
                    placeholder="Hotkey"
                    size="sm"
                  />
                </div>
                
                <Input
                  v-model="subCmd.command"
                  placeholder="Command"
                  size="sm"
                />
              </div>
            </div>
          </div>
        </div>
      </div>
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

// Track command types for UI
const commandTypes = ref<string[]>([]);
const inputKeys = ref<Record<number, Record<string, string>>>({});

// Initialize command types
watch(() => props.commands, (newCommands) => {
  commandTypes.value = newCommands.map(cmd => {
    if (cmd.submenu && cmd.submenu.length > 0) return 'submenu';
    if (cmd.commands && cmd.commands.length > 0) return 'multiple';
    return 'single';
  });
  
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
  commandTypes.value.push('single');
};

const removeCommand = (index: number) => {
  commands.value.splice(index, 1);
  commandTypes.value.splice(index, 1);
  delete inputKeys.value[index];
};

const moveCommand = (index: number, direction: number) => {
  const newIndex = index + direction;
  if (newIndex >= 0 && newIndex < commands.value.length) {
    const command = commands.value[index];
    const type = commandTypes.value[index];
    commands.value.splice(index, 1);
    commandTypes.value.splice(index, 1);
    commands.value.splice(newIndex, 0, command);
    commandTypes.value.splice(newIndex, 0, type);
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