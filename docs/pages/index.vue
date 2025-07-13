<template>
    <div id="demo-window">
      <div class="desktop">
        <!-- Desktop Icons Grid -->
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
              <img v-else-if="icon.image" :src="icon.image" :alt="icon.label" class="desktop-icon-img">
            </div>
            <div class="icon-label">{{ icon.label }}</div>
          </div>
        </div>
      </div>
      
          <!-- macOS Menu Bar -->
    <MenuBar 
      :show-switch-shuttle-icon="showSwitchShuttleIcon"
      @show-notification="openNotification"
      @show-help="openHelp"
      @show-terminal="executeTerminalCommand"
      @edit-config="openJsonEditor"
      @toggle-item="toggleItem"
      @show-help-window="openHelp"
      @show-about-window="openAbout"
      @show-homepage-window="openHomepage"
      @show-json-editor-window="openJsonEditor"
      @show-config-folder-window="openConfigFolder"
      @hide-menu-bar="hideMenuBar"
    >
      <template #right>
      </template>
    </MenuBar>
  
      <!-- Windows Area -->
      <div class="window-area">
        <component
          v-for="win in windows"
          :is="getComponent(win.component)"
          v-bind="win.props || {}"
          :key="win.id"
        >
          <template #default>
            <!-- Содержимое для каждого типа окна -->
            <ReadmeWindow v-if="win.component === 'readme-window'" />
            <TerminalWindow 
              v-else-if="win.component === 'terminal-window'" 
              :command="terminalCommand"
              :output="terminalOutput"
            />
            <BrowserWindow v-else-if="win.component === 'browser-window'" />
            <GalaxyGameWindow v-else-if="win.component === 'galaxy-game-window'" />
            <HelpWindow v-else-if="win.component === 'help-window'" />
            <AboutWindow v-else-if="win.component === 'about-window'" />
            <HomepageWindow v-else-if="win.component === 'homepage-window'" />
            <JsonEditorWindow v-else-if="win.component === 'config-editor-window'" :configFile="currentConfigFile" />
            <ConfigFolderWindow v-else-if="win.component === 'config-folder-window'" />
            <CalculatorWindow v-else-if="win.component === 'calculator-window'" />
            <MusicPlayerWindow v-else-if="win.component === 'music-player-window'" />
            <NotificationModal v-else-if="win.component === 'notification-modal'" :title="notificationTitle" :message="notificationMessage" :show="true" />
            <div v-else-if="win.component === 'div'">
              Тестовое окно: {{ win.id }}
            </div>
            <div v-else style="background: yellow; color: black; padding: 20px;">
              Неизвестное окно: {{ win.id }} ({{ typeof win.component }})
            </div>
          </template>
        </component>
      </div>
      
      <!-- TaskBar -->
      <TaskBar />
      
      <!-- Notification Components -->
      <NotificationPanel />
      <NotificationBubble />
    </div>
  </template>
  
  <script setup lang="ts">
  import { useWindowManager } from '~/composables/useWindowManager'
import { useNotifications } from '~/composables/useNotifications'
import MenuBar from '~/components/MenuBar.vue'
import TaskBar from '~/components/TaskBar.vue'
import Window from '~/components/Window.vue'
import TerminalWindow from '~/components/TerminalWindow.vue'
import NotificationModal from '~/components/NotificationModal.vue'
import HelpWindow from '~/components/HelpWindow.vue'
import AboutWindow from '~/components/AboutWindow.vue'
import HomepageWindow from '~/components/HomepageWindow.vue'
import JsonEditorWindow from '~/components/JsonEditorWindow.vue'
import ConfigFolderWindow from '~/components/ConfigFolderWindow.vue'
import CalculatorWindow from '~/components/CalculatorWindow.vue'
import MusicPlayerWindow from '~/components/MusicPlayerWindow.vue'
import ReadmeWindow from '~/components/ReadmeWindow.vue'
import NotificationPanel from '~/components/NotificationPanel.vue'
import NotificationBubble from '~/components/NotificationBubble.vue'
import { ref, onMounted, watch } from 'vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

const { windows, openWindow } = useWindowManager()

// Автоматическое создание окон на сервере
openWindow({
  id: 'readme-window',
  component: 'readme-window',
  props: {
    windowId: 'readme-window',
    title: 'README'
  },
  position: { x: 400, y: 120 },
  size: { width: 800, height: 600 }
})

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

const notificationTitle = ref('')
const notificationMessage = ref('')
const currentConfigFile = ref('config.json')
const showSwitchShuttleIcon = ref(true)

// Состояние для терминала
const terminalCommand = ref('')
const terminalOutput = ref('')

