<template>
  <div 
    class="border border-slate-200 rounded-xl p-6 space-y-6 bg-white shadow-md hover:shadow-lg transition-shadow duration-200"
  >
    <!-- Command Header -->
    <div class="flex items-center justify-between pb-4 border-b border-slate-200 -mx-6 px-6">
      <div class="flex items-center space-x-3">
        <div class="w-8 h-8 bg-blue-100 flex items-center justify-center rounded-lg">
          <svg class="w-4 h-4 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
          </svg>
        </div>
        <h4 class="font-semibold text-slate-900">
          {{ commandType === 'submenu' ? `Submenu ${index + 1}` : `Command ${index + 1}` }}
        </h4>
      </div>
      <div class="flex items-center space-x-1">
        <Button
          @click="$emit('move', index, -1)"
          variant="ghost"
          size="sm"
          :disabled="index === 0"
          class="text-slate-500 hover:text-slate-700"
        >
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 15l7-7 7 7" />
          </svg>
        </Button>
        <Button
          @click="$emit('move', index, 1)"
          variant="ghost"
          size="sm"
          :disabled="index === (parentCommands?.length || 0) - 1"
          class="text-slate-500 hover:text-slate-700"
        >
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
          </svg>
        </Button>
        <Button
          @click="$emit('remove', index)"
          variant="danger"
          size="sm"
          class="text-red-600 hover:text-red-700 hover:bg-red-50"
        >
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
          </svg>
        </Button>
      </div>
    </div>

    <!-- Basic Command Fields -->
    <div :class="[
      'grid gap-6',
      commandType === 'submenu' ? 'grid-cols-1' : 'grid-cols-1 md:grid-cols-2'
    ]">
      <div class="flex items-end gap-3">
        <div class="w-16">
          <Input
            v-model="commandIcon"
            label="Icon"
            placeholder="emoji"
            size="sm"
            @input="handleIconChange"
          />
        </div>
        <div class="flex-1">
          <Input
            v-model="command.name"
            label="Name"
            placeholder="Command name"
            required
          />
        </div>
      </div>
      
      <HotkeyInput
        v-if="commandType !== 'submenu'"
        v-model="command.hotkey"
        label="Hotkey"
        placeholder="Click to record"
      />
    </div>

    <!-- Command Type Selection -->
    <div class="space-y-4">
      <CommandTypeSelector 
        v-model="commandType" 
        @update:modelValue="handleCommandTypeChange"
      />
    </div>

    <!-- Divider after Command Type -->
    <div class="border-t-2 border-slate-200/70 my-8 -mx-6"></div>

    <!-- Single Command -->
    <div v-if="commandType === 'single'" class="space-y-6">
      <Input
        v-model="command.command"
        label="Command"
        placeholder="Enter command to execute"
        type="textarea"
        rows="3"
      />
      
      <!-- Divider -->
      <div class="border-t-2 border-slate-200/70 my-8 -mx-6"></div>
      
      <!-- Inputs Section -->
      <div class="space-y-4">
        <div class="flex items-center justify-between">
          <label class="block text-sm font-semibold text-slate-700">Inputs</label>
          <Button @click="handleAddInput" variant="ghost" size="sm" class="text-blue-600 hover:text-blue-700 hover:bg-blue-50">
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6" />
            </svg>
            Add Field
          </Button>
        </div>
        
        <div v-if="command.inputs && Object.keys(command.inputs).length > 0" class="space-y-2">
          <!-- Table Headers -->
          <div class="flex items-center gap-2 py-1 px-1 rounded-lg">
            <div class="flex-1">
              <span class="text-xs font-semibold text-slate-700 uppercase tracking-wide">Key</span>
            </div>
            <div class="flex-1">
              <span class="text-xs font-semibold text-slate-700 uppercase tracking-wide">Default Value</span>
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
              <Input
                :model-value="isRootLevel && inputKeys[index] ? inputKeys[index][key] : key"
                placeholder="Key"
                size="sm"
                input-class="border border-slate-300 bg-white rounded px-2 py-1 focus:border-blue-400 focus:ring-0"
                @input="handleInputKeyChange(key, $event)"
              />
            </div>
            <div class="flex-1">
              <Input
                v-model="command.inputs[key]"
                placeholder="Default value"
                size="sm"
                input-class="border border-slate-300 bg-white rounded px-2 py-1 focus:border-blue-400 focus:ring-0"
              />
            </div>
            <Button @click="handleRemoveInput(key)" variant="danger" size="sm" class="text-red-600 hover:text-red-700 hover:bg-red-50 flex-shrink-0 w-8">
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
              </svg>
            </Button>
          </div>
        </div>
      </div>
    </div>

    <!-- Multiple Commands -->
    <div v-if="commandType === 'multiple'" class="space-y-6">
      <div class="space-y-4">
        <div class="flex items-center justify-between">
          <label class="block text-sm font-semibold text-slate-700">Commands</label>
          <Button @click="handleAddMultipleCommand" variant="ghost" size="sm" class="text-blue-600 hover:text-blue-700 hover:bg-blue-50">
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
            class="flex items-center gap-2 py-1"
          >
            <div class="flex-1">
              <Input
                v-model="command.commands[cmdIndex]"
                placeholder="Enter command"
                size="sm"
                input-class="border border-slate-300 bg-white rounded px-2 py-1 focus:border-blue-400 focus:ring-0"
              />
            </div>
            <Button @click="handleRemoveMultipleCommand(cmdIndex)" variant="danger" size="sm" class="text-red-600 hover:text-red-700 hover:bg-red-50">
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
              </svg>
            </Button>
          </div>
        </div>
      </div>
      
      <!-- Divider -->
      <div class="border-t-2 border-slate-200/70 my-8 -mx-6"></div>
      
      <!-- Inputs for multiple commands -->
      <div class="space-y-4">
        <div class="flex items-center justify-between">
          <label class="block text-sm font-semibold text-slate-700">Inputs</label>
          <Button @click="handleAddInput" variant="ghost" size="sm" class="text-blue-600 hover:text-blue-700 hover:bg-blue-50">
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6" />
            </svg>
            Add Field
          </Button>
        </div>
        
        <div v-if="command.inputs && Object.keys(command.inputs).length > 0" class="space-y-2">
          <!-- Table Headers -->
          <div class="flex items-center gap-2 py-1 px-1 rounded-lg">
            <div class="flex-1">
              <span class="text-xs font-semibold text-slate-700 uppercase tracking-wide">Key</span>
            </div>
            <div class="flex-1">
              <span class="text-xs font-semibold text-slate-700 uppercase tracking-wide">Default Value</span>
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
              <Input
                :model-value="isRootLevel && inputKeys[index] ? inputKeys[index][key] : key"
                placeholder="Key"
                size="sm"
                input-class="border border-slate-300 bg-white rounded px-2 py-1 focus:border-blue-400 focus:ring-0"
                @input="handleInputKeyChange(key, $event)"
              />
            </div>
            <div class="flex-1">
              <Input
                v-model="command.inputs[key]"
                placeholder="Default value"
                size="sm"
                input-class="border border-slate-300 bg-white rounded px-2 py-1 focus:border-blue-400 focus:ring-0"
              />
            </div>
            <Button @click="handleRemoveInput(key)" variant="danger" size="sm" class="text-red-600 hover:text-red-700 hover:bg-red-50 flex-shrink-0 w-8">
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
              </svg>
            </Button>
          </div>
        </div>
      </div>
    </div>

    <!-- Switch -->
    <div v-if="commandType === 'switch'" class="space-y-6">
      <Input
        v-model="command.command"
        label="Toggle Command"
        placeholder="Command to execute when toggled"
        type="textarea"
        rows="3"
      />
      
      <Input
        v-model="command.switch"
        label="Switch Command"
        placeholder="Command to check state (e.g., echo 'true')"
        type="textarea"
        rows="3"
      />
      
      <!-- Divider -->
      <div class="border-t-2 border-slate-200/70 my-8 -mx-6"></div>
      
      <!-- Inputs Section -->
      <div class="space-y-4">
        <div class="flex items-center justify-between">
          <label class="block text-sm font-semibold text-slate-700">Inputs</label>
          <Button @click="handleAddInput" variant="ghost" size="sm" class="text-blue-600 hover:text-blue-700 hover:bg-blue-50">
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6" />
            </svg>
            Add Field
          </Button>
        </div>
        
        <div v-if="command.inputs && Object.keys(command.inputs).length > 0" class="space-y-2">
          <!-- Table Headers -->
          <div class="flex items-center gap-2 py-1 px-1 rounded-lg">
            <div class="flex-1">
              <span class="text-xs font-semibold text-slate-700 uppercase tracking-wide">Key</span>
            </div>
            <div class="flex-1">
              <span class="text-xs font-semibold text-slate-700 uppercase tracking-wide">Default Value</span>
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
              <Input
                :model-value="isRootLevel && inputKeys[index] ? inputKeys[index][key] : key"
                placeholder="Key"
                size="sm"
                input-class="border border-slate-300 bg-white rounded px-2 py-1 focus:border-blue-400 focus:ring-0"
                @input="handleInputKeyChange(key, $event)"
              />
            </div>
            <div class="flex-1">
              <Input
                v-model="command.inputs[key]"
                placeholder="Default value"
                size="sm"
                input-class="border border-slate-300 bg-white rounded px-2 py-1 focus:border-blue-400 focus:ring-0"
              />
            </div>
            <Button @click="handleRemoveInput(key)" variant="danger" size="sm" class="text-red-600 hover:text-red-700 hover:bg-red-50 flex-shrink-0 w-8">
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
              </svg>
            </Button>
          </div>
        </div>
      </div>
    </div>

    <!-- Monitor Command -->
    <div v-if="commandType === 'monitor'" class="space-y-6">
      <Input
        v-model="command.command"
        label="Monitor Command"
        placeholder="Command to execute for monitoring (e.g., top -l 1 | grep 'CPU usage')"
        type="textarea"
        rows="3"
      />
      
      <Input
        v-model="command.monitor"
        label="Monitor Display Command"
        placeholder="Command to get display value (e.g., echo 'CPU: 45%')"
        type="textarea"
        rows="3"
      />
      
      <!-- Divider -->
      <div class="border-t-2 border-slate-200/70 my-8 -mx-6"></div>
      
      <!-- Inputs Section -->
      <div class="space-y-4">
        <div class="flex items-center justify-between">
          <label class="block text-sm font-semibold text-slate-700">Inputs</label>
          <Button @click="handleAddInput" variant="ghost" size="sm" class="text-blue-600 hover:text-blue-700 hover:bg-blue-50">
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6" />
            </svg>
            Add Field
          </Button>
        </div>
        
        <div v-if="command.inputs && Object.keys(command.inputs).length > 0" class="space-y-2">
          <!-- Table Headers -->
          <div class="flex items-center gap-2 py-1 px-1 rounded-lg">
            <div class="flex-1">
              <span class="text-xs font-semibold text-slate-700 uppercase tracking-wide">Key</span>
            </div>
            <div class="flex-1">
              <span class="text-xs font-semibold text-slate-700 uppercase tracking-wide">Default Value</span>
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
              <Input
                :model-value="isRootLevel && inputKeys[index] ? inputKeys[index][key] : key"
                placeholder="Key"
                size="sm"
                input-class="border border-slate-300 bg-white rounded px-2 py-1 focus:border-blue-400 focus:ring-0"
                @input="handleInputKeyChange(key, $event)"
              />
            </div>
            <div class="flex-1">
              <Input
                v-model="command.inputs[key]"
                placeholder="Default value"
                size="sm"
                input-class="border border-slate-300 bg-white rounded px-2 py-1 focus:border-blue-400 focus:ring-0"
              />
            </div>
            <Button @click="handleRemoveInput(key)" variant="danger" size="sm" class="text-red-600 hover:text-red-700 hover:bg-red-50 flex-shrink-0 w-8">
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
              </svg>
            </Button>
          </div>
        </div>
      </div>
    </div>

    <!-- Submenu -->
    <div v-if="commandType === 'submenu'" class="space-y-4">
      <div class="space-y-4">
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-2">
            <button @click="toggleSubmenu" type="button" class="focus:outline-none">
              <svg :class="['w-5 h-5 transition-transform', submenuCollapsed ? 'rotate-[-90deg]' : 'rotate-0']" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
              </svg>
            </button>
            <label class="block text-sm font-semibold text-slate-700">Submenu</label>
          </div>
          <Button @click="addSubmenuCommand" variant="ghost" size="sm" class="text-blue-600 hover:text-blue-700 hover:bg-blue-50">
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6" />
            </svg>
            Add Submenu Command
          </Button>
        </div>
        <div v-if="command.submenu && command.submenu.length > 0 && !submenuCollapsed" class="space-y-4">
          <CommandItem
            v-for="(subCmd, subIndex) in command.submenu"
            :key="subIndex"
            :command="subCmd"
            :index="subIndex"
            :input-keys="{}"
            :level="level + 1"
            :parent-commands="command.submenu"
            @update:command="updateSubmenuCommand"
            @remove="removeSubmenuCommand"
            @move="moveSubmenuCommand"
            @update-command-type="updateSubmenuCommandType"
            @add-input="addSubmenuInput"
            @remove-input="removeSubmenuInput"
            @update-input-key="updateSubmenuInputKey"
            @add-multiple-command="addSubmenuMultipleCommand"
            @remove-multiple-command="removeSubmenuMultipleCommand"
            @add-submenu-command="addSubmenuSubmenuCommand"
            @remove-submenu-command="removeSubmenuSubmenuCommand"
          />
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { computed, ref, onMounted } from 'vue';
import { PropType } from 'vue';
import { Command } from '../types';
import Input from './Input.vue';
import Button from './Button.vue';
import HotkeyInput from './HotkeyInput.vue';
import CommandTypeSelector from './CommandTypeSelector.vue';

