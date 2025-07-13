<template>
  <div class="taskbar">
    <div class="taskbar-items">
      <div 
        v-for="window in allWindows" 
        :key="window.id"
        class="taskbar-item"
        :class="{ 
          'active': window.id === activeWindowId,
          'minimized': window.isMinimized,
          'maximized': window.isMaximized
        }"
        @click="handleWindowClick(window.id, window.isMinimized)"
        :title="window.title"
      >
        <div class="taskbar-item-icon">{{ window.icon }}</div>
        <div class="taskbar-item-title">{{ window.title }}</div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useWindowManager } from '~/composables/useWindowManager'

const { 
  windows,
  minimizedWindows, 
  maximizedWindows,
  activeWindowId, 
  restoreWindow, 
  activateWindow
} = useWindowManager()

// Список окон с их иконками и заголовками
const windowInfo: Record<string, { title: string; icon: string }> = {
  'readme-window': { title: 'README', icon: '📖' },
  'terminal-window': { title: 'Terminal', icon: '💻' },
  'browser-window': { title: 'Browser', icon: '🌐' },
  'galaxy-game-window': { title: 'Galaxy Game', icon: '🚀' },
  'help-window': { title: 'Help', icon: '❓' },
  'about-window': { title: 'About', icon: 'ℹ️' },
  'homepage-window': { title: 'Homepage', icon: '🏠' },
  'notification-modal': { title: 'Notification', icon: '🔔' },
  'config-editor-window': { title: 'Config Editor', icon: '⚙️' },
  'config-folder-window': { title: 'Config Folder', icon: '📁' }
}

// Computed для получения всех окон с их состоянием
const allWindows = computed(() => {
  const windowsList: Array<{ 
    id: string; 
    title: string; 
    icon: string; 
    isMinimized: boolean; 
    isMaximized: boolean;
    isActive: boolean;
  }> = []
  
  for (const [id, info] of Object.entries(windowInfo)) {
    const isMinimized = minimizedWindows.value[id] || false
    const isMaximized = maximizedWindows.value[id] || false
    const isActive = activeWindowId.value === id
    const hasWindow = windows.value.some(w => w.id === id) // окно существует в списке
    

    
    // Показываем только существующие окна
    if (hasWindow) {
      windowsList.push({
        id,
        title: info.title,
        icon: info.icon,
        isMinimized,
        isMaximized,
        isActive
      })
    }
  }
  return windowsList
})

// Обработчик клика по окну в доке
function handleWindowClick(windowId: string, isMinimized: boolean) {
  console.log('TaskBar click:', windowId, 'minimized:', isMinimized)
  if (isMinimized) {
    // Если окно свернуто - разворачиваем
    console.log('Restoring window:', windowId)
    restoreWindow(windowId)
  } else {
    // Если окно открыто - активируем
    console.log('Activating window:', windowId)
    activateWindow(windowId)
  }
}
</script>

<style scoped>
.taskbar {
  position: fixed;
  bottom: 0;
  left: 0;
  right: 0;
  height: 40px;
  background: rgba(255, 255, 255, 0.9);
  backdrop-filter: blur(10px);
  border-top: 1px solid rgba(0, 0, 0, 0.1);
  display: flex;
  align-items: center;
  padding: 0 20px;
  z-index: 9999;
}

.taskbar-items {
  display: flex;
  gap: 10px;
  align-items: center;
}

.taskbar-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 12px;
  background: rgba(0, 0, 0, 0.05);
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.2s ease;
  min-width: 120px;
  position: relative;
}

.taskbar-item:hover {
  background: rgba(0, 0, 0, 0.1);
}

.taskbar-item.active {
  background: rgba(0, 122, 255, 0.2);
  color: #007AFF;
}

.taskbar-item.minimized {
  opacity: 0.6;
  background: rgba(0, 0, 0, 0.03);
}

.taskbar-item.minimized:hover {
  opacity: 1;
  background: rgba(0, 0, 0, 0.1);
}

.taskbar-item.maximized::after {
  content: '';
  position: absolute;
  top: 2px;
  right: 2px;
  width: 6px;
  height: 6px;
  background: #007AFF;
  border-radius: 50%;
}

.taskbar-item-icon {
  font-size: 16px;
}

.taskbar-item-title {
  font-size: 12px;
  font-weight: 500;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
</style> 