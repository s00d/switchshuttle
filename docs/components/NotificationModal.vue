<template>
      <div class="notification-content">
        <div class="notification-message">{{ message }}</div>
      </div>
</template>

<script setup>
const props = defineProps({
  show: { type: Boolean, default: false },
  title: { type: String, default: 'Notification' },
  message: { type: String, default: '' }
})
</script>

<style scoped>
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