const props = defineProps({
  command: {
    type: Object as PropType<Command>,
    required: true
  },
  index: {
    type: Number,
    required: true
  },
  inputKeys: {
    type: Object as PropType<Record<number, Record<string, string>>>,
    required: true
  },
  level: {
    type: Number,
    default: 0
  },
  parentCommands: {
    type: Array as PropType<Command[]>,
    default: null
  }
});

const emit = defineEmits<{
  (e: 'update:command', value: Command): void;
  (e: 'remove', index: number): void;
  (e: 'move', index: number, direction: number): void;
  (e: 'update-command-type', index: number, type: string): void;
  (e: 'add-input', index: number): void;
  (e: 'remove-input', index: number, key: string): void;
  (e: 'update-input-key', index: number, oldKey: string, newKey: string): void;
  (e: 'add-multiple-command', index: number): void;
  (e: 'remove-multiple-command', index: number, cmdIndex: number): void;
  (e: 'add-submenu-command', index: number): void;
  (e: 'remove-submenu-command', index: number, subIndex: number): void;
}>();

// Current command type - простой ref
const commandType = ref<string>('single');

// Инициализируем тип при создании компонента
const initializeCommandType = () => {
  const cmd = props.command;
  
  // Проверяем наличие monitor (только если это не null и не пустая строка)
  if (cmd.monitor !== undefined && cmd.monitor !== null && cmd.monitor !== '') {
    commandType.value = 'monitor';
    return;
  }
  // Проверяем наличие switch (только если это не null и не пустая строка)
  else if (cmd.switch !== undefined && cmd.switch !== null && cmd.switch !== '') {
    commandType.value = 'switch';
    return;
  }
  // Проверяем наличие submenu с элементами
  else if (cmd.submenu && Array.isArray(cmd.submenu) && cmd.submenu.length > 0) {
    commandType.value = 'submenu';
    return;
  }
  // Проверяем наличие commands с элементами
  else if (cmd.commands && Array.isArray(cmd.commands) && cmd.commands.length > 0) {
    commandType.value = 'multiple';
    return;
  }
  // По умолчанию single
  else {
    commandType.value = 'single';
  }
};

