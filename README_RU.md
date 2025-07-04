<div align="center">
  
  [![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Windows%20%7C%20Linux-blue?style=for-the-badge)](https://github.com/s00d/switchshuttle)
  [![License](https://img.shields.io/badge/license-MIT-green?style=for-the-badge)](LICENSE)
  [![GitHub release](https://img.shields.io/github/v/release/s00d/switchshuttle?style=for-the-badge)](https://github.com/s00d/switchshuttle/releases)
  [![GitHub downloads](https://img.shields.io/github/downloads/s00d/switchshuttle/total?style=for-the-badge)](https://github.com/s00d/switchshuttle/releases)
  [![GitHub issues](https://img.shields.io/github/issues/s00d/switchshuttle?style=for-the-badge)](https://github.com/s00d/switchshuttle/issues)
  [![GitHub stars](https://img.shields.io/github/stars/s00d/switchshuttle?style=for-the-badge)](https://github.com/s00d/switchshuttle/stargazers)
  [![Donate](https://img.shields.io/badge/Donate-Donationalerts-ff4081?style=for-the-badge)](https://www.donationalerts.com/r/s00d88)

  <img src="https://raw.githubusercontent.com/s00d/switchshuttle/refs/heads/main/icons/logo-min.png" alt="SwitchShuttle Logo" width="200">
  
  # SwitchShuttle
  
  **🚀 Кроссплатформенный менеджер терминальных команд с глобальными горячими клавишами**

  <img src="https://github.com/s00d/switchshuttle/blob/main/icons/intro.gif?raw=true" alt="SwitchShuttle Demo" width="600">
  
  <div class="locale">
    <a href="README.md">🇺🇸 English</a> • 
    <a href="README_ZH.md">🇨🇳 中文</a> • 
    <a href="README_RU.md">🇷🇺 Русский</a> • 
    <a href="README_DE.md">🇩🇪 Deutsch</a> • 
    <a href="README_JA.md">🇯🇵 日本語</a>
  </div>
</div>

---

## ✨ Что такое SwitchShuttle?

SwitchShuttle — это мощное кроссплатформенное приложение для системного трея, которое революционизирует способ управления и выполнения терминальных команд. Построенное на современных технологиях (Rust + Tauri + Vue.js), оно предоставляет элегантный интерфейс для организации, настройки и быстрого доступа к наиболее часто используемым терминальным операциям.

### 🎯 Ключевые возможности

- **🖥️ Мультиплатформенная поддержка** - Работает на macOS, Windows и Linux
- **⚡ Глобальные горячие клавиши** - Выполняйте команды мгновенно из любого места
- **🎨 Поддержка множества терминалов** - iTerm, Terminal, Warp, Alacritty, Hyper и другие
- **📁 Умная организация** - Создавайте вложенные подменю для лучшей организации команд
- **🔧 Динамические входные данные** - Интерактивные запросы для команд, требующих ввода пользователя
- **🔄 Множественные режимы выполнения** - Запуск в текущем окне, новой вкладке или новом окне
- **🚀 Автозапуск** - Запуск при старте системы для мгновенного доступа
- **🎨 Современный интерфейс** - Красивый, интуитивный интерфейс на Vue.js
- **💻 Командная строка** - Выполняйте команды напрямую из терминала с CLI
- **⚙️ Управление конфигурациями** - Включайте/выключайте конфигурации без удаления
- **🔄 Команды-переключатели** - Переключайте системные функции с фоновым выполнением

## 🚀 Быстрый старт

### Скачивание и установка

#### Вариант 1: Homebrew (macOS - Рекомендуется)
```bash
# Установка через Homebrew
brew tap s00d/switchshuttle
brew install --cask switchshuttle
```

#### Вариант 2: Ручное скачивание
1. **Скачайте** последний релиз для вашей платформы с [GitHub Releases](https://github.com/s00d/switchshuttle/releases)
2. **Установите** приложение
3. **Запустите** SwitchShuttle - он появится в системном трее
4. **Щелкните правой кнопкой** по иконке трея для доступа к меню

### Первая настройка

1. **Редактировать конфигурацию** → Открывает файл конфигурации в вашем редакторе
2. **Добавьте ваши команды** используя JSON формат (см. примеры ниже)
3. **Сохраните и перезапустите** приложение
4. **Наслаждайтесь** организованными ярлыками команд!

## 📋 Руководство по настройке

### Базовая структура

SwitchShuttle использует JSON файлы конфигурации, хранящиеся в:
- **macOS/Linux**: `~/.config/switch-shuttle/`
- **Windows**: `C:\Users\<Username>\AppData\Roaming\switch-shuttle\`

### Простой пример

```json
{
  "terminal": "iterm",
  "launch_in": "new_tab",
  "title": "Мои команды",
  "commands": [
    {
      "name": "🚀 Запустить сервер разработки",
      "command": "npm run dev",
      "hotkey": "Ctrl+Shift+D"
    },
    {
      "name": "📦 Установить зависимости",
      "command": "npm install",
      "hotkey": "Ctrl+Shift+I"
    },
    {
      "name": "🔧 Инструменты разработки",
      "submenu": [
        {
          "name": "🧪 Запустить тесты",
          "command": "npm test",
          "hotkey": "Ctrl+Shift+T"
        },
        {
          "name": "📊 Собрать проект",
          "command": "npm run build",
          "hotkey": "Ctrl+Shift+B"
        }
      ]
    }
  ]
}
```

### Продвинутые возможности

#### 🔧 Динамические входные данные

Создавайте интерактивные команды с запросом ввода пользователя:

```json
{
  "name": "📝 Создать новый компонент",
  "inputs": {
    "componentName": "MyComponent",
    "componentType": "functional"
  },
  "commands": [
    "mkdir -p src/components/[componentName]",
    "touch src/components/[componentName]/index.tsx",
    "echo 'import React from \"react\";' > src/components/[componentName]/index.tsx",
    "echo 'export const [componentName] = () => <div>[componentName]</div>;' >> src/components/[componentName]/index.tsx"
  ],
  "hotkey": "Ctrl+Shift+N"
}
```

#### 🔄 Множественные команды

Выполняйте последовательность команд:

```json
{
  "name": "🔄 Полный цикл разработки",
  "commands": [
    "git pull origin main",
    "npm install",
    "npm run lint",
    "npm run test",
    "npm run build"
  ],
  "hotkey": "Ctrl+Shift+F"
}
```

#### 📁 Вложенные подменю

Организуйте команды в иерархических меню:

```json
{
  "name": "🐳 Операции Docker",
  "submenu": [
    {
      "name": "🚀 Запустить сервисы",
      "submenu": [
        {
          "name": "🏗️ Разработка",
          "command": "docker-compose -f docker-compose.dev.yml up -d"
        },
        {
          "name": "🏭 Продакшн",
          "command": "docker-compose -f docker-compose.prod.yml up -d"
        }
      ]
    },
    {
      "name": "🛑 Остановить все",
      "command": "docker-compose down"
    }
  ]
}
```

#### 🔄 Команды-переключатели

Переключайте системные функции с фоновым выполнением:

```json
{
  "name": "🔧 Системные элементы управления",
  "submenu": [
    {
      "name": "📶 Переключить WiFi",
      "command": "networksetup -setairportpower en0 toggle",
      "switch": "networksetup -getairportpower en0 | grep -q 'On' && echo 'true' || echo 'false'"
    },
    {
      "name": "🔊 Переключить Bluetooth",
      "command": "blueutil -p toggle",
      "switch": "blueutil -p | grep -q '1' && echo 'true' || echo 'false'"
    },
    {
      "name": "🌙 Переключить темный режим",
      "command": "osascript -e 'tell app \"System Events\" to tell appearance preferences to set dark mode to not dark mode'",
      "switch": "osascript -e 'tell app \"System Events\" to tell appearance preferences to get dark mode'"
    }
  ]
}
```

**Возможности команд-переключателей:**
- **Фоновое выполнение** - Команды выполняются тихо без открытия терминала
- **Проверка статуса** - Автоматически определяет текущее состояние
- **Визуальная обратная связь** - Показывает статус включено/выключено в меню
- **Кроссплатформенность** - Работает на macOS, Windows и Linux

## ⚙️ Справочник по конфигурации

### Основная конфигурация

| Параметр | Тип | Описание | По умолчанию |
|----------|-----|----------|--------------|
| `terminal` | String | Терминальное приложение для использования | `"terminal"` |
| `launch_in` | String | Где запускать команды | `"current"` |
| `theme` | String | Тема терминала (если поддерживается) | - |
| `title` | String | Заголовок окна/вкладки | - |
| `menu_hotkey` | String | Глобальная горячая клавиша для открытия меню | - |
| `commands` | Array | Список конфигураций команд | `[]` |
| `enabled` | Boolean | Должна ли эта конфигурация загружаться | `true` |

### Опции терминалов

| Терминал | macOS | Windows | Linux |
|----------|-------|---------|-------|
| `iterm` | ✅ | ❌ | ❌ |
| `terminal` | ✅ | ✅ | ✅ |
| `warp` | ✅ | ❌ | ❌ |
| `alacritty` | ✅ | ✅ | ✅ |
| `hyper` | ✅ | ✅ | ✅ |

### Режимы запуска

| Режим | Описание |
|-------|----------|
| `current` | Выполнить в текущем окне терминала |
| `new_tab` | Открыть новую вкладку и выполнить |
| `new_window` | Открыть новое окно и выполнить |

### Конфигурация команд

| Параметр | Тип | Обязательный | Описание |
|----------|-----|--------------|----------|
| `name` | String | ✅ | Отображаемое имя команды |
| `command` | String | ❌ | Одиночная команда для выполнения |
| `commands` | Array | ❌ | Множественные команды для выполнения |
| `submenu` | Array | ❌ | Вложенные подкоманды |
| `inputs` | Object | ❌ | Динамические поля ввода |
| `hotkey` | String | ❌ | Глобальная горячая клавиша |
| `switch` | String | ❌ | Команда для проверки текущего статуса (для команд-переключателей) |

### Управление конфигурациями

#### Включение/выключение конфигураций

Вы можете включать или выключать отдельные файлы конфигурации для управления тем, какие команды доступны в меню системного трея. Это полезно для:

- **Временного отключения** - Отключайте конфигурации без удаления
- **Тестирования** - Включайте/выключайте конфигурации во время разработки
- **Организации** - Храните множество конфигураций, но используйте только определенные

**В визуальном редакторе:**
- Откройте редактор конфигураций
- Используйте переключатель в разделе "Статус конфигурации"
- Включенные конфигурации будут загружены и доступны в меню
- Отключенные конфигурации будут проигнорированы

**В JSON конфигурации:**
```json
{
  "terminal": "iterm",
  "launch_in": "current",
  "title": "Мои команды",
  "enabled": true,
  "commands": [
    {
      "name": "Пример команды",
      "command": "echo Hello World"
    }
  ]
}
```

| Параметр | Тип | По умолчанию | Описание |
|----------|-----|--------------|----------|
| `enabled` | Boolean | `true` | Должна ли эта конфигурация загружаться и быть доступной в меню |

**Примечание:** Когда `enabled` установлен в `false` или опущен, конфигурация будет проигнорирована и её команды не появятся в меню системного трея.

## 🎯 Случаи использования

### 👨‍💻 Разработчики
- **Быстрая навигация по проектам** - Мгновенный переход между проектами
- **Рабочие процессы сборки и тестирования** - Циклы разработки в один клик
- **Управление Docker** - Запуск/остановка контейнеров горячими клавишами
- **Git операции** - Общие git команды под рукой

### 🛠️ DevOps инженеры
- **Управление серверами** - SSH соединения и серверные команды
- **Инструменты мониторинга** - Быстрый доступ к логам и метрикам
- **Скрипты развертывания** - Автоматизированные рабочие процессы развертывания
- **Операции с базами данных** - Общие команды базы данных

### 🎨 Дизайнеры
- **Оптимизация ресурсов** - Обработка и оптимизация изображений
- **Инструменты дизайн-системы** - Генерация и обновление компонентов
- **Серверы прототипов** - Быстрый запуск сервера дизайна

## 🔧 Сборка из исходников

### Требования

- [Rust](https://www.rust-lang.org/tools/install) (последняя стабильная версия)
- [Node.js](https://nodejs.org/) (v16 или выше)
- [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites/)

### Шаги сборки

```bash
# Клонировать репозиторий
git clone https://github.com/s00d/switchshuttle.git
cd switchshuttle

# Установить зависимости
npm install

# Режим разработки
npm run tauri dev

# Сборка для продакшна
npm run tauri build
```

### Заметки по платформам

#### macOS
```bash
# Если возникают проблемы с подписью
chmod +x /Applications/switch-shuttle.app
xattr -cr /Applications/switch-shuttle.app
codesign --force --deep --sign - /Applications/switch-shuttle.app
```

## 🤝 Участие в разработке

Мы приветствуем вклад! Вот как вы можете помочь:

1. **Форкните** репозиторий
2. **Создайте** ветку функции (`git checkout -b feature/amazing-feature`)
3. **Зафиксируйте** ваши изменения (`git commit -m 'Add amazing feature'`)
4. **Отправьте** в ветку (`git push origin feature/amazing-feature`)
5. **Откройте** Pull Request

### Рекомендации по разработке

- Следуйте существующему стилю кода
- Добавляйте тесты для новых функций
- Обновляйте документацию по мере необходимости
- Обеспечивайте кроссплатформенную совместимость

## 📄 Лицензия

Этот проект лицензирован под MIT License - см. файл [LICENSE](LICENSE) для деталей.

## 🙏 Благодарности

- Вдохновлен оригинальным проектом [Shuttle](https://github.com/fitztrev/shuttle)
- Построен с [Tauri](https://tauri.app/) для кроссплатформенных десктопных приложений
- UI работает на [Vue.js](https://vuejs.org/)

## 📞 Поддержка

- **Issues**: [GitHub Issues](https://github.com/s00d/switchshuttle/issues)
- **Releases**: [GitHub Releases](https://github.com/s00d/switchshuttle/releases)

---

<div align="center">
  <p>Сделано с ❤️ сообществом SwitchShuttle</p>
  <p>⭐ Поставьте звезду этому репозиторию, если он вам полезен!</p>
</div>
