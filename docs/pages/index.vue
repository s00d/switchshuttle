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
      @show-changelog-window="openChangelog"
      @hide-menu-bar="hideMenuBar"
    >
      <template #right>
      </template>
    </MenuBar>

    <!-- Windows Area -->
    <div class="window-area">
      <Window
        v-for="win in windows"
        :key="win.id"
        :windowId="win.id"
        v-bind="{ ...win.props, title: t(win.props?.title) }"
      >
        <component
          :is="getWindowComponent(win.component)"
          v-bind="getWindowProps(win)"
        />
      </Window>
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
import NotificationPanel from '~/components/NotificationPanel.vue'
import NotificationBubble from '~/components/NotificationBubble.vue'
import DesktopIcons from '~/components/DesktopIcons.vue'
import { ref, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { createWindowConfig, getWindowConfig } from '~/config/windows'

const { t } = useI18n()

const { windows, openWindow } = useWindowManager()

// Автоматическое создание окон на сервере
openWindow(createWindowConfig('readme-window', {
  position: { x: 700, y: 120 },
  size: { width: 600, height: 500 }
}))

openWindow(createWindowConfig('terminal-window', {
  position: { x: 120, y: 120 },
  size: { width: 500, height: 380 }
}))

const notificationTitle = ref('')
const notificationMessage = ref('')
const currentConfigFile = ref('config.json')
const showSwitchShuttleIcon = ref(true)

// Состояние для терминала
const terminalCommand = ref('')
const terminalOutput = ref('')

// Динамический импорт компонентов
const componentCache = new Map()

function getWindowComponent(componentId: string) {
  if (componentCache.has(componentId)) {
    return componentCache.get(componentId)
  }

  // Специальные случаи
  if (componentId === 'terminal-window') {
    const component = defineAsyncComponent(() => import('~/components/TerminalWindow.vue'))
    componentCache.set(componentId, component)
    return component
  }

  if (componentId === 'notification-modal') {
    const component = defineAsyncComponent(() => import('~/components/NotificationModal.vue'))
    componentCache.set(componentId, component)
    return component
  }

  // Автоматический импорт по имени компонента
  const config = getWindowConfig(componentId)
  if (!config) {
    console.warn(`Window component not found: ${componentId}`)
    return 'div'
  }

  try {
    const component = defineAsyncComponent(() => import(`~/components/${config.component}.vue`))
    componentCache.set(componentId, component)
    return component
  } catch (error) {
    console.error(`Failed to load component ${config.component}:`, error)
    return 'div'
  }
}

function getWindowProps(win: any) {
  const config = getWindowConfig(win.component)
  if (!config) {
    return win.props || {}
  }

  // Специальные пропсы для терминала
  if (win.component === 'terminal-window') {
    return {
      ...win.props,
      command: terminalCommand.value,
      output: terminalOutput.value
    }
  }

  // Специальные пропсы для notification-modal
  if (win.component === 'notification-modal') {
    return {
      ...win.props,
      title: notificationTitle.value,
      message: notificationMessage.value,
      show: true
    }
  }

  // Специальные пропсы для config-editor-window
  if (win.component === 'config-editor-window') {
    return {
      ...win.props,
      configFile: currentConfigFile.value
    }
  }

  return win.props || {}
}

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

function openTerminal() {
  openWindow(createWindowConfig('terminal-window', {
    position: { x: 120, y: 120 },
    size: { width: 700, height: 480 }
  }))
}

function openHelp() {
  openWindow(createWindowConfig('help-window', {
    position: { x: 300, y: 180 },
    size: { width: 600, height: 500 }
  }))
}

function openAbout() {
  openWindow(createWindowConfig('about-window', {
    position: { x: 250, y: 200 },
    size: { width: 600, height: 500 }
  }))
}

function openHomepage() {
  openWindow(createWindowConfig('homepage-window', {
    position: { x: 180, y: 160 },
    size: { width: 850, height: 650 }
  }))
}

function openJsonEditor(configFile?: string) {
  const fileToOpen = configFile || 'switch-shuttle.json'
  currentConfigFile.value = fileToOpen
  
  openWindow(createWindowConfig('config-editor-window', {
    position: { x: 220, y: 140 },
    size: { width: 800, height: 600 },
    configFile: fileToOpen
  }))
}

function openConfigFolder() {
  openWindow(createWindowConfig('config-folder-window', {
    position: { x: 280, y: 180 },
    size: { width: 700, height: 500 }
  }))
}

function openChangelog() {
  openWindow(createWindowConfig('changelog-window', {
    position: { x: 300, y: 150 },
    size: { width: 800, height: 600 }
  }))
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