import { invoke } from '@tauri-apps/api/core';

// Типы для структур данных
export interface CommandConfig {
  id?: string;
  name: string;
  inputs?: Record<string, string>;
  command?: string;
  commands?: string[];
  hotkey?: string;
  submenu?: CommandConfig[];
  switch?: string;
  monitor?: string;
  icon?: string;
}

export interface Config {
  terminal: string;
  launch_in: string;
  theme: string;
  title: string;
  commands: CommandConfig[];
  menu_hotkey?: string;
  enabled?: boolean;
}

// Универсальная структура для любых настроек
export interface AppSettings {
  [section: string]: Record<string, string | number | boolean>;
}

export interface GitHubRelease {
  tag_name: string;
  html_url: string;
}

export interface MenuItem {
  name: string;
  disabled: boolean;
  event: string;
  id: string;
  command?: string;
  hotkey: string;
  submenu?: MenuItem[];
}

export interface ConfigFile {
  name: string;
  path: string;
  size: number;
  modified: string;
}

export interface SettingsSchema {
  sections: Array<{
    id: string;
    title: string;
    description: string;
    fields: Array<{
      id: string;
      type: string;
      label: string;
      description: string;
      default?: any;
      min?: number;
      max?: number;
      options?: Array<{ value: string; label: string }>;
    }>;
  }>;
}

/**
 * @description Класс для работы с командами Tauri SwitchShuttle
 * Предоставляет типизированные методы для всех доступных команд
 */
export class SwitchShuttleCommands {

  /**
   * @description Открывает папку с конфигурациями в проводнике
   * @returns {Promise<void>} Promise, который разрешается при успешном открытии
   */
  static async open_config_folder(): Promise<void> {
    try {
      await invoke<void>('open_config_folder');
    } catch (error) {
      throw new Error(`Failed to open config folder: ${error}`);
    }
  }

  /**
   * @description Создает новый файл конфигурации
   * @param {string} file_name - Имя файла без расширения .json
   * @returns {Promise<void>} Promise, который разрешается при успешном создании
   */
  static async create_new_config(file_name: string): Promise<void> {
    try {
      await invoke<void>('create_new_config', { file_name });
    } catch (error) {
      throw new Error(`Failed to create new config: ${error}`);
    }
  }

  /**
   * @description Проверяет наличие обновлений приложения
   * @returns {Promise<{ message: string; url: string }>} Promise с информацией об обновлении
   */
  static async check_for_updates(): Promise<{ message: string; url: string }> {
    try {
      const result = await invoke<[string, string]>('check_for_updates');
      return {
        message: result[0],
        url: result[1]
      };
    } catch (error) {
      throw new Error(`Failed to check for updates: ${error}`);
    }
  }

  /**
   * @description Получает текущую версию приложения
   * @returns {Promise<string>} Promise с версией приложения
   */
  static async get_version(): Promise<string> {
    try {
      return await invoke<string>('get_version');
    } catch (error) {
      throw new Error(`Failed to get version: ${error}`);
    }
  }

  /**
   * @description Выполняет команду по ID
   * @param {string} command - ID команды для выполнения
   * @returns {Promise<string>} Promise с результатом выполнения
   */
  static async execute(command: string): Promise<string> {
    try {
      return await invoke<string>('execute', { command });
    } catch (error) {
      throw new Error(`Failed to execute command: ${error}`);
    }
  }

  /**
   * @description Выполняет команду с пользовательскими вводами
   * @param {Record<string, string>} inputs - Объект с вводами пользователя
   * @param {string} command - ID команды для выполнения
   * @returns {Promise<string>} Promise с результатом выполнения
   */
  static async execute_command_with_inputs(
    inputs: Record<string, string>,
    command: string
  ): Promise<string> {
    try {
      return await invoke<string>('execute_command_with_inputs', { inputs, command });
    } catch (error) {
      throw new Error(`Failed to execute command with inputs: ${error}`);
    }
  }

  /**
   * @description Получает данные меню для системного трея
   * @returns {Promise<Record<string, MenuItem[]>>} Promise с данными меню
   */
  static async get_menu_data(): Promise<Record<string, MenuItem[]>> {
    try {
      const result = await invoke<string>('get_menu_data');
      return JSON.parse(result);
    } catch (error) {
      throw new Error(`Failed to get menu data: ${error}`);
    }
  }

