<template>
    <div id="demo-window">
      <div class="desktop">
        <!-- Desktop Icons -->
        <div class="desktop-icon readme-icon" @click="toggleReadmeWindow" title="README (Click to open)">
          <div class="icon-image">📖</div>
          <div class="icon-label">README</div>
        </div>
        <div class="desktop-icon terminal-icon" @click="toggleTerminalWindow" title="Terminal (Click to open)">
          <div class="icon-image">💻</div>
          <div class="icon-label">Terminal</div>
        </div>
        <div class="desktop-icon help-icon" @click="showHelpWindow" title="Help (Click to open)">
          <div class="icon-image">❓</div>
          <div class="icon-label">Help</div>
        </div>
        <div class="desktop-icon about-icon" @click="showAboutWindow" title="About (Click to open)">
          <div class="icon-image">ℹ️</div>
          <div class="icon-label">About</div>
        </div>
        <div class="desktop-icon switchshuttle-icon" @click="showMenuBar" title="SwitchShuttle (Click to show menu bar)">
          <div class="icon-image">
            <img src="/logo.svg" alt="SwitchShuttle" class="desktop-icon-img">
          </div>
          <div class="icon-label">SwitchShuttle</div>
        </div>
      </div>
      
          <!-- macOS Menu Bar -->
    <MenuBar 
      :show-switch-shuttle-icon="showSwitchShuttleIcon"
      @show-notification="showNotification"
      @show-help="showHelp"
      @show-terminal="showTerminal"
      @edit-config="editConfig"
      @toggle-item="toggleItem"
      @show-help-window="showHelpWindow"
      @show-about-window="showAboutWindow"
      @show-homepage-window="showHomepageWindow"
      @show-json-editor-window="showJsonEditorWindow"
      @show-config-folder-window="showConfigFolderWindow"
      @hide-menu-bar="hideMenuBar"
    >
      <template #right>
      </template>
    </MenuBar>
  
      <!-- Windows Area -->
      <div v-if="!readmePending" class="window-area">
        <TerminalWindow 
          v-if="showTerminalWindowModal"
          :title="terminalTitle"
          ref="terminalRef"
          @close="closeTerminalWindow"
        />
        
        <NotificationModal
          :show="showNotificationModal"
          :title="notificationTitle"
          :message="notificationMessage"
          @close="closeNotification"
        />
        
        <HelpWindow
          v-if="showHelpWindowModal"
          @close="showHelpWindowModal = false"
        />
        
        <AboutWindow
          v-if="showAboutWindowModal"
          @close="showAboutWindowModal = false"
        />
        
        <HomepageWindow
          v-if="showHomepageWindowModal"
          @close="showHomepageWindowModal = false"
        />
        
        <JsonEditorWindow
          v-if="showJsonEditorWindowModal"
          :config-file="currentConfigFile"
          @close="showJsonEditorWindowModal = false"
        />
        
        <ConfigFolderWindow
          v-if="showConfigFolderWindowModal"
          @close="showConfigFolderWindowModal = false"
          @open-file="openConfigFile"
        />
        
        <ReadmeWindow 
          v-if="showReadmeWindowModal && readmeData"
          :data="readmeData"
          @close="showReadmeWindowModal = false"
        />
        
        <Window v-if="showConfigEditor" :title="`${configFileName} — SwitchShuttle`" :initial-x="150" :initial-y="150" :z="1600" @close="showConfigEditor = false">
          <template #titlebar>
            <div class="window-title">{{ configFileName }} — SwitchShuttle</div>
          </template>
          <div class="config-editor">
            <pre class="config-code">{{ configContent }}</pre>
          </div>
        </Window>
      </div>
    </div>
  </template>
  
  <script setup lang="ts">
  import { ref, nextTick, onMounted, watch } from 'vue'
  import { useI18n } from 'vue-i18n'
import MenuBar from '~/components/MenuBar.vue'
import TerminalWindow from '~/components/TerminalWindow.vue'
import NotificationModal from '~/components/NotificationModal.vue'
import Window from '~/components/Window.vue'
import HelpWindow from '~/components/HelpWindow.vue'
import AboutWindow from '~/components/AboutWindow.vue'
import HomepageWindow from '~/components/HomepageWindow.vue'
import JsonEditorWindow from '~/components/JsonEditorWindow.vue'
import ConfigFolderWindow from '~/components/ConfigFolderWindow.vue'
import ReadmeWindow from '~/components/ReadmeWindow.vue'

// Types
interface ReadmeResponse {
  success: boolean
  content?: string
  toc?: Array<{id: string, title: string, level: number}>
  error?: string
  locale?: string
  fallback?: boolean
}

// Получаем данные README на странице
const { locale } = useI18n()

// Импортируем статические данные README
const readmeData = ref<ReadmeResponse | null>(null)
const readmePending = ref(false)

// Загружаем данные при изменении локали
watch(locale, async () => {
  readmePending.value = true
  try {
    // Динамический импорт JSON файла
    const data = await import(`../data/readme/${locale.value}.json`)
    readmeData.value = data.default
  } catch {
    // Fallback на английский если файл не найден
    const fallbackData = await import('../data/readme/en.json')
    readmeData.value = fallbackData.default
  } finally {
    readmePending.value = false
  }
}, { immediate: true })
  
  // Состояние окон
