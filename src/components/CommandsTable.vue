<template>
  <table id="commands-table">
    <thead>
    <tr>
      <th>Command Name</th>
      <th>Single Command</th>
      <th>Commands</th>
      <th>Hotkey</th>
      <th>Actions</th>
    </tr>
    </thead>
    <tbody>
    <template v-for="(command, index) in commands" :key="index">
      <tr :data-index="index">
        <td><input type="text" class="form-control" v-model="command.name"></td>
        <td><input v-if="!command.submenu" type="text" class="form-control" v-model="command.command" :disabled="!!(command.commands && command.commands.length)"></td>
        <td>
          <div v-if="!command.submenu" v-for="(_cmd, cmdIndex) in command.commands" :key="cmdIndex" class="d-flex align-items-center">
            <input type="text" class="form-control" v-model="command.commands[cmdIndex]">
            <button class="cancel-button" @click="deleteInnerCommand(index, cmdIndex)">x</button>
          </div>
          <button v-if="!command.submenu" class="button-blue fill-width" @click="addInnerCommand(index)">+</button>
        </td>
        <td>
          <div class="d-flex align-items-center">
            <input type="checkbox" class="mr-2" :checked="command.hotkey !== null" @change="toggleHotkeyInput(index)">
            <input type="text" class="form-control" v-model="command.hotkey" v-if="command.hotkey !== null" @keydown="hotkeyHandler(command, 'hotkey', $event)" :disabled="!!command.submenu">
          </div>
        </td>
        <td>
          <button class="button-blue" @click="addSubCommand(index)">+</button>
          <button class="cancel-button" @click="deleteCommand(index)">x</button>
        </td>
      </tr>
      <tr v-if="command.submenu" :key="'sub' + index">
        <td colspan="5">
          <CommandsTable :commands="command.submenu" @addCommand="addSubCommand(index)" @deleteCommand="deleteSubCommand(index, $event)" @addInnerCommand="addInnerSubCommand(index, $event)" @deleteInnerCommand="deleteInnerSubCommand(index, $event, $event)" @toggleHotkeyInput="toggleSubHotkeyInput(index, $event)" @hotkeyHandler="hotkeyHandler" />
        </td>
      </tr>
    </template>
    </tbody>
  </table>
  <div class="table-actions text-center">
    <button class="button-blue fill-width" @click="addCommand">+</button>
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
<style scoped>
table {
  width: 100%;
  border-collapse: collapse;
}

th, td {
  padding: 10px;
  border: 1px solid #ccc;
  text-align: left;
}

th {
  background: #f0f0f5;
}

.form-control {
  width: calc(100% - 20px);
  margin: 0 10px;
  height: 28px;
}

.d-flex {
  display: flex;
  align-items: center;
}

.mr-2 {
  margin-right: 8px;
}

.button-blue {
  background-color: #007bff;
  color: white;
  border: none;
  padding: 5px 10px;
  border-radius: 4px;
  cursor: pointer;
}

.cancel-button {
  background-color: #dc3545;
  color: white;
  border: none;
  padding: 5px 10px;
  border-radius: 4px;
  cursor: pointer;

}

.fill-width {
  width: 100%;
}

.table-actions {
  border: 1px solid #ccc;
  border-radius: 6px;
  padding: 1px;
  display: flex;
  justify-content: center;
  align-items: center;
  background-color: #f9f9f9;
}
</style>
