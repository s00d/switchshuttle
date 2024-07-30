![SwitchShuttle](https://raw.githubusercontent.com/s00d/switchshuttle/main/icons/logo.webp)
![intro](https://github.com/s00d/switchshuttle/blob/main/icons/intro.gif?raw=true)

[[English](README.md)] [[中文](README_ZH.md)] [[Русский](README_RU.md)] [[Deutsch](README_DE.md)] [[日本語](README_JA.md)]

## SwitchShuttle

SwitchShuttle 是一个跨平台系统托盘应用程序，允许用户在各种终端应用程序中运行预定义的命令。它支持 macOS、Windows 和 Linux，提供了一种简单且可定制的方式来管理和执行您经常使用的命令。

## 关于

SwitchShuttle 是对 [Shuttle](https://github.com/fitztrev/shuttle) 应用程序的重新构想和扩展。虽然 Shuttle 提供了一种简单有效的方式来管理 macOS 中的命令快捷方式，但 SwitchShuttle 在这一概念基础上进行了扩展，提供对多种操作系统和终端模拟器的支持，并增强了配置和用户自定义功能。

## 功能

- 支持多种终端应用程序：iTerm、Terminal、Warp。
- 在不同模式下运行命令：当前窗口、新标签页、新窗口。
- 切换登录时启动。
- 直接从托盘菜单编辑配置。
- 从托盘菜单打开配置文件夹。
- 支持子菜单以更好地组织命令。
- 支持命令的动态输入。
- **新功能**：通过热键触发上下文菜单。

## 配置

配置存储在用户配置目录中的 JSON 文件中。默认路径为 Linux 和 macOS 上的 `~/.config/switch-shuttle/`，Windows 上的 `C:\Users\<Username>\AppData\Roaming\switch-shuttle\`。您可以在该目录中存储多个配置文件，每个配置文件代表一组不同的命令和设置。

以下是一个配置文件示例：

```json
{
  "terminal": "iterm",
  "launch_in": "current",
  "theme": "Homebrew",
  "title": "新标签页",
  "menu_hotkey": "Ctrl+Shift+M",
  "commands": [
    {
      "name": "命令",
      "inputs": null,
      "command": null,
      "commands": null,
      "hotkey": null,
      "submenu": [
        {
          "name": "示例命令",
          "inputs": null,
          "command": "echo 你好，世界！",
          "commands": null,
          "submenu": null,
          "hotkey": "Ctrl+Shift+E"
        },
        {
          "name": "带输入的示例多命令",
          "inputs": {
            "key1": "默认值1",
            "key2": "默认值2"
          },
          "command": null,
          "commands": [
            "export MY_VAR=$(echo '步骤1: [key1]')",
            "RESULT=$(echo '步骤2: [key2]' && echo $MY_VAR)",
            "echo 步骤3: 完成 && echo $RESULT"
          ],
          "submenu": null,
          "hotkey": "Ctrl+Shift+M"
        },
        {
          "name": "示例子菜单",
          "inputs": null,
          "command": null,
          "commands": null,
          "submenu": [
            {
              "name": "子命令1",
              "inputs": null,
              "command": "echo 子命令1",
              "commands": null,
              "submenu": null,
              "hotkey": "Ctrl+Shift+S"
            },
            {
              "name": "子命令2",
              "inputs": null,
              "command": "echo 子命令2",
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

### 配置参数

| 参数          | 类型                | 描述           | 有效值                                |
|-------------|-------------------|--------------|------------------------------------|
| terminal    | String            | 使用的终端应用程序    | "iterm", "terminal", "warp"        |
| launch_in   | String            | 命令启动位置       | "current", "new_tab", "new_window" |
| theme       | String            | 终端支持的主题      | 表示主题的任何字符串值                        |
| title       | String            | 终端窗口/标签页的标题  | 任何字符串值                             |
| menu_hotkey | String (Optional) | 触发上下文菜单的全局热键 | 任何有效的热键组合，例如 "Ctrl+Shift+M"        |
| commands    | Array             | 命令配置列表       | 参见下方的命令参数                          |

### 命令参数

| 参数       | 类型                | 描述               | 有效值                                      |
|----------|-------------------|------------------|------------------------------------------|
| name     | String            | 命令或子菜单的名称        | 任何字符串值                                   |
| inputs   | Object (Optional) | 输入的键值对           | {"key1": "default1", "key2": "default2"} |
| command  | String (Optional) | 要执行的命令（如果这是一个命令） | 表示命令的任何字符串值                              |
| commands | Array (Optional)  | 顺序执行的命令列表        | 任何字符串数组，每个字符串都是一个命令                      |
| submenu  | Array (Optional)  | 子命令列表（如果这是一个子菜单） | 参见上方的命令参数                                |
| hotkey   | String (Optional) | 触发命令的全局热键        | 任何有效的热键组合，例如 "Ctrl+Shift+E"              |

### 命令执行逻辑

SwitchShuttle 支持使用 `command` 参数定义单个命令，使用 `commands` 参数定义命令列表，或同时使用这两个参数。如果同时指定了 `command` 和 `commands`，则先执行单个命令，然后执行列表中的命令。

#### 执行示例流程

1. **单个命令**：如果仅指定了 `command`，则执行该命令。
2. **多个命令**：如果仅指定了 `commands`，则按顺序执行列表中的每个命令。
3. **同时指定**：如果同时指定了 `command` 和 `commands`，则先执行单个命令，然后按顺序执行 `commands` 列表中的每个命令。

### 动态输入

SwitchShuttle 允许您为命令定义动态输入。在执行命令之前，这些输入将从用户处获取。您可以使用命令配置中的 `inputs` 参数来定义输入。

#### 带有输入的示例配置

```json
{
  "name": "带输入的示例多命令",
  "inputs": {
    "key1": "默认值1",
    "key2": "默认值2"
  },
  "command": null,
  "commands": [
    "export MY_VAR=$(echo '步骤1: [key1]')",
    "RESULT=$(echo '步骤2: [key2]' && echo $MY_VAR)",
    "echo 步骤3: 完成 && echo $RESULT"
  ],
  "submenu": null,
  "hotkey": "Ctrl+Shift+M"
}
```

### 热键

您可以通过在命令配置中添加 `hotkey` 参数来为命令分配全局热键。热键组合必须遵循修饰键（Ctrl、Shift、Alt、Win）与键（A-Z、0-9 等）相结合的格式。例如，要为命令设置 "Ctrl+Shift+E" 作为热键：

```json
{
  "name": "示例命令",
  "command": "echo 你好，世界！",
  "submenu": null,
  "hotkey": "Ctrl+Shift+E",
  "commands": null
}
```

参数 `hotkey` 是可选的。如果没有指定，该命令将没有与之关联的全局热键。

### 如何使用热键

1. **分配热键**：编辑配置文件，以包含 `hotkey` 参数，为您希望通过全局热键触发的命令。
2. **使用热键**：重新启动应用程序后，使用分配的热键触发相应的命令，无论当前焦点在哪个应用程序上。

## 如何使用

1. **Edit Config**：右键单击托盘图标并选择 "Edit Config" 以在默认编辑器中打开配置文件。根据需要修改配置。
2. **Show Config Folder**：右键单击托盘图标并选择 "Show Config Folder" 以在文件管理器中打开配置目录。
3. **Toggle Launch at Login**：右键单击托盘图标并选择 "Toggle Launch at Login" 以启用或禁用应用程序在登录时启动。
4. **Execute Command**：左键单击托盘图标并从菜单

中选择您要执行的命令。该命令将在指定的终端应用程序中执行。

### 创建子菜单

要创建子菜单，请将 `command` 字段设置为 `null` 并在 `submenu` 字段中提供子命令列表。子命令也可以有自己的子菜单，从而允许嵌套菜单。

## 构建应用程序

### 先决条件

- [Rust](https://www.rust-lang.org/tools/install)
- [Tauri](https://tauri.app/v1/guides/getting-started/prerequisites/)

### 步骤

1. 克隆存储库：
   ```sh
   git clone https://github.com/s00d/switchshuttle.git
   cd switchshuttle
   ```

2. 构建应用程序：
   ```sh
   cargo build --release
   ```

3. 运行应用程序：
   ```sh
   cargo run
   ```

## 下载

可以从 [GitHub Releases](https://github.com/s00d/switchshuttle/releases) 页面下载最新版本的 SwitchShuttle。

### macOS

应用程序签名

如果您使用的是 macOS，可能需要在运行应用程序之前对其进行签名。步骤如下：

1. 使二进制文件可执行：

```bash
chmod +x /Applications/switch-shuttle.app
```

2. 清除扩展属性并签署二进制文件：

```bash
xattr -cr /Applications/switch-shuttle.app && codesign --force --deep --sign - /Applications/switch-shuttle.app
```

## 贡献

欢迎贡献！请随时提交拉取请求或在 GitHub 上打开问题。

## 许可证

本项目依据 MIT 许可证授权。详情请参见 [LICENSE](LICENSE) 文件。

---

享受使用 SwitchShuttle 轻松管理您的终端命令吧！