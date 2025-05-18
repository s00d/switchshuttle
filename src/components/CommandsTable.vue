<!-- src/components/CommandsTable.vue -->
<template>
  <table class="w-full table-fixed border-collapse text-sm">
    <thead>
    <tr class="bg-gray-100 text-left border-b">
      <th class="p-2">Command Name</th>
      <th class="p-2">Single Command</th>
      <th class="p-2">Commands</th>
      <th class="p-2">Hotkey</th>
      <th class="p-2">Actions</th>
    </tr>
    </thead>
    <tbody>
    <template v-for="(command, index) in commands" :key="index">
      <tr class="border-b" :data-index="index">
        <td class="p-2">
          <input type="text" class="w-full border border-gray-300 px-2 py-1" v-model="command.name" />
        </td>
        <td class="p-2">
          <input
              v-if="!command.submenu"
              type="text"
              class="w-full border border-gray-300 px-2 py-1"
              v-model="command.command"
              :disabled="!!(command.commands && command.commands.length)"
          />
        </td>
        <td class="p-2">
          <div v-if="!command.submenu" class="space-y-1">
            <div
                v-for="(_cmd, cmdIndex) in command.commands"
                :key="cmdIndex"
                class="flex items-center gap-2"
            >
              <input type="text" class="flex-1 border border-gray-300 px-2 py-1" v-model="command.commands[cmdIndex]" />
              <button
                  class="bg-red-500 text-white px-2 py-1 text-xs hover:bg-red-600"
                  @click="deleteInnerCommand(index, cmdIndex)"
              >
                x
              </button>
            </div>
            <button
                class="w-full bg-blue-600 text-white text-xs py-1 mt-1 hover:bg-blue-700"
                @click="addInnerCommand(index)"
            >
              +
            </button>
          </div>
        </td>
        <td class="p-2">
          <div class="flex items-center gap-2">
            <input
                type="checkbox"
                :checked="command.hotkey !== null"
                @change="toggleHotkeyInput(index)"
            />
            <input
                v-if="command.hotkey !== null"
                type="text"
                class="flex-1 border border-gray-300 px-2 py-1"
                v-model="command.hotkey"
                @keydown="hotkeyHandler(command, 'hotkey', $event)"
                :disabled="!!command.submenu"
            />
          </div>
        </td>
        <td class="p-2 flex gap-2">
          <button
              class="bg-blue-600 text-white px-2 py-1 text-xs hover:bg-blue-700"
              @click="addSubCommand(index)"
          >
            +
          </button>
          <button
              class="bg-red-500 text-white px-2 py-1 text-xs hover:bg-red-600"
              @click="deleteCommand(index)"
          >
            x
          </button>
        </td>
      </tr>
      <tr v-if="command.submenu" :key="'sub' + index">
        <td colspan="5" class="p-2">
          <CommandsTable
              :commands="command.submenu"
              @addCommand="addSubCommand(index)"
              @deleteCommand="deleteSubCommand(index, $event)"
              @addInnerCommand="addInnerSubCommand(index, $event)"
              @deleteInnerCommand="deleteInnerSubCommand(index, $event, $event)"
              @toggleHotkeyInput="toggleSubHotkeyInput(index, $event)"
              @hotkeyHandler="hotkeyHandler"
          />
        </td>
      </tr>
    </template>
    </tbody>
  </table>

  <div class="flex justify-center border border-gray-300 rounded mt-2 bg-gray-50 py-2">
    <button class="w-full max-w-xs bg-blue-600 text-white py-1 text-sm hover:bg-blue-700" @click="addCommand">
      +
    </button>
  </div>
</template>


<script lang="ts" setup>
import { PropType } from 'vue';
import { Command } from '../types';

const props = defineProps({
  commands: {
    type: Array as PropType<Command[]>,
    required: true
  }
});

const emit = defineEmits<{
  (e: 'addCommand'): void;
  (e: 'deleteCommand', index: number): void;
  (e: 'addInnerCommand', index: number): void;
  (e: 'deleteInnerCommand', index: number, cmdIndex: number): void;
  (e: 'toggleHotkeyInput', index: number): void;
  (e: 'hotkeyHandler', command: Command, key: string, event: KeyboardEvent): void;
}>();

function addCommand() {
  emit('addCommand');
}

function deleteCommand(index: number) {
  emit('deleteCommand', index);
}

function addInnerCommand(index: number) {
  emit('addInnerCommand', index);
}

function deleteInnerCommand(index: number, cmdIndex: number) {
  emit('deleteInnerCommand', index, cmdIndex);
}

function toggleHotkeyInput(index: number) {
  emit('toggleHotkeyInput', index);
}

function hotkeyHandler(command: Command, key: string, event: KeyboardEvent) {
  emit('hotkeyHandler', command, key, event);
}

function addSubCommand(index: number) {
  if (!props.commands[index].submenu) {
    props.commands[index].submenu = [];
  }
  props.commands[index].submenu.push({ name: '', command: '', hotkey: null, submenu: null, commands: [] });
}

function deleteSubCommand(parentIndex: number, subIndex: number) {
  props.commands[parentIndex].submenu?.splice(subIndex, 1);
}

function addInnerSubCommand(parentIndex: number, subIndex: number) {
  const command = props.commands[parentIndex].submenu?.[subIndex];
  if (command) {
    command.commands.push('');
  }
}

function deleteInnerSubCommand(parentIndex: number, subIndex: number, cmdIndex: number) {
  const command = props.commands[parentIndex].submenu?.[subIndex];
  if (command) {
    command.commands.splice(cmdIndex, 1);
  }
}

function toggleSubHotkeyInput(parentIndex: number, subIndex: number) {
  const command = props.commands[parentIndex].submenu?.[subIndex];
  if (command) {
    command.hotkey = command.hotkey ? null : '';
  }
}
</script>