// Инициализируем при монтировании компонента
onMounted(() => {
  initializeCommandType();
});

// Обработчик изменения типа команды
const handleCommandTypeChange = (type: string) => {
  commandType.value = type;
  
  if (isRootLevel.value) {
    emit('update-command-type', props.index, type);
  } else {
    updateSubmenuCommandType(type);
  }
};

// Computed properties for determining logic
const isRootLevel = computed(() => props.level === 0);
// const isSubmenuLevel = computed(() => props.level > 0);

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
  }
});

// Method for handling icon changes
const handleIconChange = (event: Event) => {
  const value = (event.target as HTMLInputElement).value;
  if (value.trim() === '') {
    props.command.icon = null;
  } else {
    props.command.icon = value;
  }
  emit('update:command', props.command);
};

// Functions for handling inputs
const handleAddInput = () => {
  if (isRootLevel.value) {
    emit('add-input', props.index);
  } else {
    addSubmenuInput();
  }
};

const handleRemoveInput = (key: string) => {
  if (isRootLevel.value) {
    emit('remove-input', props.index, key);
  } else {
    removeSubmenuInput(key);
  }
};

const handleInputKeyChange = (key: string, event: Event) => {
  const value = (event.target as HTMLInputElement).value;
  if (isRootLevel.value) {
    emit('update-input-key', props.index, key, value);
  } else {
    updateSubmenuInputKey(key, value);
  }
};

