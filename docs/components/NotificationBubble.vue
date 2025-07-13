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
  background: rgba(255, 255, 255, 0.8);
  backdrop-filter: blur(30px) saturate(1.5);
  -webkit-backdrop-filter: blur(30px) saturate(1.5);
  border-radius: 6px;
  box-shadow: 0 20px 40px rgba(0, 0, 0, 0.15), 0 0 10px rgba(0, 0, 0, 0.1);
  border: 1px solid rgba(255, 255, 255, 0.15);
  padding: 12px 16px;
  max-width: 320px;
  pointer-events: auto;
  position: relative;
  overflow: hidden;
  font-family: "SF Pro Display", -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
  font-size: 13px;
  font-weight: 500;
  color: #1d1d1f;
}

.notification-bubble::after {
  content: "";
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  box-shadow: inset 0 0 0 1px rgba(255, 255, 255, 0.15);
  border-radius: 6px;
  pointer-events: none;
}

.notification-bubble-icon {
  font-size: 16px;
  margin-right: 12px;
  flex-shrink: 0;
  margin-top: 1px;
}

.notification-bubble-content {
  flex: 1;
  min-width: 0;
}

.notification-bubble-title {
  font-weight: 600;
  font-size: 13px;
  color: #1d1d1f;
  margin-bottom: 2px;
  line-height: 1.3;
}

.notification-bubble-message {
  font-size: 12px;
  color: #86868b;
  line-height: 1.3;
}

.notification-bubble-close {
  background: none;
  border: none;
  color: #86868b;
  font-size: 14px;
  cursor: pointer;
  padding: 4px;
  margin-left: 8px;
  border-radius: 4px;
  transition: all 0.2s ease;
  flex-shrink: 0;
  font-weight: 500;
}

.notification-bubble-close:hover {
  background: rgba(0, 0, 0, 0.05);
  color: #1d1d1f;
}

/* Типы уведомлений с macOS цветами */
.notification-bubble-info {
  border-left: 3px solid #007AFF;
}

.notification-bubble-success {
  border-left: 3px solid #34C759;
}

.notification-bubble-warning {
  border-left: 3px solid #FF9500;
}

.notification-bubble-error {
  border-left: 3px solid #FF3B30;
}

/* Анимации в стиле macOS */
.notification-bubble-enter-active,
.notification-bubble-leave-active {
  transition: all 0.2s ease;
}

.notification-bubble-enter-from {
  opacity: 0;
  transform: translateX(100%) scale(0.95);
}

.notification-bubble-leave-to {
  opacity: 0;
  transform: translateX(100%) scale(0.95);
}

.notification-bubble-move {
  transition: transform 0.2s ease;
}

/* Hover эффект */
.notification-bubble:hover {
  background: rgba(255, 255, 255, 0.9);
  transform: translateY(-1px);
  box-shadow: 0 24px 48px rgba(0, 0, 0, 0.18), 0 0 12px rgba(0, 0, 0, 0.12);
}
</style> 