const showNotificationModal = ref(false)
const showHelpModal = ref(false)
const showHelpWindowModal = ref(false)
const showAboutWindowModal = ref(false)
const showHomepageWindowModal = ref(false)
const showJsonEditorWindowModal = ref(false)
const showConfigFolderWindowModal = ref(false)
const showConfigEditor = ref(false)
const showReadmeWindowModal = ref(true)
const showTerminalWindowModal = ref(true)
const showSwitchShuttleIcon = ref(true)
  
  // Уведомления
  const notificationTitle = ref('')
  const notificationMessage = ref('')
  
  // Терминал
  const terminalTitle = ref('iTerm2 — SwitchShuttle Demo')
  const terminalRef = ref(null)
  
  // Редактор конфигурации
  const configFileName = ref('')
  const configContent = ref('')
  const currentConfigFile = ref('config.json')
  
  // Состояние toggle элементов
  const toggleStates = ref({
    '🔧 System Monitoring': true,
    'Launch at Login': false
  })
  
  // Обработчики событий от MenuBar
  function showNotification(title: string, message: string) {
    notificationTitle.value = title
    notificationMessage.value = message
    showNotificationModal.value = true
  }
  
  function closeNotification() {
    showNotificationModal.value = false
  }
  
  function showHelp() {
    showHelpModal.value = true
  }
  
  function showHelpWindow() {
    showHelpWindowModal.value = true
  }
  
  function showAboutWindow() {
    showAboutWindowModal.value = true
  }
  
  function showHomepageWindow() {
    showHomepageWindowModal.value = true
  }
  
  function showJsonEditorWindow(configFile?: string) {
  showJsonEditorWindowModal.value = true
  // Можно передать имя файла в компонент, если нужно
  currentConfigFile.value = configFile || 'config.json'
}

function showConfigFolderWindow() {
  showConfigFolderWindowModal.value = true
}

function openConfigFile(fileName: string) {
  showConfigFolderWindowModal.value = false
  currentConfigFile.value = fileName
  showJsonEditorWindowModal.value = true
}
  
  async function showTerminal(command?: string, terminalOutput?: string) {
    // Открываем терминал, если он закрыт
    if (!showTerminalWindowModal.value) {
      showTerminalWindowModal.value = true
      // Ждем обновления DOM
      await nextTick()
    }
    
    // Обновляем заголовок если есть команда
    if (command) {
      terminalTitle.value = `Terminal — ${command}`
      
      // Если есть terminalOutput, добавляем его в терминал
      if (terminalOutput && terminalRef.value) {
        (terminalRef.value as any).executeCommandFromMenu(command, terminalOutput)
      }
    }
  }
  
  function editConfig(config: string) {
    configFileName.value = config
    configContent.value = `{
    "name": "${config}",
    "version": "1.0.0",
    "description": "Configuration file for SwitchShuttle",
    "commands": [
      {
        "name": "Development Server",
        "command": "npm run dev",
        "hotkey": "Cmd+D"
      },
      {
        "name": "Build Project",
        "command": "npm run build",
        "hotkey": "Cmd+B"
      }
    ],
    "settings": {
      "autoRefresh": true,
      "launchAtLogin": false,
      "systemMonitoring": true
    }
  }`
    showConfigEditor.value = true
  }
  
  function toggleItem(itemName: string) {
  if (toggleStates.value.hasOwnProperty(itemName)) {
    (toggleStates.value as any)[itemName] = !(toggleStates.value as any)[itemName]
  }
}

function toggleReadmeWindow() {
  showReadmeWindowModal.value = !showReadmeWindowModal.value
}

function toggleTerminalWindow() {
  showTerminalWindowModal.value = !showTerminalWindowModal.value
}

function closeTerminalWindow() {
  showTerminalWindowModal.value = false
  // Очищаем терминал при закрытии
  if (terminalRef.value) {
    (terminalRef.value as any).clearTerminal()
  }
}

function showMenuBar() {
  showSwitchShuttleIcon.value = true
}

function hideMenuBar() {
  showSwitchShuttleIcon.value = false
}
  </script>
  
  <style scoped>
  .desktop {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 34px;
    width: 100%;
    height: 100vh;
    user-select: none;
    transform: translate3d(0, 0, 0);
    background: url('/background.jpg') center center;
    background-size: cover;
    background-position: center center;
  }

  .desktop-icon {
    position: absolute;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    width: 80px;
    height: 80px;
    cursor: pointer;
    border-radius: 8px;
    transition: all 0.2s ease;
    user-select: none;
    /* Сетка: 100px между иконками по вертикали, 50px отступ слева */
  }

  .desktop-icon:hover {
    background: rgba(255, 255, 255, 0.1);
    transform: scale(1.05);
  }

  .desktop-icon:active {
    transform: scale(0.95);
  }

  /* Сетка иконок: вертикальное выравнивание с отступами */
  .readme-icon {
    top: 80px;
    left: 50px;
  }

  .terminal-icon {
    top: 180px;
    left: 50px;
  }

  .help-icon {
    top: 280px;
    left: 50px;
  }

  .about-icon {
    top: 380px;
    left: 50px;
  }

  .switchshuttle-icon {
    top: 480px;
    left: 50px;
  }

  .icon-image {
    font-size: 48px;
    margin-bottom: 8px;
    filter: drop-shadow(0 2px 4px rgba(0, 0, 0, 0.3));
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .desktop-icon-img {
    width: 48px;
    height: 48px;
    filter: drop-shadow(0 2px 4px rgba(0, 0, 0, 0.3));
  }

  .icon-label {
    font-size: 10px;
    color: white;
    text-align: center;
    font-weight: 500;
    text-shadow: 0 1px 2px rgba(0, 0, 0, 0.8);
    max-width: 100%;
    word-wrap: break-word;
    line-height: 1.1;
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
  </style> 