// Functions for handling multiple commands
const handleAddMultipleCommand = () => {
  if (isRootLevel.value) {
    emit('add-multiple-command', props.index);
  } else {
    addSubmenuMultipleCommand();
  }
};

const handleRemoveMultipleCommand = (cmdIndex: number) => {
  if (isRootLevel.value) {
    emit('remove-multiple-command', props.index, cmdIndex);
  } else {
    removeSubmenuMultipleCommand(cmdIndex);
  }
};

const updateSubmenuCommand = (subIndex: number, subCommand: Command) => {
  if (props.command.submenu) {
    props.command.submenu[subIndex] = subCommand;
    emit('update:command', props.command);
  }
};

const removeSubmenuCommand = (subIndex: number) => {
  if (props.command.submenu) {
    props.command.submenu.splice(subIndex, 1);
    emit('update:command', props.command);
  }
};

const moveSubmenuCommand = (subIndex: number, direction: number) => {
  if (props.command.submenu) {
    const newIndex = subIndex + direction;
    if (newIndex >= 0 && newIndex < props.command.submenu.length) {
      const subCommand = props.command.submenu[subIndex];
      props.command.submenu.splice(subIndex, 1);
      props.command.submenu.splice(newIndex, 0, subCommand);
      emit('update:command', props.command);
    }
  }
};