  /**
   * @description Получает данные вводов для команды
   * @param {string} command - ID команды
   * @returns {Promise<Record<string, string>>} Promise с данными вводов
   */
  static async fetch_input_data(command: string): Promise<Record<string, string>> {
    try {
      const result = await invoke<string>('fetch_input_data', { command });
      return JSON.parse(result);
    } catch (error) {
      throw new Error(`Failed to fetch input data: ${error}`);
    }
  }

  /**
   * @description Получает сообщение "О программе"
   * @returns {Promise<string>} Promise с HTML сообщением
   */
  static async about_message(): Promise<string> {
    try {
      return await invoke<string>('about_message');
    } catch (error) {
      throw new Error(`Failed to get about message: ${error}`);
    }
  }

  /**
   * @description Получает все конфигурации
   * @returns {Promise<Config[]>} Promise с массивом конфигураций
   */
  static async get_configurations(): Promise<Config[]> {
    try {
      return await invoke<Config[]>('get_configurations');
    } catch (error) {
      throw new Error(`Failed to get configurations: ${error}`);
    }
  }

  /**
   * @description Открывает конфигурацию в редакторе
   * @param {string} id - ID конфигурации
   * @returns {Promise<void>} Promise, который разрешается при успешном открытии
   */
  static async open_configuration(id: string): Promise<void> {
    try {
      await invoke<void>('open_configuration', { id });
    } catch (error) {
      throw new Error(`Failed to open configuration: ${error}`);
    }
  }

  /**
   * @description Удаляет конфигурацию
   * @param {string} id - ID конфигурации
   * @returns {Promise<void>} Promise, который разрешается при успешном удалении
   */
  static async delete_configuration(id: string): Promise<void> {
    try {
      await invoke<void>('delete_configuration', { id });
    } catch (error) {
      throw new Error(`Failed to delete configuration: ${error}`);
    }
  }

  /**
   * @description Сохраняет или обновляет конфигурацию
   * @param {Config} config - Конфигурация для сохранения
   * @param {string} originalTitle - Оригинальное название для обновления (опционально)
   * @returns {Promise<void>} Promise, который разрешается при успешном сохранении
   */
  static async save_or_update_configuration(
    config: Config,
    originalTitle?: string
  ): Promise<void> {
    try {
      await invoke<void>('save_or_update_configuration', { config, originalTitle });
    } catch (error) {
      throw new Error(`Failed to save or update configuration: ${error}`);
    }
  }

  /**
   * @description Получает уникальное название конфигурации
   * @param {string} base_title - Базовое название
   * @returns {Promise<string>} Promise с уникальным названием
   */
  static async get_unique_config_title(baseTitle: string): Promise<string> {
    try {
      return await invoke<string>('get_unique_config_title', { baseTitle });
    } catch (error) {
      throw new Error(`Failed to get unique config title: ${error}`);
    }
  }

  /**
   * @description Сохраняет конфигурацию по ID
   * @param {Config} config - Конфигурация для сохранения
   * @param {string} id - ID конфигурации
   * @returns {Promise<void>} Promise, который разрешается при успешном сохранении
   */
  static async save_configuration_by_id(config: Config, id: string): Promise<void> {
    try {
      await invoke<void>('save_configuration_by_id', { config, id });
    } catch (error) {
      throw new Error(`Failed to save configuration by ID: ${error}`);
    }
  }

  /**
   * @description Создает новую конфигурацию
   * @returns {Promise<Config>} Promise с новой конфигурацией
   */
  static async create_new_configuration(): Promise<Config> {
    try {
      return await invoke<Config>('create_new_configuration');
    } catch (error) {
      throw new Error(`Failed to create new configuration: ${error}`);
    }
  }

  /**
   * @description Дублирует конфигурацию
   * @param {Config} config - Конфигурация для дублирования
   * @returns {Promise<Config>} Promise с дублированной конфигурацией
   */
  static async duplicate_configuration(config: Config): Promise<Config> {
    try {
      return await invoke<Config>('duplicate_configuration', { config });
    } catch (error) {
      throw new Error(`Failed to duplicate configuration: ${error}`);
    }
  }

  /**
   * @description Валидирует конфигурацию
   * @param {Config} config - Конфигурация для валидации
   * @returns {Promise<void>} Promise, который разрешается при успешной валидации
   */
  static async validate_configuration(config: Config): Promise<void> {
    try {
      await invoke<void>('validate_configuration', { config });
    } catch (error) {
      throw new Error(`Failed to validate configuration: ${error}`);
    }
  }