// Массив иконок рабочего стола
const desktopIcons = ref([
  {
    id: 'switchshuttle',
    label: 'SwitchShuttle',
    image: '/switchshuttle.svg',
    title: 'SwitchShuttle (Click to show menu bar icon)',
    action: showMenuBar
  },
  {
    id: 'galaxy-game',
    label: 'Galaxy\nGame',
    emoji: '🚀',
    title: 'Galaxy Game (Click to play)',
    action: openGalaxyGame
  },
  {
    id: 'readme',
    label: 'README',
    emoji: '📖',
    title: 'README (Click to open)',
    action: openReadme
  },
  {
    id: 'terminal',
    label: 'Terminal',
    emoji: '💻',
    title: 'Terminal (Click to open)',
    action: openTerminal
  },
  {
    id: 'browser',
    label: 'Browser',
    emoji: '🌐',
    title: 'Browser (Click to open)',
    action: openBrowser
  },
  {
    id: 'help',
    label: 'Help',
    emoji: '❓',
    title: 'Help (Click to open)',
    action: openHelp
  },
  {
    id: 'about',
    label: 'About',
    emoji: 'ℹ️',
    title: 'About (Click to open)',
    action: openAbout
  },
  {
    id: 'calculator',
    label: 'Calculator',
    emoji: '🧮',
    title: 'Calculator (Click to open)',
    action: openCalculator
  },
  {
    id: 'music-player',
    label: 'Music\nPlayer',
    emoji: '🎵',
    title: 'Music Player (Click to open)',
    action: openMusicPlayer
  }
])

// Функция для выполнения команд в терминале
function executeTerminalCommand(command: string, output: string) {
  console.log('Executing terminal command:', command, 'output:', output)
  
  // Проверяем, открыт ли терминал
  const terminalExists = windows.value.some(w => w.id === 'terminal-window')
  
  if (!terminalExists) {
    // Если терминал закрыт, сначала открываем его
    openTerminal()
    // Ждем немного, чтобы окно создалось, затем устанавливаем команду
    setTimeout(() => {
      // Сбрасываем предыдущие значения, чтобы watch сработал
      terminalCommand.value = ''
      terminalOutput.value = ''
      // Устанавливаем новые значения
      setTimeout(() => {
        terminalCommand.value = command
        terminalOutput.value = output
      }, 50)
    }, 200)
  } else {
    // Если терминал уже открыт, сбрасываем и устанавливаем заново
    terminalCommand.value = ''
    terminalOutput.value = ''
    setTimeout(() => {
      terminalCommand.value = command
      terminalOutput.value = output
    }, 50)
  }
}

// Функция для получения компонента по строковому идентификатору
function getComponent(componentId: string) {
  const componentMap: Record<string, any> = {
    'readme-window': Window,
    'terminal-window': Window,
    'browser-window': Window,
    'galaxy-game-window': Window,
    'help-window': Window,
    'about-window': Window,
    'homepage-window': Window,
    'config-editor-window': Window,
    'config-folder-window': Window,
    'calculator-window': Window,
    'music-player-window': Window,
    'notification-modal': Window,
    'div': 'div'
  }
  return componentMap[componentId] || 'div'
}

function openReadme() {
  openWindow({
    id: 'readme-window',
    component: 'readme-window',
    props: {
      windowId: 'readme-window',
      title: 'README'
    },
    position: { x: 400, y: 120 }, // Правее и выше
    size: { width: 800, height: 600 }
  })
}

function openTerminal() {
  openWindow({
    id: 'terminal-window',
    component: 'terminal-window',
    props: {
      windowId: 'terminal-window',
      title: 'iTerm2 — SwitchShuttle Demo'
    },
    position: { x: 120, y: 120 },
    size: { width: 700, height: 480 }
  })
}

