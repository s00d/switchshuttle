<template>
  <div
    class="border border-slate-200 rounded-xl p-6 space-y-6 bg-blue-50/50 shadow-md hover:shadow-lg transition-shadow duration-200"
  >
    <!-- Command Header -->
    <div
      class="flex items-center justify-between pb-4 border-b border-slate-200 -mx-6 px-6"
    >
      <div class="flex items-center space-x-3">
        <div
          class="w-8 h-8 bg-blue-100 flex items-center justify-center rounded-lg"
        >
          <FolderOpenIcon class="w-4 h-4 text-blue-600" />
        </div>
        <h4 class="font-semibold text-blue-900">
          Group {{ index + 1 }}{{ command.name ? ` - ${command.name}` : '' }}
        </h4>
      </div>
      <div class="flex items-center space-x-1">
        <CustomButton
          variant="ghost"
          size="sm"
          :disabled="index === 0"
          class="text-blue-600 hover:text-blue-700 hover:bg-blue-100"
          @click="$emit('move', index, -1)"
        >
          <ChevronUpIcon />
        </CustomButton>
        <CustomButton
          variant="ghost"
          size="sm"
          :disabled="index === (parentCommands?.length || 0) - 1"
          class="text-blue-600 hover:text-blue-700 hover:bg-blue-100"
          @click="$emit('move', index, 1)"
        >
          <ChevronDownIcon />
        </CustomButton>
        <CustomButton
          variant="danger"
          size="sm"
          class="text-red-600 hover:text-red-700 hover:bg-red-50"
          @click="$emit('remove', index)"
        >
          <TrashIcon />
        </CustomButton>
      </div>
    </div>

    <!-- Basic Command Fields -->
    <div class="grid gap-6 grid-cols-1">
      <div class="flex items-start gap-3">
        <div class="w-16">
          <IconSelector
            v-model="commandIcon"
            label="Icon"
            placeholder="emoji"
            input-class="bg-white h-10 px-2 py-1"
            @update:modelValue="handleIconChange"
          />
        </div>
        <div class="flex-1">
          <Input
            v-model="command.name"
            label="Name"
            placeholder="Group name"
            required
            input-class="bg-white h-10"
          />
        </div>
      </div>
    </div>

    <!-- Submenu -->
    <div class="space-y-4">
      <div class="space-y-4">
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-2">
            <CustomButton
              variant="ghost"
              class="focus:outline-none"
              @click="toggleSubmenu"
            >
              <ChevronRightIcon :collapsed="submenuCollapsed" />
            </CustomButton>
            <label class="block text-sm font-semibold text-blue-700"
              >Submenu</label
            >
          </div>
          <CustomButton
            variant="ghost"
            size="sm"
            class="text-blue-600 hover:text-blue-700 hover:bg-blue-50"
            @click="addSubmenuCommand"
          >
            <AddIcon />
            Add Submenu Command
          </CustomButton>
        </div>
        <div
          v-if="
            command.submenu && command.submenu.length > 0 && !submenuCollapsed
          "
          class="space-y-4"
        >
          <CommandItem
            v-for="(subCmd, subIndex) in command.submenu"
            :key="subIndex"
            :command="subCmd"
            :index="subIndex"
            :input-keys="{}"
            :level="level + 1"
            :parent-commands="command.submenu"
            @update:command="updateSubmenuCommand(subIndex, $event)"
            @remove="removeSubmenuCommand(subIndex)"
            @move="moveSubmenuCommand(subIndex, $event)"
            @add-input="() => addSubmenuInput()"
            @remove-input="(_index, key) => removeSubmenuInput(key)"
            @update-input-key="
              (_index, oldKey, newKey) => updateSubmenuInputKey(oldKey, newKey)
            "
            @add-multiple-command="addSubmenuMultipleCommand"
            @remove-multiple-command="removeSubmenuMultipleCommand"
          />
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { computed, ref, onMounted } from 'vue';
import { PropType } from 'vue';
import { Command } from '../../types';
import Input from '../ui/Input.vue';
import CustomButton from '../ui/CustomButton.vue';
import IconSelector from '../forms/IconSelector.vue';
import CommandItem from './CommandItem.vue';
import ChevronUpIcon from '../icons/ChevronUpIcon.vue';
import ChevronDownIcon from '../icons/ChevronDownIcon.vue';
import ChevronRightIcon from '../icons/ChevronRightIcon.vue';
import TrashIcon from '../icons/TrashIcon.vue';
import AddIcon from '../icons/AddIcon.vue';
import FolderOpenIcon from '../icons/FolderOpenIcon.vue';

const props = defineProps({
  command: {
    type: Object as PropType<Command>,
    required: true,
  },
  index: {
    type: Number,
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
});

const emit = defineEmits<{
  (e: 'update:command', value: Command): void;
  (e: 'remove', index: number): void;
  (e: 'move', index: number, direction: number): void;
}>();

// Инициализируем submenu если его нет
onMounted(() => {
  if (!props.command.submenu) {
    props.command.submenu = [];
    emit('update:command', props.command);
  }
});

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

// Submenu functions
const updateSubmenuCommand = (subIndex: number, subCommand: Command) => {
  if (props.command.submenu) {
    props.command.submenu[subIndex] = subCommand;
    emit('update:command', props.command);
  }
};

const removeSubmenuCommand = (subIndex: number) => {
  if (props.command.submenu && props.command.submenu.length > 1) {
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

const addSubmenuCommand = () => {
  if (!props.command.submenu) {
    props.command.submenu = [];
  }
  props.command.submenu.push({
    name: '',
    command: undefined,
    hotkey: undefined,
    submenu: null,
    commands: [''],
    inputs: null,
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
  if (cmd?.commands && cmd.commands.length > 1) {
    cmd.commands.splice(cmdIndex, 1);
    emit('update:command', props.command);
  }
};

// Сворачивание секции Submenu
const submenuCollapsed = ref(false);
const toggleSubmenu = () => {
  submenuCollapsed.value = !submenuCollapsed.value;
};
</script>
