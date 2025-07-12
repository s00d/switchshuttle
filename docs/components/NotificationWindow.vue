<template>
  <div class="notification-window">
    <div class="notification-titlebar">
      <div class="notification-buttons">
        <div class="notification-button close" @click="$emit('close')"></div>
      </div>
      <div class="notification-title">Notification</div>
    </div>
    <div class="notification-content">
      <div class="notification-message">{{ message }}</div>
    </div>
  </div>
</template>

<script setup>
defineProps({
  message: {
    type: String,
    required: true
  }
})
</script>

<style scoped>
.notification-window {
  background: #23272e;
  border-radius: 10px;
  box-shadow: 0 8px 32px 0 rgba(31, 38, 135, 0.37);
  overflow: hidden;
  min-width: 300px;
  min-height: 150px;
  max-width: 90vw;
  max-height: 90vh;
  display: flex;
  flex-direction: column;
  align-items: stretch;
  text-align: left;
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  z-index: 1000;
  pointer-events: auto;
}

.notification-titlebar {
  display: flex;
  align-items: center;
  height: 32px;
  background: rgba(40,40,40,0.7);
  backdrop-filter: blur(8px);
  cursor: grab;
  padding: 0 12px;
}

.notification-buttons {
  display: flex;
  gap: 6px;
  margin-right: 12px;
}

.notification-button {
  width: 12px;
  height: 12px;
  border-radius: 50%;
  background: #e66b6b;
  border: 1px solid #e66b6b;
  margin-top: 2px;
  cursor: pointer;
  transition: opacity 0.2s;
}

.notification-button:hover {
  opacity: 0.8;
}

.notification-title {
  font-weight: 500;
  font-size: 15px;
  color: #fff;
  flex: 1;
  text-align: left;
}

.notification-content {
  background: #181a20;
  flex: 1;
  overflow: auto;
  padding: 16px;
  text-align: left;
}

.notification-message {
  color: #e6e6e6;
  font-size: 14px;
  line-height: 1.5;
  white-space: pre-wrap;
  word-break: break-word;
}
</style> 