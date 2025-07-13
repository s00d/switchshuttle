<template>
  <div class="notification-panel" v-show="isNotificationPanelOpen">
    <div class="notification-panel-header">
      <h3>{{ $t('demo.notifications.panel.title') }}</h3>
      <button class="clear-all-btn" @click="clearAllNotifications" v-if="notifications.length > 0">
        {{ $t('demo.notifications.panel.clearAll') }}
      </button>
    </div>
    
    <div class="notification-list">
      <div 
        v-for="notification in notifications" 
        :key="notification.id"
        class="notification-item"
        :class="{ 
          'unread': !notification.read,
          [`notification-${notification.type}`]: true
        }"
        @click="markAsRead(notification.id)"
      >
        <div class="notification-icon">
          <span v-if="notification.type === 'info'">ℹ️</span>
          <span v-else-if="notification.type === 'success'">✅</span>
          <span v-else-if="notification.type === 'warning'">⚠️</span>
          <span v-else-if="notification.type === 'error'">❌</span>
        </div>
        
        <div class="notification-content">
          <div class="notification-title">{{ notification.title }}</div>
          <div class="notification-message">{{ notification.message }}</div>
          <div class="notification-time">{{ formatTime(notification.timestamp) }}</div>
        </div>
        
        <div class="notification-actions">
          <button 
            v-if="notification.action"
            class="notification-action-btn"
            @click.stop="notification.action?.callback()"
          >
            {{ notification.action.label }}
          </button>
          <button 
            class="notification-close-btn"
            @click.stop="removeNotification(notification.id)"
          >
            ×
          </button>
        </div>
      </div>
      
      <div v-if="notifications.length === 0" class="no-notifications">
        <div class="no-notifications-icon">🔔</div>
        <div class="no-notifications-text">{{ $t('demo.notifications.panel.noNotifications') }}</div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useNotifications } from '~/composables/useNotifications'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

const {
  notifications,
  isNotificationPanelOpen,
  removeNotification,
  markAsRead,
  clearAllNotifications
} = useNotifications()

function formatTime(timestamp: Date): string {
  const now = new Date()
  const diff = now.getTime() - timestamp.getTime()
  const minutes = Math.floor(diff / 60000)
  const hours = Math.floor(diff / 3600000)
  const days = Math.floor(diff / 86400000)
  
  if (minutes < 1) return t('demo.notifications.time.justNow', 'Только что')
  if (minutes < 60) return t('demo.notifications.time.minutesAgo', { minutes }, `${minutes} мин назад`)
  if (hours < 24) return t('demo.notifications.time.hoursAgo', { hours }, `${hours} ч назад`)
  if (days < 7) return t('demo.notifications.time.daysAgo', { days }, `${days} дн назад`)
  
  return timestamp.toLocaleDateString()
}
</script>

<style scoped>
.notification-panel {
  position: fixed;
  top: 34px;
  right: 20px;
  width: 380px;
  max-height: 500px;
  background: rgba(255, 255, 255, 0.85);
  backdrop-filter: blur(30px) saturate(180%);
  -webkit-backdrop-filter: blur(30px) saturate(180%);
  border-radius: 8px;
  box-shadow: 
    0 20px 40px rgba(0, 0, 0, 0.12),
    0 8px 16px rgba(0, 0, 0, 0.08),
    inset 0 1px 0 rgba(255, 255, 255, 0.6);
  border: 1px solid rgba(255, 255, 255, 0.3);
  z-index: 10000;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  animation: notificationPanelSlideIn 0.3s cubic-bezier(0.25, 0.46, 0.45, 0.94);
}

.notification-panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 18px 20px;
  border-bottom: 1px solid rgba(0, 0, 0, 0.06);
  background: rgba(248, 248, 248, 0.6);
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
}

.notification-panel-header h3 {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
  color: #333;
}

.clear-all-btn {
  background: none;
  border: none;
  color: #007AFF;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  padding: 6px 10px;
  border-radius: 8px;
  transition: all 0.2s cubic-bezier(0.25, 0.46, 0.45, 0.94);
}

.clear-all-btn:hover {
  background: rgba(0, 122, 255, 0.1);
  transform: translateY(-1px);
}

.notification-list {
  flex: 1;
  overflow-y: auto;
  max-height: 500px;
}

