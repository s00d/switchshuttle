<template>
  <div class="desktop-icons-grid">
    <div 
      v-for="icon in desktopIcons" 
      :key="icon.id"
      class="desktop-icon"
      @click="icon.action"
      :title="icon.title"
    >
      <div class="icon-image">
        <span v-if="icon.emoji">{{ icon.emoji }}</span>
        <img 
          v-else-if="icon.image" 
          :src="icon.image" 
          :alt="icon.label" 
          class="desktop-icon-img"
          @error="handleImageError"
        >
      </div>
      <div class="icon-label">{{ icon.label }}</div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useRuntimeConfig } from '#app'
import { useWindowManager } from '~/composables/useWindowManager'
import { useNotifications } from '~/composables/useNotifications'

// Nuxt config для правильного формирования путей
const config = useRuntimeConfig()

// Функция для правильного формирования путей к изображениям
function getImagePath(imagePath: string) {
  const baseURL = config.app?.baseURL || ''
  
  // Если baseURL пустой или равен '/', используем относительный путь
  if (!baseURL || baseURL === '/') {
    return imagePath
  }
  
  // Иначе добавляем baseURL
  return `${baseURL}${imagePath}`
}

// Функция для обработки ошибок загрузки изображений
function handleImageError(event: Event) {
  const img = event.target as HTMLImageElement
  console.warn(`Failed to load image: ${img.src}`)
  // Можно добавить fallback изображение или скрыть элемент
}

// Определяем интерфейс для иконки
interface DesktopIcon {
  id: string
  label: string
  image?: string
  emoji?: string
  title: string
  action: () => void
}

// Массив иконок рабочего стола
const desktopIcons = computed<DesktopIcon[]>(() => [
  {
    id: 'switchshuttle',
    label: 'SwitchShuttle',
    image: getImagePath('/switchshuttle.svg'),
    title: 'SwitchShuttle (Click to show menu bar icon)',
    action: () => {
      // Проверяем, есть ли уже иконка SwitchShuttle в меню-баре
      const { addNotification } = useNotifications()
      addNotification({
        title: 'SwitchShuttle is already active',
        message: 'SwitchShuttle icon is already displayed in the menu bar',
        type: 'warning'
      })
    }
  },
  {
    id: 'galaxy-game',
    label: 'Galaxy\nGame',
    emoji: '🚀',
    title: 'Galaxy Game (Click to play)',
    action: () => {
      const { openWindow } = useWindowManager()
      openWindow({
        id: 'galaxy-game-window',
        component: 'galaxy-game-window',
        props: {
          windowId: 'galaxy-game-window',
          title: 'Galaxy Game'
        },
        position: { x: 150, y: 150 },
        size: { width: 800, height: 600 }
      })
    }
  },
  {
    id: 'readme',
    label: 'README',
    emoji: '📖',
    title: 'README (Click to open)',
    action: () => {
      const { openWindow } = useWindowManager()
      openWindow({
        id: 'readme-window',
        component: 'readme-window',
        props: {
          windowId: 'readme-window',
          title: 'README'
        },
        position: { x: 400, y: 120 },
        size: { width: 600, height: 500 }
      })
    }
  },
  {
    id: 'terminal',
    label: 'Terminal',
    emoji: '💻',
    title: 'Terminal (Click to open)',
    action: () => {
      const { openWindow } = useWindowManager()
      openWindow({
        id: 'terminal-window',
        component: 'terminal-window',
        props: {
          windowId: 'terminal-window',
          title: 'iTerm2 — SwitchShuttle Demo'
        },
        position: { x: 120, y: 120 },
        size: { width: 500, height: 380 }
      })
    }
  },
  {
    id: 'browser',
    label: 'Browser',
    emoji: '🌐',
    title: 'Browser (Click to open)',
    action: () => {
      const { openWindow } = useWindowManager()
      openWindow({
        id: 'browser-window',
        component: 'browser-window',
        props: {
          windowId: 'browser-window',
          title: 'Safari — SwitchShuttle'
        },
        position: { x: 200, y: 200 },
        size: { width: 900, height: 600 }
      })
    }
  },
  {
    id: 'help',
    label: 'Help',
    emoji: '❓',
    title: 'Help (Click to open)',
    action: () => {
      const { openWindow } = useWindowManager()
      openWindow({
        id: 'help-window',
        component: 'help-window',
        props: {
          windowId: 'help-window',
          title: 'Help'
        },
        position: { x: 300, y: 180 },
        size: { width: 750, height: 550 }
      })
    }
  },
  {
    id: 'about',
    label: 'About',
    emoji: 'ℹ️',
    title: 'About (Click to open)',
    action: () => {
      const { openWindow } = useWindowManager()
      openWindow({
        id: 'about-window',
        component: 'about-window',
        props: {
          windowId: 'about-window',
          title: 'About'
        },
        position: { x: 250, y: 200 },
        size: { width: 600, height: 500 }
      })
    }
  },
  {
    id: 'calculator',
    label: 'Calculator',
    emoji: '🧮',
    title: 'Calculator (Click to open)',
    action: () => {
      const { openWindow } = useWindowManager()
      openWindow({
        id: 'calculator-window',
        component: 'calculator-window',
        props: {
          windowId: 'calculator-window',
          title: 'Calculator'
        },
        position: { x: 300, y: 200 },
        size: { width: 320, height: 550 }
      })
    }
  },

  {
    id: 'music-player',
    label: 'Music\nPlayer',
    emoji: '🎵',
    title: 'Music Player (Click to open)',
    action: () => {
      const { openWindow } = useWindowManager()
      openWindow({
        id: 'music-player-window',
        component: 'music-player-window',
        props: {
          windowId: 'music-player-window',
          title: 'Music Player'
        },
        position: { x: 400, y: 150 },
        size: { width: 400, height: 600 }
      })
    }
  },
  {
    id: 'download',
    label: 'Download',
    emoji: '⬇️',
    title: 'Download SwitchShuttle (Click to open)',
    action: () => {
      const { openWindow } = useWindowManager()
      openWindow({
        id: 'download-window',
        component: 'download-window',
        props: {
          windowId: 'download-window',
          title: 'Download SwitchShuttle'
        },
        position: { x: 200, y: 120 },
        size: { width: 650, height: 500 }
      })
    }
  },
  {
    id: 'changelog',
    label: 'Changelog',
    emoji: '📝',
    title: 'Changelog (Click to open)',
    action: () => {
      const { openWindow } = useWindowManager()
      openWindow({
        id: 'changelog-window',
        component: 'changelog-window',
        props: {
          windowId: 'changelog-window',
          title: 'Changelog'
        },
        position: { x: 350, y: 150 },
        size: { width: 600, height: 500 }
      })
    }
  }
])
</script>