const updateSubmenuCommandType = (type: string) => {
  commandType.value = type;
  
  const cmd = props.command;
  if (cmd) {
    if (type === 'single') {
      cmd.submenu = null;
      cmd.commands = null;
      cmd.inputs = null;
      cmd.switch = undefined;
      cmd.monitor = undefined;
      // Не очищаем command, так как это основное поле для single команды
    } else if (type === 'multiple') {
      cmd.command = undefined;
      cmd.submenu = null;
      cmd.switch = undefined;
      cmd.monitor = undefined;
      cmd.commands = cmd.commands || [];
      cmd.inputs = cmd.inputs || {};
    } else if (type === 'submenu') {
      cmd.command = undefined;
      cmd.commands = null;
      cmd.inputs = null;
      cmd.switch = undefined;
      cmd.monitor = undefined;
      cmd.submenu = cmd.submenu || [];
    } else if (type === 'switch') {
      cmd.submenu = null;
      cmd.commands = null;
      cmd.inputs = cmd.inputs || {};
      cmd.switch = cmd.switch || '';
      cmd.monitor = undefined;
      // Не очищаем command, так как это поле для toggle команды
    } else if (type === 'monitor') {
      cmd.submenu = null;
      cmd.commands = null;
      cmd.inputs = cmd.inputs || {};
      cmd.monitor = cmd.monitor || '';
      cmd.switch = undefined;
      // Не очищаем command, так как это поле для команды мониторинга
    }
    emit('update:command', props.command);
  }
};

const addSubmenuCommand = () => {
  if (!props.command.submenu) {
    props.command.submenu = [];
  }
  props.command.submenu.push({
    name: '',
    command: undefined,
    hotkey: undefined,
    submenu: null,
    commands: null,
    inputs: null
  });
  emit('update:command', props.command);
};

const addSubmenuInput = () => {
  const cmd = props.command;
  if (cmd) {
    if (!cmd.inputs) {
      cmd.inputs = {};
    }
    
    const key = `key${Object.keys(cmd.inputs).length + 1}`;
    cmd.inputs[key] = '';
    emit('update:command', props.command);
  }
};

const removeSubmenuInput = (key: string) => {
  const cmd = props.command;
  if (cmd?.inputs) {
    delete cmd.inputs[key];
    emit('update:command', props.command);
  }
};

const updateSubmenuInputKey = (oldKey: string, newKey: string) => {
  const cmd = props.command;
  if (cmd?.inputs && cmd.inputs[oldKey] !== undefined) {
    const value = cmd.inputs[oldKey];
    delete cmd.inputs[oldKey];
    cmd.inputs[newKey] = value;
    emit('update:command', props.command);
  }
};

const addSubmenuMultipleCommand = () => {
  const cmd = props.command;
  if (cmd) {
    if (!cmd.commands) {
      cmd.commands = [];
    }
    cmd.commands.push('');
    emit('update:command', props.command);
  }
};

const removeSubmenuMultipleCommand = (cmdIndex: number) => {
  const cmd = props.command;
  if (cmd?.commands) {
    cmd.commands.splice(cmdIndex, 1);
    emit('update:command', props.command);
  }
};

const addSubmenuSubmenuCommand = (subIndex: number) => {
  const subCommand = props.command.submenu?.[subIndex];
  if (subCommand) {
    if (!subCommand.submenu) {
      subCommand.submenu = [];
    }
    subCommand.submenu.push({
      name: '',
      command: undefined,
      hotkey: undefined,
      submenu: null,
      commands: null,
      inputs: null
    });
    emit('update:command', props.command);
  }
};

const removeSubmenuSubmenuCommand = (subIndex: number, subSubIndex: number) => {
  const subCommand = props.command.submenu?.[subIndex];
  if (subCommand?.submenu) {
    subCommand.submenu.splice(subSubIndex, 1);
    emit('update:command', props.command);
  }
};

// Сворачивание секции Submenu
const submenuCollapsed = ref(false);
const toggleSubmenu = () => {
  submenuCollapsed.value = !submenuCollapsed.value;
};
</script> 