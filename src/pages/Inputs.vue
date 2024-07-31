<template>
  <div class="container">
    <form id="inputForm">
      <div v-for="(value, key) in inputs" :key="key" class="input-group">
        <label :for="key">{{ key }}</label>
        <input type="text" :name="key" :id="key" v-model="inputs[key]">
      </div>
    </form>
    <div id="error-message">{{ errorMessage }}</div>
    <div class="buttons">
      <button type="button" class="button-blue" @click="submitForm">OK</button>
      <button type="button" class="cancel-button" @click="onClose">Cancel</button>
    </div>
  </div>
</template>

<script setup>
import { ref, onBeforeMount, onMounted, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api';
import { emit, listen } from '@tauri-apps/api/event'
import { appWindow } from '@tauri-apps/api/window';
import eventBus from '../plugins/eventBus';

const command = ref('');
const inputs = ref({});
const errorMessage = ref('');

function handleInputData(payload) {
  command.value = payload[0];
  inputs.value = payload[1];
}

function submitForm() {
  invoke('execute_command_with_inputs', { inputs: inputs.value, command: command.value }).then(() => {
    onClose();
  }).catch((error) => {
    errorMessage.value = error;
  });
}

function onClose() {
  appWindow.hide();
}

onMounted(() => {
  eventBus.on('input_data', handleInputData);
});

onUnmounted(() => {
  eventBus.off('input_data', handleInputData);
});
</script>

<style scoped>
/* Add styles from inputs.html */
.container {
  overflow-x: scroll;
}

h1 {
  margin-top: 20px;
  font-size: 24px;
  color: #333;
}

form {
  display: flex;
  flex-direction: column;
  margin-top: 20px;
  padding: 10px;
}

.input-group {
  display: flex;
  flex-direction: column;
  align-items: center;
  margin-bottom: 10px;
  width: 100%;
}

label {
  font-size: 16px;
  color: #333;
  margin-bottom: 5px;
  background: #f0f0f5;
  padding: 2px 6px;
  border-radius: 4px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  width: 80%;
}

input[type="text"] {
  padding: 10px;
  font-size: 16px;
  border: 1px solid #ccc;
  border-radius: 6px;
  width: 80%;
  box-shadow: inset 0 1px 3px rgba(0, 0, 0, 0.1);
  transition: border-color 0.2s, box-shadow 0.2s;
}

input[type="text"]:focus {
  border-color: #007aff;
  box-shadow: 0 0 3px 2px rgba(0, 123, 255, 0.25);
  outline: none;
}

.buttons {
  display: flex;
  justify-content: center;
}

#error-message {
  color: red;
  margin-top: 10px;
}
</style>
