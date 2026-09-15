<template>
  <div :class="ui.root()">
    <div :class="ui.header()">
      <div :class="ui.headerLeft()">
        <div :class="ui.iconWrap()">
          <FolderOpenIcon :class="ui.icon()" />
        </div>
        <h4 :class="ui.title()">
          Group {{ index + 1 }}{{ command.name ? ` - ${command.name}` : '' }}
        </h4>
      </div>
      <div :class="ui.actions()">
        <CustomButton
          variant="ghost"
          size="sm"
          :disabled="index === 0"
          :class="ui.ghostAccentStrong()"
          @click="$emit('move', index, -1)"
        >
          <ChevronUpIcon />
        </CustomButton>
        <CustomButton
          variant="ghost"
          size="sm"
          :disabled="index === (parentCommands?.length || 0) - 1"
          :class="ui.ghostAccentStrong()"
          @click="$emit('move', index, 1)"
        >
          <ChevronDownIcon />
        </CustomButton>
        <CustomButton
          variant="danger"
          size="sm"
          :class="ui.dangerGhost()"
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
            input-class="bg-white h-8 px-2 py-1"
            @update:modelValue="handleIconChange"
          />
        </div>
        <div :class="ui.nameField()">
          <Input
            v-model="command.name"
            label="Name"
            placeholder="Group name"
            required
            input-class="bg-white h-8"
          />
        </div>
      </div>
    </div>

    <div :class="ui.section()">
      <div :class="ui.sectionInner()">
        <div :class="ui.sectionHeader()">
          <div :class="ui.sectionHeaderLeft()">
            <CustomButton variant="ghost" @click="toggleSubmenu">
              <ChevronRightIcon :collapsed="submenuCollapsed" />
            </CustomButton>
            <label :class="ui.sectionLabel()">Submenu</label>
          </div>
          <CustomButton
            variant="ghost"
            size="sm"
            :class="ui.ghostAccent()"
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
          :class="ui.section()"
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
import { commandCardTv } from './commandCardTheme';

defineOptions({ name: 'CommandSubmenu' });

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

const ui = commandCardTv({ tone: 'group' });

onMounted(() => {
  if (!props.command.submenu) {
    props.command.submenu = [];
    emit('update:command', props.command);
  }
});

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

const submenuCollapsed = ref(false);
const toggleSubmenu = () => {
  submenuCollapsed.value = !submenuCollapsed.value;
};
</script>
