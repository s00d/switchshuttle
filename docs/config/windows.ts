// Централизованная конфигурация окон
export interface WindowConfig {
  id: string
  title: string
  icon: string
  component: string // Имя компонента для динамического импорта
  description?: string
  props?: Record<string, any> // Дополнительные пропсы
}

export const windowConfigs: WindowConfig[] = [
  {
    id: 'readme-window',
    title: 'Readme',
    icon: '📖',
    component: 'ReadmeWindow',
    description: 'Documentation and guides'
  },
  {
    id: 'terminal-window',
    title: 'Terminal',
    icon: '💻',
    component: 'TerminalWindow',
    description: 'Command line interface',
    props: { command: '', output: '' }
  },
  {
    id: 'browser-window',
    title: 'Browser',
    icon: '🌐',
    component: 'BrowserWindow',
    description: 'Web browser'
  },
  {
    id: 'galaxy-game-window',
    title: 'GalaxyGame',
    icon: '🚀',
    component: 'GalaxyGameWindow',
    description: 'Space exploration game'
  },
  {
    id: 'help-window',
    title: 'Help',
    icon: '❓',
    component: 'HelpWindow',
    description: 'Help and support'
  },
  {
    id: 'about-window',
    title: 'About',
    icon: 'ℹ️',
    component: 'AboutWindow',
    description: 'About SwitchShuttle'
  },
  {
    id: 'homepage-window',
    title: 'Homepage',
    icon: '🏠',
    component: 'HomepageWindow',
    description: 'Official homepage'
  },
  {
    id: 'config-editor-window',
    title: 'ConfigEditor',
    icon: '⚙️',
    component: 'JsonEditorWindow',
    description: 'Configuration editor',
    props: { configFile: 'config.json' }
  },
  {
    id: 'config-folder-window',
    title: 'ConfigFolder',
    icon: '📁',
    component: 'ConfigFolderWindow',
    description: 'Configuration folder'
  },
  {
    id: 'calculator-window',
    title: 'Calculator',
    icon: '🧮',
    component: 'CalculatorWindow',
    description: 'Calculator tool'
  },
  {
    id: 'music-player-window',
    title: 'MusicPlayer',
    icon: '🎵',
    component: 'MusicPlayerWindow',
    description: 'Music player'
  },
  {
    id: 'download-window',
    title: 'Download',
    icon: '⬇️',
    component: 'DownloadWindow',
    description: 'Download SwitchShuttle'
  },
  {
    id: 'changelog-window',
    title: 'Changelog',
    icon: '📝',
    component: 'ChangelogWindow',
    description: 'Changelog and version history'
  },
  {
    id: 'notification-modal',
    title: 'Notification',
    icon: '🔔',
    component: 'NotificationModal',
    description: 'Notification center',
    props: { title: '', message: '', show: true }
  }
]

// Создаем Map для быстрого доступа по ID
export const windowConfigMap = new Map<string, WindowConfig>(
  windowConfigs.map(config => [config.id, config])
)

// Функция для получения конфигурации окна по ID
export function getWindowConfig(windowId: string): WindowConfig | undefined {
  return windowConfigMap.get(windowId)
}

// Функция для получения информации об окне по ID
export function getWindowInfo(windowId: string) {
  const config = windowConfigMap.get(windowId)
  if (!config) {
    return { title: 'Unknown', icon: '❓' }
  }
  return {
    title: config.title,
    icon: config.icon
  }
}

// Функция для получения всех конфигураций окон
export function getAllWindowConfigs() {
  return windowConfigs
}

// Функция для получения списка окон для TaskBar
export function getTaskBarWindows() {
  return windowConfigs
    .filter(config => config.id !== 'notification-modal') // Исключаем модальные окна
    .map(config => ({
      id: config.id,
      title: config.title,
      icon: config.icon
    }))
}

// Функция для открытия окна по ID
export function createWindowConfig(windowId: string, customProps?: Record<string, any>) {
  const config = windowConfigMap.get(windowId)
  if (!config) {
    throw new Error(`Window ${windowId} not found`)
  }
  
  // Разделяем пропсы на те, что идут в WindowInfo и те, что в props
  const { position, size, ...otherProps } = customProps || {}
  
  return {
    id: windowId,
    component: windowId,
    position,
    size,
    props: {
      windowId,
      title: config.title, // Ключ для перевода (например, 'windows.readme')
      ...config.props,
      ...otherProps
    }
  }
} 