<template>
  <table class="nested-table">
    <thead>
      <tr>
        <th>Subcommand Name</th>
        <th>Single Command</th>
        <th>Commands</th>
        <th>Hotkey</th>
        <th>Actions</th>
      </tr>
    </thead>
    <tbody>
      <tr v-for="(subCommand, subIndex) in subcommands" :key="'sub' + subIndex" :data-index="subIndex">
        <td><input type="text" class="form-control" v-model="subCommand.name"></td>
        <td><input type="text" class="form-control" v-model="subCommand.command" :disabled="subCommand.commands && subCommand.commands.length"></td>
        <td>
          <div v-if="!subCommand.submenu" v-for="(cmd, cmdIndex) in subCommand.commands" :key="cmdIndex" class="d-flex align-items-center">
            <input type="text" class="form-control" v-model="subCommand.commands[cmdIndex]">
            <button class="cancel-button" @click="$emit('deleteInnerSubcommand', commandIndex, subIndex, cmdIndex)">x</button>
          </div>
          <button v-if="!subCommand.submenu" class="button-blue" @click="$emit('addInnerSubcommand', commandIndex, subIndex)">+</button>
        </td>
        <td>
          <div class="d-flex align-items-center">
            <input type="checkbox" class="mr-2" :checked="subCommand.hotkey !== null" @change="$emit('toggleHotkeyInput', commandIndex, subIndex, true)">
            <input type="text" class="form-control" v-model="subCommand.hotkey" v-if="subCommand.hotkey !== null" @keydown="$emit('hotkeyHandler', $event, commandIndex, subIndex)">
          </div>
        </td>
        <td>
          <button class="button-blue" @click="$emit('addSubcommand', commandIndex, subIndex)">+</button>
          <button class="cancel-button" @click="$emit('deleteSubcommand', commandIndex, subIndex)">x</button>
        </td>
      </tr>
    </tbody>
  </table>
</template>

<script setup>
const props = defineProps(['subcommands', 'commandIndex']);
const emit = defineEmits(['addInnerSubcommand', 'deleteInnerSubcommand', 'toggleHotkeyInput', 'hotkeyHandler', 'deleteSubcommand', 'addSubcommand']);
</script>

<style scoped>
/* Add styles from subcommand table in editor.html */
.nested-table {
  margin-left: 5px;
}

.form-control {
  width: calc(100% - 20px);
  margin: 0 10px;
}

.nested-table {
  margin-left: 5px;
}
</style>
