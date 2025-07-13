<template>
    <div class="notification-content">
      <div class="notification-message">{{ message }}</div>
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