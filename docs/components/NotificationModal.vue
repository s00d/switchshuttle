<template>
  <div class="notification-overlay" v-show="show" @click="close">
    <Window :title="title" :initial-x="200" :initial-y="200" :z="2000" @close="close">
      <template #titlebar>
        <div class="window-title">{{ title }}</div>
      </template>
      <div class="notification-content">
        <div class="notification-message">{{ message }}</div>
      </div>
      <div class="notification-footer">
        <button class="notification-btn" @click="close">OK</button>
      </div>
    </Window>
  </div>
</template>

<script setup>
import Window from './Window.vue'

const props = defineProps({
  show: { type: Boolean, default: false },
  title: { type: String, default: 'Notification' },
  message: { type: String, default: '' }
})

const emit = defineEmits(['close'])

function close() {
  emit('close')
}
</script>

<style scoped>
.notification-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  background: rgba(0,0,0,0.18);
  z-index: 2000;
  display: flex;
  align-items: center;
  justify-content: center;
  animation: fadeIn 0.2s;
}

@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}

.notification-content {
  padding: 18px 20px 10px 20px;
  font-size: 14px;
  color: #1d1d1f;
  flex: 1;
  overflow-y: auto;
  white-space: pre-line;
}

.notification-footer {
  display: flex;
  justify-content: flex-end;
  padding: 10px 20px 14px 20px;
  border-top: 1px solid #f0f0f0;
}

.notification-btn {
  background: #007AFF;
  color: #fff;
  border: none;
  border-radius: 6px;
  padding: 7px 18px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: background 0.2s;
}

.notification-btn:hover {
  background: #005ecb;
}

.notification-btn:active {
  background: #003e7e;
}
</style> 