.notification-item {
  display: flex;
  align-items: flex-start;
  padding: 14px 16px;
  border-bottom: 1px solid rgba(0, 0, 0, 0.04);
  cursor: pointer;
  transition: all 0.2s cubic-bezier(0.25, 0.46, 0.45, 0.94);
  position: relative;
  background: rgba(255, 255, 255, 0.3);
  gap: 10px;
  min-height: 0;
  align-items: center;
}

.notification-item:hover {
  background: rgba(255, 255, 255, 0.6);
  transform: translateX(-2px);
}

.notification-item.unread {
  background: rgba(0, 122, 255, 0.08);
}

.notification-item.unread::before {
  content: '';
  position: absolute;
  left: 0;
  top: 0;
  bottom: 0;
  width: 3px;
  background: linear-gradient(180deg, #007AFF 0%, #5AC8FA 100%);
  border-radius: 0 2px 2px 0;
}

.notification-icon {
  font-size: 16px;
  flex-shrink: 0;
  opacity: 0.8;
  align-self: center;
  margin-top: 0;
  line-height: 1;
  display: flex;
  align-items: center;
  height: 16px;
}

.notification-content {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 2px;
  align-self: center;
}

.notification-title {
  font-weight: 600;
  font-size: 13px;
  color: #333;
  line-height: 1.2;
  margin: 0;
  padding: 0;
}

.notification-message {
  font-size: 12px;
  color: #666;
  line-height: 1.3;
}

.notification-time {
  font-size: 10px;
  color: #999;
  margin-top: 2px;
}

.notification-actions {
  display: flex;
  align-items: center;
  gap: 6px;
  flex-shrink: 0;
  margin-top: 0;
  align-self: center;
}

.notification-action-btn {
  background: linear-gradient(135deg, #007AFF 0%, #5AC8FA 100%);
  color: white;
  border: none;
  padding: 4px 8px;
  border-radius: 6px;
  font-size: 10px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s cubic-bezier(0.25, 0.46, 0.45, 0.94);
  box-shadow: 0 1px 4px rgba(0, 122, 255, 0.3);
  line-height: 1;
}

.notification-action-btn:hover {
  background: linear-gradient(135deg, #0056CC 0%, #007AFF 100%);
  transform: translateY(-1px);
  box-shadow: 0 2px 6px rgba(0, 122, 255, 0.4);
}

.notification-close-btn {
  background: none;
  border: none;
  color: rgba(0, 0, 0, 0.4);
  font-size: 14px;
  font-weight: 300;
  cursor: pointer;
  padding: 2px;
  border-radius: 3px;
  transition: all 0.2s cubic-bezier(0.25, 0.46, 0.45, 0.94);
  line-height: 1;
  width: 16px;
  height: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.notification-close-btn:hover {
  color: rgba(0, 0, 0, 0.7);
  background: rgba(0, 0, 0, 0.05);
  transform: scale(1.1);
}

.no-notifications {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 60px 20px;
  color: rgba(0, 0, 0, 0.4);
  background: rgba(255, 255, 255, 0.3);
}

.no-notifications-icon {
  font-size: 48px;
  margin-bottom: 16px;
  opacity: 0.6;
  filter: blur(0.5px);
}

.no-notifications-text {
  font-size: 14px;
  font-weight: 500;
  text-align: center;
  line-height: 1.4;
}

/* Типы уведомлений */
.notification-info {
  border-left: 4px solid #007AFF;
}

.notification-success {
  border-left: 4px solid #34C759;
}

.notification-warning {
  border-left: 4px solid #FF9500;
}

.notification-error {
  border-left: 4px solid #FF3B30;
}

/* Скроллбар */
.notification-list::-webkit-scrollbar {
  width: 8px;
}

.notification-list::-webkit-scrollbar-track {
  background: transparent;
}

.notification-list::-webkit-scrollbar-thumb {
  background: rgba(0, 0, 0, 0.15);
  border-radius: 4px;
  border: 2px solid transparent;
  background-clip: content-box;
}

.notification-list::-webkit-scrollbar-thumb:hover {
  background: rgba(0, 0, 0, 0.25);
  background-clip: content-box;
}

/* Анимация появления панели */
@keyframes notificationPanelSlideIn {
  from {
    opacity: 0;
    transform: translateX(20px) scale(0.95);
  }
  to {
    opacity: 1;
    transform: translateX(0) scale(1);
  }
}
</style> 