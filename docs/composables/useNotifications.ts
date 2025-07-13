import { ref, computed } from 'vue'

export interface Notification {
  id: string
  title: string
  message: string
  type: 'info' | 'success' | 'warning' | 'error'
  timestamp: Date
  read: boolean
  action?: {
    label: string
    callback: () => void
  }
}

export function useNotifications() {
  // Используем useState для SSR совместимости
  const notifications = useState<Notification[]>('notifications', () => [])
  const isNotificationPanelOpen = useState<boolean>('notificationPanelOpen', () => false)
  const bubbleTimeouts = useState<Map<string, NodeJS.Timeout>>('bubbleTimeouts', () => new Map())

  // Добавить уведомление
  function addNotification(notification: Omit<Notification, 'id' | 'timestamp' | 'read'>) {
    const newNotification: Notification = {
      ...notification,
      id: Date.now().toString(),
      timestamp: new Date(),
      read: false
    }
    
    notifications.value.unshift(newNotification)
    
    // Ограничиваем количество уведомлений
    if (notifications.value.length > 50) {
      notifications.value = notifications.value.slice(0, 50)
    }

    // Автоматически скрываем бабл через 5 секунд (но уведомление остается в панели)
    const timeout = setTimeout(() => {
      markAsRead(newNotification.id)
      bubbleTimeouts.value.delete(newNotification.id)
    }, 5000)

    bubbleTimeouts.value.set(newNotification.id, timeout)
  }

  // Удалить уведомление
  function removeNotification(id: string) {
    // Очищаем таймаут если он есть
    const timeout = bubbleTimeouts.value.get(id)
    if (timeout) {
      clearTimeout(timeout)
      bubbleTimeouts.value.delete(id)
    }
    
    notifications.value = notifications.value.filter(n => n.id !== id)
  }

  // Отметить как прочитанное
  function markAsRead(id: string) {
    const notification = notifications.value.find(n => n.id === id)
    if (notification) {
      notification.read = true
      
      // Очищаем таймаут если он есть
      const timeout = bubbleTimeouts.value.get(id)
      if (timeout) {
        clearTimeout(timeout)
        bubbleTimeouts.value.delete(id)
      }
    }
  }

  // Очистить все уведомления
  function clearAllNotifications() {
    // Очищаем все таймауты
    bubbleTimeouts.value.forEach(timeout => clearTimeout(timeout))
    bubbleTimeouts.value.clear()
    
    notifications.value = []
  }

  // Переключить панель уведомлений
  function toggleNotificationPanel() {
    isNotificationPanelOpen.value = !isNotificationPanelOpen.value
    // При открытии панели отмечаем все уведомления как прочитанные
    // но не удаляем их - они остаются в панели до ручного удаления
    if (isNotificationPanelOpen.value) {
      notifications.value.forEach(n => {
        n.read = true
        // Очищаем таймауты при открытии панели
        const timeout = bubbleTimeouts.value.get(n.id)
        if (timeout) {
          clearTimeout(timeout)
          bubbleTimeouts.value.delete(n.id)
        }
      })
    }
  }

  // Закрыть панель уведомлений
  function closeNotificationPanel() {
    isNotificationPanelOpen.value = false
  }

  // Computed свойства
  const unreadCount = computed(() => {
    return notifications.value.filter(n => !n.read).length
  })

  const recentNotifications = computed(() => {
    return notifications.value.slice(0, 10) // Последние 10 уведомлений
  })

  const hasUnreadNotifications = computed(() => {
    return unreadCount.value > 0
  })

  return {
    notifications,
    isNotificationPanelOpen,
    unreadCount,
    recentNotifications,
    hasUnreadNotifications,
    addNotification,
    removeNotification,
    markAsRead,
    clearAllNotifications,
    toggleNotificationPanel,
    closeNotificationPanel
  }
} 