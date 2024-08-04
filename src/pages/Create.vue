<template>
  <div class="container">
    <h1>Create New Config</h1>
    <input type="text" id="fileName" name="fileName" placeholder="Enter file name (without .json)" v-model="fileName" required>
    <div id="error-message">{{ errorMessage }}</div>
    <button type="submit" class="button-blue" @click="onCreate">Create</button>
    <button type="button" class="cancel-button" @click="onClose">Cancel</button>
  </div>
</template>

<script setup>
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { useRouter } from 'vue-router';

const router = useRouter();
const fileName = ref('');
const errorMessage = ref('');

function onCreate() {
  invoke('create_new_config', { fileName: fileName.value }).then(() => {
    onClose();
  }).catch((error) => {
    errorMessage.value = error;
  });
}

function onClose() {
  router.push('/').catch((error) => {});
  getCurrentWindow().hide();
}
</script>

<style scoped>
/* Add styles from create.html */
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

label {
  margin-bottom: 10px;
  font-size: 16px;
  color: #333;
  text-align: left;
}

input[type="text"] {
  padding: 10px;
  font-size: 16px;
  border: 1px solid #ccc;
  border-radius: 6px;
  width: 80%;
  margin: 20px auto;
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
  justify-content: space-between;
}

#error-message {
  color: red;
  margin-top: 10px;
}
</style>
