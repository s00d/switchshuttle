<template>
    <div id="demo-window">
      <div class="desktop">
        <DesktopIcons />
      </div>

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
            <DownloadWindow v-else-if="win.component === 'download-window'" />
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
import DesktopIcons from '~/components/DesktopIcons.vue'
import DownloadWindow from '~/components/DownloadWindow.vue'
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { getComponent } from '~/config/windows'

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
  position: { x: 700, y: 120 },
  size: { width: 600, height: 500 }
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

// Функция getComponent теперь импортируется из конфигурации окон

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

function openHelp() {
  openWindow({
    id: 'help-window',
    component: 'help-window',
    props: {
      windowId: 'help-window',
      title: 'Help'
    },
    position: { x: 300, y: 180 },
    size: { width: 600, height: 500 }
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
  // Если configFile не передан, используем дефолтный файл
  const fileToOpen = configFile || 'switch-shuttle.json'
  
  openWindow({
    id: 'config-editor-window',
    component: 'config-editor-window',
    props: {
      windowId: 'config-editor-window',
      title: `Config Editor - ${fileToOpen}`,
      configFile: fileToOpen
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

function openNotification(title: string, message: string) {
  // Используем новую систему уведомлений
  const { addNotification } = useNotifications()
  addNotification({
    title: title || t('demo.notifications.notAvailable'),
    message: message || t('demo.notifications.notAvailableMessage'),
    type: 'info'
  })
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