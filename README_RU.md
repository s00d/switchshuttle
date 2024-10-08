![SwitchShuttle](https://raw.githubusercontent.com/s00d/switchshuttle/main/icons/logo.webp)
![intro](https://github.com/s00d/switchshuttle/blob/main/icons/intro.gif?raw=true)

<span class="locale">
  <a href="README.md">English</a> | 
  <a href="README_ZH.md">中文</a> | 
  <a href="README_RU.md">Русский</a> | 
  <a href="README_DE.md">Deutsch</a> | 
  <a href="README_JA.md">日本語</a>
</span>

## SwitchShuttle

SwitchShuttle — это кроссплатформенное приложение для системного трея, которое позволяет пользователям запускать предопределенные команды в различных терминальных приложениях. Оно поддерживает macOS, Windows и Linux, предлагая простой и настраиваемый способ управления и выполнения ваших часто используемых команд.

## О проекте

SwitchShuttle — это переосмысление и расширение приложения [Shuttle](https://github.com/fitztrev/shuttle). В то время как Shuttle предоставляет простой и эффективный способ управления ярлыками команд в macOS, SwitchShuttle расширяет эту концепцию, предлагая поддержку нескольких операционных систем и терминальных эмуляторов, а также улучшенные возможности конфигурации и настройки пользователем.

## Возможности

- Поддержка нескольких терминальных приложений: iTerm, Terminal, Warp, Alacritty, Hyper.
- Запуск команд в различных режимах: текущее окно, новая вкладка, новое окно.
- Переключение запуска при входе в систему.
- Редактирование конфигурации прямо из меню трея.
- Открытие папки конфигурации из меню трея.
- Поддержка подменю для лучшей организации команд.
- Поддержка динамических входных данных для команд.
- **Новое**: Вызов контекстного меню с помощью горячей клавиши.

## Конфигурация

Конфигурация хранится в JSON-файлах, расположенных в каталоге конфигурации пользователя. Путь по умолчанию — `~/.config/switch-shuttle/` на Linux и macOS, и `C:\Users\<Username>\AppData\Roaming\switch-shuttle\` на Windows. Вы можете хранить несколько файлов конфигурации в этом каталоге, каждый из которых представляет собой другой набор команд и настроек.

Пример конфигурационного файла:

```json
{
   "terminal": "iterm",
   "launch_in": "current",
   "theme": "Homebrew",
   "title": "New tab",
   "menu_hotkey": "Ctrl+Shift+M",
   "commands": [
      {
         "name": "Command",
         "inputs": null,
         "command": null,
         "commands": null,
         "hotkey": null,
         "submenu": [
            {
               "name": "Example Command",
               "inputs": null,
               "command": "echo Hello, world!",
               "commands": null,
               "submenu": null,
               "hotkey": "Ctrl+Shift+E"
            },
            {
               "name": "Example Multi-Command with input",
               "inputs": {
                  "key1": "default1",
                  "key2": "default2"
               },
               "command": null,
               "commands": [
                  "export MY_VAR=$(echo 'Step 1: [key1]')",
                  "RESULT=$(echo 'Step 2: [key2]' && echo $MY_VAR)",
                  "echo Step 3: Finalize && echo $RESULT"
               ],
               "submenu": null,
               "hotkey": "Ctrl+Shift+M"
            },
            {
               "name": "Example Submenu",
               "inputs": null,
               "command": null,
               "commands": null,
               "submenu": [
                  {
                     "name": "Subcommand 1",
                     "inputs": null,
                     "command": "echo Subcommand 1",
                     "commands": null,
                     "submenu": null,
                     "hotkey": "Ctrl+Shift+S"
                  },
                  {
                     "name": "Subcommand 2",
                     "inputs": null,
                     "command": "echo Subcommand 2",
                     "commands": null,
                     "submenu": null,
                     "hotkey": null
                  }
               ],
               "hotkey": null
            }
         ]
      }
   ]
}
```

### Параметры конфигурации

| Параметр    | Тип               | Описание                                                | Допустимые значения                                                  |
|-------------|-------------------|---------------------------------------------------------|----------------------------------------------------------------------|
| terminal    | String            | Терминальное приложение для использования               | "iterm", "terminal", "warp"                                          |
| launch_in   | String            | Где запускать команду                                   | "current", "new_tab", "new_window"                                   |
| theme       | String            | Тема для использования (если поддерживается)            | Любое строковое значение, представляющее тему                        |
| title       | String            | Заголовок для окна/вкладки терминала                    | Любое строковое значение                                             |
| menu_hotkey | String (Optional) | Глобальная горячая клавиша для вызова контекстного меню | Любая допустимая комбинация горячих клавиш, например, "Ctrl+Shift+M" |
| commands    | Array             | Список конфигураций команд                              | См. ниже для параметров команд                                       |

### Параметры команд

| Параметр | Тип               | Описание                                       | Допустимые значения                                                  |
|----------|-------------------|------------------------------------------------|----------------------------------------------------------------------|
| name     | String            | Название команды или подменю                   | Любое строковое значение                                             |
| inputs   | Object (Optional) | Пары ключ-значение для входных данных          | {"key1": "default1", "key2": "default2"}                             |
| command  | String (Optional) | Команда для выполнения (если это команда)      | Любое строковое значение, представляющее команду                     |
| commands | Array (Optional)  | Список команд для последовательного выполнения | Любой массив строк, каждая строка - команда                          |
| submenu  | Array (Optional)  | Список подкоманд (если это подменю)            | См. выше для параметров команд                                       |
| hotkey   | String (Optional) | Глобальная горячая клавиша для вызова команды  | Любая допустимая комбинация горячих клавиш, например, "Ctrl+Shift+E" |

### Логика выполнения команд

SwitchShuttle поддерживает определение одной команды с использованием параметра `command`, списка команд с использованием параметра `commands` или обоих сразу. Если указаны и `command`, и `commands`, сначала выполняется одиночная команда, а затем команды из списка.

#### Пример выполнения

1. **Одиночная команда**: Если указан только `command`, выполняется эта команда.
2. **Несколько команд**: Если указан только `commands`, каждая команда из списка выполняется последовательно.
3. **Оба параметра**: Если указаны и `command`, и `commands`, сначала выполняется одиночная команда, а затем каждая команда из списка `commands`.

### Динамические входные данные

SwitchShuttle позволяет определить динамические входные данные для команд. Эти данные будут запрашиваться у пользователя перед выполнением команды. Вы можете определить входные данные с помощью параметра `inputs` в конфигурации команды.

#### Пример конфигурации с входными данными

```json
{
   "name": "Example Multi-Command with input",
   "inputs": {
      "key1": "default1",
      "key2": "default2"
   },
   "command": null,
   "commands": [
      "export MY_VAR=$(echo 'Step 1: [key1]')",
      "RESULT=$(echo 'Step 2: [key2]' && echo $MY_VAR)",
      "echo Step 3: Finalize && echo $RESULT"
   ],
   "submenu": null,
   "hotkey": "Ctrl+Shift+M"
}
```

### Горячие клавиши

Вы можете назначить глобальные горячие клавиши для команд, добавив параметр `hotkey` в конфигурацию команды. Комбинация горячих клавиш должна следовать формату модификаторов (Ctrl, Shift, Alt, Win) в сочетании с клавишей (A-Z, 0-9 и т.д.). Например, чтобы установить "Ctrl+Shift+E" в качестве горячей клавиши для команды:

```json
{
   "name": "Example Command",
   "command": "echo Hello, world!",
   "submenu": null,
   "hotkey": "Ctrl+Shift+E",
   "commands": null
}
```

Параметр `hotkey` является необязательным. Если он не указан, у команды не будет связанной с ней глобальной горячей клавиши.

### Как использовать горячие клавиши

1. **Назначение горячих клавиш**: Отредактируйте файл конфигурации, чтобы включить параметр `hotkey` для команд, которые вы хотите вызывать с помощью глобальных горячих клавиш.
2. **Использование горячих клавиш**: После перезапуска приложения используйте назначенные горячие клавиши для вызова соответствующих команд, независимо от того, какое приложение в данный момент активно.

## Как использовать

1. **Edit Config**: Щелкните правой кнопкой мыши по значку в трее и выберите конфигурация, чтобы открыть файл конфигурации в вашем редакторе по умолчанию. Измените конфигурацию по мере необходимости.
2. **Show Config Folder**: Щелкните правой кнопкой мыши по значку в трее и выберите "Show Config Folder", чтобы открыть каталог конфигурации в вашем файловом менеджере.
3. **Toggle Launch at Login**: Щелкните правой кнопкой мыши по значку в трее и выберите "Toggle Launch at Login", чтобы включить или отключить автоматический запуск приложения при входе в систему.
4. **Execute Command**: Щелкните левой кнопкой мыши по значку в трее и выберите команду, которую хотите выполнить из меню. Команда будет выполнена в указанном терминальном приложении.

### Создание подменю

Чтобы создать подменю, установите поле `command` и `commands` в `null` и предоставьте список подкоманд в поле `submenu`. Подкоманды также могут иметь свои собственные подменю, что позволяет создавать вложенные меню.

## Сборка приложения

### Необходимые условия

- [Rust](https://www.rust-lang.org/tools/install)
- [Tauri](https://tauri.app/v1/guides/getting-started/prerequisites/)

### Шаги

1. Клонируйте репозиторий:
```sh
git clone https://github.com/s00d/switchshuttle.git
cd switchshuttle
npm i
```

2. Соберите приложение:
```sh
cargo tauri build
```

3. Запустите приложение:
```sh
cargo tauri dev
```

## Загрузка

Последнюю версию SwitchShuttle можно скачать со страницы [GitHub Releases](https://github.com/s00d/switchshuttle/releases).

### macOS

Подпись приложения

Если вы используете macOS, возможно, вам потребуется подписать приложение перед его запуском. Вот шаги:

1. Сделайте бинарный файл исполняемым:

```bash
chmod +x /Applications/switch-shuttle.app
```

2. Очистите расширенные атрибуты и подпишите бинарный файл:

```bash
xattr -cr /Applications/switch-shuttle.app && codesign --force --deep --sign - /Applications/switch-shuttle.app
```

## Участие в разработке

Приветствуются ваши вклады! Пожалуйста, не стесняйтесь отправлять запросы на внесение изменений или открывать новые вопросы на GitHub.

## Лицензия

Этот проект лицензирован по лицензии MIT. См. файл [LICENSE](LICENSE) для подробностей.

---

Наслаждайтесь использованием SwitchShuttle для легкого управления вашими терминальными командами!
