<template>
  <TransitionGroup 
    name="notification-bubble" 
    tag="div" 
    class="notification-bubbles-container"
  >
    <div
      v-for="notification in visibleNotifications"
      :key="notification.id"
      class="notification-bubble"
      :class="[`notification-bubble-${notification.type}`]"
    >
      <div class="notification-bubble-icon">
        <span v-if="notification.type === 'info'">ℹ️</span>
        <span v-else-if="notification.type === 'success'">✅</span>
        <span v-else-if="notification.type === 'warning'">⚠️</span>
        <span v-else-if="notification.type === 'error'">❌</span>
      </div>
      
      <div class="notification-bubble-content">
        <div class="notification-bubble-title">{{ notification.title }}</div>
        <div class="notification-bubble-message">{{ notification.message }}</div>
      </div>
      
      <button 
        class="notification-bubble-close"
        @click="removeNotification(notification.id)"
      >
        ×
      </button>
    </div>
  </TransitionGroup>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useNotifications } from '~/composables/useNotifications'

const {
  notifications,
  isNotificationPanelOpen,
  removeNotification
} = useNotifications()

// Показываем только непрочитанные уведомления, если панель закрыта
// Баблы автоматически скрываются через 5 секунд (настроено в useNotifications)
const visibleNotifications = computed(() => {
  if (isNotificationPanelOpen.value) {
    return []
  }
  
  return notifications.value
    .filter(n => !n.read)
    .slice(0, 3) // Максимум 3 уведомления одновременно
})
</script>

<style scoped>
.notification-bubbles-container {
  position: fixed;
  top: 50px;
  right: 20px;
  z-index: 10001;
  display: flex;
  flex-direction: column;
  gap: 8px;
  pointer-events: none;
}

.notification-bubble {
  display: flex;
  align-items: flex-start;
  background: rgba(255, 255, 255, 0.95);
  backdrop-filter: blur(20px);
  border-radius: 8px;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.15);
  border: 1px solid rgba(0, 0, 0, 0.1);
  padding: 12px 16px;
  max-width: 320px;
  pointer-events: auto;
  position: relative;
  overflow: hidden;
}

.notification-bubble::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 3px;
  background: var(--notification-color, #007AFF);
}

.notification-bubble-icon {
  font-size: 16px;
  margin-right: 12px;
  flex-shrink: 0;
}

.notification-bubble-content {
  flex: 1;
  min-width: 0;
}

.notification-bubble-title {
  font-weight: 600;
  font-size: 13px;
  color: #333;
  margin-bottom: 2px;
  line-height: 1.3;
}

.notification-bubble-message {
  font-size: 12px;
  color: #666;
  line-height: 1.3;
}

.notification-bubble-close {
  background: none;
  border: none;
  color: #999;
  font-size: 14px;
  cursor: pointer;
  padding: 2px;
  margin-left: 8px;
  border-radius: 2px;
  transition: color 0.2s;
  flex-shrink: 0;
}

.notification-bubble-close:hover {
  color: #666;
}

/* Типы уведомлений */
.notification-bubble-info::before {
  --notification-color: #007AFF;
}

.notification-bubble-success::before {
  --notification-color: #34C759;
}

.notification-bubble-warning::before {
  --notification-color: #FF9500;
}

.notification-bubble-error::before {
  --notification-color: #FF3B30;
}

/* Анимации */
.notification-bubble-enter-active,
.notification-bubble-leave-active {
  transition: all 0.3s ease;
}

.notification-bubble-enter-from {
  opacity: 0;
  transform: translateX(100%) scale(0.8);
}

.notification-bubble-leave-to {
  opacity: 0;
  transform: translateX(100%) scale(0.8);
}

.notification-bubble-move {
  transition: transform 0.3s ease;
}
</style> 