  /**
   * @description Получает список файлов конфигураций
   * @returns {Promise<ConfigFile[]>} Promise с массивом файлов конфигураций
   */
  static async get_config_files(): Promise<ConfigFile[]> {
    try {
      return await invoke<ConfigFile[]>('get_config_files');
    } catch (error) {
      throw new Error(`Failed to get config files: ${error}`);
    }
  }

  /**
   * @description Загружает конфигурацию из файла
   * @param {string} path - Путь к файлу конфигурации
   * @returns {Promise<Config>} Promise с загруженной конфигурацией
   */
  static async load_config(path: string): Promise<Config> {
    try {
      return await invoke<Config>('load_config', { path });
    } catch (error) {
      throw new Error(`Failed to load config: ${error}`);
    }
  }

  /**
   * @description Обновляет конфигурации
   * @returns {Promise<void>} Promise, который разрешается при успешном обновлении
   */
  static async refresh_configurations(): Promise<void> {
    try {
      await invoke<void>('refresh_configurations');
    } catch (error) {
      throw new Error(`Failed to refresh configurations: ${error}`);
    }
  }

  /**
   * @description Получает схему настроек
   * @returns {Promise<SettingsSchema>} Promise со схемой настроек
   */
  static async get_settings_schema(): Promise<SettingsSchema> {
    try {
      return await invoke<SettingsSchema>('get_settings_schema');
    } catch (error) {
      throw new Error(`Failed to get settings schema: ${error}`);
    }
  }

  /**
   * @description Получает текущие настройки приложения
   * @returns {Promise<AppSettings>} Promise с настройками
   */
  static async get_settings(): Promise<AppSettings> {
    try {
      return await invoke<AppSettings>('get_settings');
    } catch (error) {
      throw new Error(`Failed to get settings: ${error}`);
    }
  }

  /**
   * @description Сохраняет настройки приложения
   * @param {AppSettings} settings - Настройки для сохранения
   * @returns {Promise<void>} Promise, который разрешается при успешном сохранении
   */
  static async save_settings(settings: AppSettings): Promise<void> {
    try {
      await invoke<void>('save_settings', { settings });
    } catch (error) {
      throw new Error(`Failed to save settings: ${error}`);
    }
  }

  /**
   * @description Показывает уведомление указанного типа
   * @param {string} title - Заголовок уведомления
   * @param {string} body - Текст уведомления
   * @param {'default' | 'success' | 'error' | 'info' | 'warning'} notificationType - Тип уведомления
   * @returns {Promise<void>} Promise, который разрешается при успешном показе
   */
  static async show_notification(
    title: string, 
    body: string, 
    notificationType: 'default' | 'success' | 'error' | 'info' | 'warning' = 'default'
  ): Promise<void> {
    try {
      await invoke<void>('show_notification', { title, body, notificationType });
    } catch (error) {
      throw new Error(`Failed to show ${notificationType} notification: ${error}`);
    }
  }

  /**
   * @description Показывает уведомление об успехе
   * @param {string} title - Заголовок уведомления
   * @param {string} body - Текст уведомления
   * @returns {Promise<void>} Promise, который разрешается при успешном показе
   */
  static async show_success_notification(title: string, body: string): Promise<void> {
    return this.show_notification(title, body, 'success');
  }

  /**
   * @description Показывает уведомление об ошибке
   * @param {string} title - Заголовок уведомления
   * @param {string} body - Текст уведомления
   * @returns {Promise<void>} Promise, который разрешается при успешном показе
   */
  static async show_error_notification(title: string, body: string): Promise<void> {
    return this.show_notification(title, body, 'error');
  }

  /**
   * @description Показывает информационное уведомление
   * @param {string} title - Заголовок уведомления
   * @param {string} body - Текст уведомления
   * @returns {Promise<void>} Promise, который разрешается при успешном показе
   */
  static async show_info_notification(title: string, body: string): Promise<void> {
    return this.show_notification(title, body, 'info');
  }

  /**
   * @description Показывает предупреждающее уведомление
   * @param {string} title - Заголовок уведомления
   * @param {string} body - Текст уведомления
   * @returns {Promise<void>} Promise, который разрешается при успешном показе
   */
  static async show_warning_notification(title: string, body: string): Promise<void> {
    return this.show_notification(title, body, 'warning');
  }
}

// Экспортируем экземпляр по умолчанию для удобства использования
export default SwitchShuttleCommands; 