function openBrowser() {
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

function openGalaxyGame() {
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

function openHelp() {
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

function openAbout() {
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

function openHomepage() {
  openWindow({
    id: 'homepage-window',
    component: 'homepage-window',
    props: {
      windowId: 'homepage-window',
      title: 'Homepage'
    },
    position: { x: 180, y: 160 },
    size: { width: 850, height: 650 }
  })
}

function openJsonEditor(configFile?: string) {
  openWindow({
    id: 'config-editor-window',
    component: 'config-editor-window',
    props: {
      windowId: 'config-editor-window',
      title: 'Config Editor'
    },
    position: { x: 220, y: 140 },
    size: { width: 800, height: 600 }
  })
}

function openConfigFolder() {
  openWindow({
    id: 'config-folder-window',
    component: 'config-folder-window',
    props: {
      windowId: 'config-folder-window',
      title: 'Config Folder'
    },
    position: { x: 280, y: 180 },
    size: { width: 700, height: 500 }
  })
}

function openCalculator() {
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

function openMusicPlayer() {
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

function openNotification(title: string, message: string) {
  // Используем новую систему уведомлений
  const { addNotification } = useNotifications()
  addNotification({
    title: title || t('demo.notifications.notAvailable'),
    message: message || t('demo.notifications.notAvailableMessage'),
    type: 'info'
  })
}

function showMenuBar() {
  // Проверяем, есть ли уже иконка SwitchShuttle в меню-баре
  if (showSwitchShuttleIcon.value) {
    // Если иконка уже видна, показываем уведомление
    const { addNotification } = useNotifications()
    addNotification({
      title: 'SwitchShuttle is already active',
      message: 'SwitchShuttle icon is already displayed in the menu bar',
      type: 'warning'
    })
  } else {
    // Если иконки нет, показываем её
    showSwitchShuttleIcon.value = true
  }
}

function hideMenuBar() {
  showSwitchShuttleIcon.value = false
}

function toggleItem(itemName: string) {
  // This function is no longer needed as toggle logic is handled by the component
  // Keeping it for now, but it will not have an effect on the state.
}
  </script>
  
  <style scoped>
  .desktop {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 74px; /* 34px для MenuBar + 40px для TaskBar */
    width: 100%;
    height: 100vh;
    user-select: none;
    transform: translate3d(0, 0, 0);
    background: url('/background.jpg') center center;
    background-size: cover;
    background-position: center center;
    display: flex;
    align-items: flex-start;
    justify-content: flex-start;
    padding: 0;
  }

  .desktop-icons-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, 100px);
    grid-template-rows: repeat(6, 100px);
    gap: 0;
    width: 100%;
    height: 100%;
    align-content: start;
    justify-content: start;
    padding: 60px 50px;
  }

  .desktop-icon {
    display: flex !important;
    flex-direction: column !important;
    align-items: center !important;
    justify-content: center !important;
    width: 80px;
    height: 80px;
    cursor: pointer;
    border-radius: 8px;
    transition: all 0.2s ease;
    user-select: none;
    padding: 0;
    box-sizing: border-box;
    min-height: 80px;
    position: relative;
  }

  .desktop-icon:hover {
    background: rgba(255, 255, 255, 0.1);
    transform: scale(1.05);
  }

  .desktop-icon:active {
    transform: scale(0.95);
  }

  .icon-image {
    font-size: 48px;
    margin-bottom: 8px;
    filter: drop-shadow(0 2px 4px rgba(0, 0, 0, 0.3));
    display: flex !important;
    align-items: center !important;
    justify-content: center !important;
    height: 48px;
    flex-shrink: 0;
    margin-bottom: 4px;
  }

  .desktop-icon-img {
    width: 48px;
    height: 48px;
    filter: drop-shadow(0 2px 4px rgba(0, 0, 0, 0.3));
  }

  .icon-label {
    font-size: 10px !important;
    color: white !important;
    text-align: center !important;
    font-weight: 500 !important;
    text-shadow: 0 1px 2px rgba(0, 0, 0, 0.8) !important;
    max-width: 100%;
    word-wrap: break-word;
    line-height: 1.1 !important;
    display: block !important;
    opacity: 1 !important;
    visibility: visible !important;
    position: relative !important;
    z-index: 1 !important;
    margin-top: 4px !important;
    white-space: pre-line !important;
  }
  
  #demo-window {
    position: relative;
    width: 100vw;
    height: 100vh;
    overflow: hidden;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  }
  
  .window-area {
    position: absolute;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    pointer-events: none;
    z-index: 1000;
  }
  
  .window-area > * {
    pointer-events: auto;
  }
  
  
  
  .config-editor {
    width: 100%;
    height: 100%;
    padding: 18px 20px 12px 20px;
    font-family: 'JetBrains Mono', 'Fira Mono', 'Menlo', 'Consolas', monospace;
    font-size: 15px;
    color: #1d1d1f;
    text-align: left;
    background: #f8f8fa;
  }
  
  .config-code {
    margin: 0;
    white-space: pre-wrap;
    word-break: break-word;
    text-align: left;
    background: none;
    border: none;
    font-family: inherit;
    font-size: inherit;
    color: inherit;
  }

  /* Responsive adjustments */
  @media (max-width: 768px) {
    .desktop {
      padding: 40px 30px;
    }
    
    .desktop-icons-grid {
      grid-template-columns: 100px;
      grid-auto-rows: 80px;
      gap: 15px;
    }
    
    .desktop-icon {
      width: 70px;
      height: 70px;
      min-height: 70px;
    }
    
    .icon-image {
      font-size: 36px;
      height: 36px;
      margin-bottom: 2px;
    }
    
    .desktop-icon-img {
      width: 36px;
      height: 36px;
    }
    
    .icon-label {
      font-size: 9px;
      opacity: 1;
      visibility: visible;
    }
  }
  </style> 