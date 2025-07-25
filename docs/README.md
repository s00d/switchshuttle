# SwitchShuttle Demo - Nuxt Version

Демо-версия SwitchShuttle, реализованная на Nuxt 3 с поддержкой SSG и интернационализации.

## Особенности

- ✅ **Nuxt 3** - современный фреймворк для Vue.js
- ✅ **SSG (Static Site Generation)** - генерация статических HTML файлов
- ✅ **i18n** - поддержка 5 языков (en, ru, de, ja, zh)
- ✅ **Pinia** - управление состоянием
- ✅ **macOS-style UI** - интерфейс в стиле macOS
- ✅ **Перетаскиваемые окна** - терминал и уведомления
- ✅ **Системный мониторинг** - CPU и RAM в реальном времени

## Команды

```bash
# Установка зависимостей
pnpm install

# Разработка
pnpm dev

# Сборка
pnpm build

# Генерация статики
pnpm generate-static

# Предпросмотр
pnpm preview
```

## Структура проекта

```
demo-nuxt/
├── components/          # Vue компоненты
│   ├── TerminalWindow.vue
│   └── NotificationWindow.vue
├── pages/              # Страницы
│   ├── index.vue       # Редирект на /en
│   └── [locale].vue    # Основная страница с локализацией
├── locales/            # Файлы переводов
│   ├── en.json
│   ├── ru.json
│   ├── de.json
│   ├── ja.json
│   └── zh.json
├── assets/             # Статические ресурсы
│   └── css/
│       └── main.css
├── public/             # Публичные файлы
├── nuxt.config.ts      # Конфигурация Nuxt
└── generate-static.cjs # Скрипт генерации статики
```

## Локализация

Поддерживаемые языки:
- 🇺🇸 English (en)
- 🇷🇺 Русский (ru)
- 🇩🇪 Deutsch (de)
- 🇯🇵 日本語 (ja)
- 🇨🇳 中文 (zh)

## Деплой

Статические файлы генерируются в папку `docs/switchshuttle/` и включают:

- `/switchshuttle/en` - английская версия
- `/switchshuttle/ru` - русская версия
- `/switchshuttle/de` - немецкая версия
- `/switchshuttle/ja` - японская версия
- `/switchshuttle/zh` - китайская версия

Главная страница `docs/index.html` автоматически редиректит на локаль пользователя.

## Технологии

- **Nuxt 3** - Vue.js фреймворк
- **Vite** - сборщик
- **TypeScript** - типизация
- **Pinia** - управление состоянием
- **@nuxtjs/i18n** - интернационализация
- **@nuxt/image** - оптимизация изображений
- **@nuxt/eslint** - линтер

## Отличия от Vue версии

1. **SSG вместо SPA** - лучшая производительность и SEO
2. **Встроенная i18n** - более простая настройка локализации
3. **Автоматическая оптимизация** - изображения, CSS, JS
4. **Лучшая структура** - файловая система роутинга
5. **TypeScript из коробки** - полная поддержка типов
