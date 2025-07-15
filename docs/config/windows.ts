import Window from '~/components/Window.vue'

// Централизованная конфигурация окон
export interface WindowConfig {
  id: string
  title: string
  icon: string
  component: any
  description?: string
}

export const windowConfigs: WindowConfig[] = [
  {
    id: 'readme-window',
    title: 'windows.readme',
    icon: '📖',
    component: Window,
    description: 'Documentation and guides'
  },
  {
    id: 'terminal-window',
    title: 'windows.terminal',
    icon: '💻',
    component: Window,
    description: 'Command line interface'
  },
  {
    id: 'browser-window',
    title: 'windows.browser',
    icon: '🌐',
    component: Window,
    description: 'Web browser'
  },
  {
    id: 'galaxy-game-window',
    title: 'windows.galaxyGame',
    icon: '🚀',
    component: Window,
    description: 'Space exploration game'
  },
  {
    id: 'help-window',
    title: 'windows.help',
    icon: '❓',
    component: Window,
    description: 'Help and support'
  },
  {
    id: 'about-window',
    title: 'windows.about',
    icon: 'ℹ️',
    component: Window,
    description: 'About SwitchShuttle'
  },
  {
    id: 'homepage-window',
    title: 'windows.homepage',
    icon: '🏠',
    component: Window,
    description: 'Official homepage'
  },
  {
    id: 'config-editor-window',
    title: 'windows.configEditor',
    icon: '⚙️',
    component: Window,
    description: 'Configuration editor'
  },
  {
    id: 'config-folder-window',
    title: 'windows.configFolder',
    icon: '📁',
    component: Window,
    description: 'Configuration folder'
  },
  {
    id: 'calculator-window',
    title: 'windows.calculator',
    icon: '🧮',
    component: Window,
    description: 'Calculator tool'
  },
  {
    id: 'music-player-window',
    title: 'windows.musicPlayer',
    icon: '🎵',
    component: Window,
    description: 'Music player'
  },
  {
    id: 'download-window',
    title: 'windows.download',
    icon: '⬇️',
    component: Window,
    description: 'Download SwitchShuttle'
  },
  {
    id: 'changelog-window',
    title: 'windows.changelog',
    icon: '📝',
    component: Window,
    description: 'Changelog and version history'
  },
  {
    id: 'notification-modal',
    title: 'windows.notification',
    icon: '🔔',
    component: Window,
    description: 'Notification center'
  }
]

// Создаем Map для быстрого доступа по ID
export const windowConfigMap = new Map<string, WindowConfig>(
  windowConfigs.map(config => [config.id, config])
)

// Функция для получения компонента по ID
export function getComponent(componentId: string) {
  const config = windowConfigMap.get(componentId)
  return config?.component || 'div'
}

// Функция для получения информации об окне по ID
export function getWindowInfo(windowId: string) {
  const config = windowConfigMap.get(windowId)
  if (!config) {
    return { title: 'windows.unknown', icon: '❓' }
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