<style lang="scss" scoped>
.desktop-icons-grid {
  display: grid;
  grid-template-columns: repeat(10, 1fr);
  grid-template-rows: repeat(6, 1fr);
  grid-auto-flow: column;
  gap: 20px;
  padding: 40px;
  width: 100vw;
  position: absolute;
  top: 0;
  left: 0;
  z-index: 1;
}

.desktop-icon {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  padding: 10px;
  border-radius: 8px;
  transition: all 0.2s ease;
  user-select: none;
  
  &:hover {
    background: rgba(255, 255, 255, 0.1);
    transform: scale(1.05);
  }
  
  &:active {
    transform: scale(0.95);
  }
}

.icon-image {
  width: 64px;
  height: 64px;
  display: flex;
  align-items: center;
  justify-content: center;
  margin-bottom: 8px;
  
  span {
    font-size: 48px;
    line-height: 1;
  }
  
  .desktop-icon-img {
    width: 100%;
    height: 100%;
    object-fit: contain;
  }
}

.icon-label {
  font-size: 12px;
  color: white;
  text-align: center;
  line-height: 1.2;
  text-shadow: 1px 1px 2px rgba(0, 0, 0, 0.8);
  white-space: pre-line;
  max-width: 80px;
  word-wrap: break-word;
}

@media (max-width: 768px) {
  .desktop-icons-grid {
    grid-template-columns: repeat(4, 1fr);
    gap: 15px;
    padding: 20px;
  }
  
  .icon-image {
    width: 48px;
    height: 48px;
    
    span {
      font-size: 36px;
    }
  }
  
  .icon-label {
    font-size: 10px;
    max-width: 60px;
  }
}

@media (max-width: 480px) {
  .desktop-icons-grid {
    grid-template-columns: repeat(3, 1fr);
    gap: 10px;
    padding: 15px;
  }
  
  .icon-image {
    width: 40px;
    height: 40px;
    
    span {
      font-size: 32px;
    }
  }
  
  .icon-label {
    font-size: 9px;
    max-width: 50px;
  }
}
</style> 