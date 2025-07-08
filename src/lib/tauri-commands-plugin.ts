import type { App } from 'vue';
import SwitchShuttleCommands from './tauri-commands';

/**
 * Vue плагин для глобального доступа к командам Tauri SwitchShuttle
 * Предоставляет типизированные методы для работы с бэкендом
 */
export const TauriCommandsPlugin = {
  install(app: App) {
    // Добавляем команды в глобальные свойства приложения
    app.config.globalProperties.$tauri = SwitchShuttleCommands;
    
    // Добавляем команды в provide/inject для Composition API
    app.provide('tauri', SwitchShuttleCommands);
    
    // Добавляем команды в глобальные свойства для Options API
    app.config.globalProperties.$commands = SwitchShuttleCommands;
  }
};

// Экспортируем для использования в main.ts
export default TauriCommandsPlugin;

// Типы для TypeScript поддержки
declare module 'vue' {
  interface ComponentCustomProperties {
    $tauri: typeof SwitchShuttleCommands;
    $commands: typeof SwitchShuttleCommands;
  }
}

// Типы для Composition API
export interface TauriInjectionKey {
  tauri: typeof SwitchShuttleCommands;
}

export const TAURI_KEY: TauriInjectionKey = {
  tauri: SwitchShuttleCommands
} as TauriInjectionKey; 