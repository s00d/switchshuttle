<div align="center">
  
  [![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Windows%20%7C%20Linux-blue?style=for-the-badge)](https://github.com/s00d/switchshuttle)
  [![License](https://img.shields.io/badge/license-MIT-green?style=for-the-badge)](https://github.com/s00d/switchshuttle/blob/main/LICENSE)
  [![GitHub release](https://img.shields.io/github/v/release/s00d/switchshuttle?style=for-the-badge)](https://github.com/s00d/switchshuttle/releases)
  [![GitHub downloads](https://img.shields.io/github/downloads/s00d/switchshuttle/total?style=for-the-badge)](https://github.com/s00d/switchshuttle/releases)
  [![GitHub issues](https://img.shields.io/badge/github-issues-orange?style=for-the-badge)](https://github.com/s00d/switchshuttle/issues)
  [![GitHub stars](https://img.shields.io/badge/github-stars-yellow?style=for-the-badge)](https://github.com/s00d/switchshuttle/stargazers)
  [![Donate](https://img.shields.io/badge/Donate-Donationalerts-ff4081?style=for-the-badge)](https://www.donationalerts.com/r/s00d88)

  <img src="https://raw.githubusercontent.com/s00d/switchshuttle/refs/heads/main/icons/logo-min.png" alt="SwitchShuttle Logo" width="200">
  
  # SwitchShuttle
  
  **🚀 带全局热键的跨平台终端命令管理器**

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

## ✨ 什么是 SwitchShuttle？

SwitchShuttle 是一个强大的跨平台系统托盘应用程序，它彻底改变了您管理和执行终端命令的方式。使用现代技术（Rust + Tauri + Vue.js）构建，它提供了一个优雅的界面来组织、自定义和快速访问您最常用的终端操作。

### 🎯 主要功能

- **🖥️ 多平台支持** - 在 macOS、Windows 和 Linux 上无缝运行
- **⚡ 全局热键** - 使用键盘快捷键从任何地方立即执行命令
- **🎨 多终端支持** - iTerm、Terminal、Warp、Alacritty、Hyper 等
- **📁 智能组织** - 创建嵌套子菜单以更好地组织命令
- **🔧 动态输入** - 需要用户输入的命令的交互式提示
- **🔄 多种执行模式** - 在当前窗口、新标签页或新窗口中执行
- **🚀 自动启动** - 系统启动时启动以立即访问
- **🎨 现代界面** - 使用 Vue.js 构建的美丽直观界面
- **💻 命令行界面** - 直接从终端执行命令的 CLI
- **⚙️ 配置管理** - 启用/禁用配置而无需删除
- **🔄 开关命令** - 通过后台执行切换系统功能
- **📊 监控命令** - 实时系统资源监控，带视觉指示器
- **📅 计划命令** - 使用 cron 表达式自动化任务
- **🎯 模板系统** - 常见工作流程的预构建命令模板

## 🖥️ 用户界面概述

SwitchShuttle 提供了一个现代、直观的界面，具有几个关键组件：

### 🎛️ 主要界面组件

#### 1. **配置编辑器**
- **可视化 JSON 编辑器** - 使用语法高亮和验证编辑配置
- **模板系统** - 导入常见工作流程的预构建命令模板
- **实时验证** - 配置错误的即时反馈
- **自动保存** - 输入时自动保存更改
- **配置管理** - 启用/禁用配置而无需删除
- **搜索和过滤** - 快速找到特定配置
- **配置复制** - 为测试创建现有配置的副本

#### 2. **命令管理**
- **命令构建器** - 使用可视化表单界面创建命令
- **热键配置** - 设置即时命令执行的全局快捷键
- **图标选择器** - 选择表情符号图标以更好地视觉组织
- **输入字段** - 为交互式命令配置动态输入提示
- **嵌套子菜单** - 在层次结构中组织命令
- **命令验证** - 命令语法的实时验证

#### 3. **设置面板**
- **终端选择** - 选择您首选的终端应用程序
- **启动模式** - 配置命令执行方式（当前/新标签页/新窗口）
- **主题设置** - 自定义应用程序外观
- **自动启动配置** - 启用/禁用系统启动
- **全局热键设置** - 配置系统范围的菜单快捷键

#### 4. **系统托盘菜单**
- **快速访问** - 右键点击托盘图标以立即访问命令
- **状态指示器** - 开关命令和监控的视觉反馈
- **嵌套菜单** - 易于导航的有组织命令层次结构
- **全局热键** - 即时命令执行的键盘快捷键
- **实时监控** - 实时系统资源指示器

### 🎨 界面功能

#### **可视化命令构建器**
```json
{
  "name": "🚀 启动开发服务器",
  "command": "npm run dev",
  "hotkey": "Ctrl+Shift+D",
  "icon": "🚀",
  "background": false,
  "inputs": {
    "port": "3000",
    "host": "localhost"
  }
}
```

#### **模板系统**
SwitchShuttle 包含常见开发工作流程的预构建模板：

- **开发** - Git 操作、构建工具、测试
- **DevOps** - Docker、Kubernetes、服务器管理
- **数据库** - MySQL、PostgreSQL、MongoDB 操作
- **云服务** - AWS、Azure、Google Cloud 命令
- **安全** - 网络扫描、漏洞评估
- **监控** - 系统资源、日志、指标
- **实用工具** - 文件操作、系统工具
- **调度器** - Cron 作业和自动化任务

#### **智能组织**
- **嵌套子菜单** - 在逻辑组中组织命令
- **图标支持** - 使用表情符号图标进行视觉识别
- **热键管理** - 即时访问的全局快捷键
- **状态指示器** - 开关命令的实时反馈
- **搜索功能** - 快速命令发现

## 🔒 安全管理器

SwitchShuttle包含一个全面的安全管理器，可以保护您的系统免受潜在有害命令的影响，并提供对命令执行的精细控制。

### 🛡️ 安全功能

#### **命令验证**
- **长度限制**: 最大命令长度（1000字符）和输入长度（500字符）防止过长的命令
- **阻止的命令**: 定义永远不应该执行的危险命令列表
- **可疑模式**: 使用正则表达式模式来检测和阻止潜在有害的命令模式
- **实时验证**: 命令在编辑时进行验证以确保安全

#### **安全设置**
- **启用/禁用安全**: 根据需要切换安全功能
- **自定义阻止列表**: 向阻止的命令列表添加特定命令
- **模式匹配**: 定义正则表达式模式来捕获可疑的命令结构
- **长度限制**: 配置命令和用户输入的最大长度

#### **工作原理**
1. **编辑器验证**: SecurityManager在配置编辑器中保存前验证命令
2. **模式匹配**: 命令根据阻止的模式和可疑的正则表达式模式进行检查
3. **长度验证**: 命令和输入根据最大长度限制进行验证
4. **阻止列表检查**: 命令与用户定义的阻止命令列表进行比较
5. **安全配置**: 只允许保存和使用经过验证的配置



## 🚀 快速开始

### 下载和安装

#### 选项 1: Homebrew (macOS - 推荐)
```bash
# 通过 Homebrew 安装
brew tap s00d/switchshuttle
brew install --cask switchshuttle
```

#### 选项 2: 手动下载
1. **下载** 从 [GitHub Releases](https://github.com/s00d/switchshuttle/releases) 下载您平台的最新版本
2. **安装** 应用程序
3. **启动** SwitchShuttle - 它将出现在系统托盘中
4. **右键点击** 托盘图标访问菜单

### 首次配置

1. **打开配置编辑器** - 点击系统托盘菜单中的"编辑配置"
2. **选择终端** - 选择您首选的终端应用程序
3. **添加命令** - 使用可视化编辑器或导入模板
4. **设置热键** - 配置快速访问的全局快捷键
5. **保存并重启** - 您的命令现在在托盘菜单中可用

### 界面逐步指南

#### **步骤 1: 配置编辑器**
- 打开 SwitchShuttle 并导航到"编辑器"选项卡
- 选择您的终端应用程序（iTerm、Terminal、Warp 等）
- 设置启动模式（当前窗口、新标签页或新窗口）
- 配置菜单访问的全局热键

#### **步骤 2: 添加命令**
- 点击"添加命令"创建新命令
- 填写命令详情：
  - **名称**: 命令的显示名称
  - **命令**: 要执行的实际终端命令
  - **热键**: 全局快捷键（可选）
  - **图标**: 视觉识别的表情符号图标
  - **后台**: 是否在后台运行
  - **输入**: 交互式命令的动态输入字段

#### **步骤 3: 使用模板**
- 点击"导入模板"访问预构建的命令集合
- 浏览开发、DevOps、数据库等类别
- 选择并导入您需要的模板
- 根据需要自定义导入的命令

#### **步骤 4: 系统托盘访问**
- 右键点击 SwitchShuttle 托盘图标
- 浏览您有组织的命令菜单
- 使用全局热键进行即时命令执行
- 使用实时指示器监控系统状态

#### **步骤 5: 高级功能**
- **开关命令**: 带视觉状态指示器的系统功能切换
- **监控命令**: 系统资源的实时监控
- **计划命令**: 使用 cron 表达式自动化任务
- **嵌套菜单**: 层次结构中的命令组织

## 💻 命令行界面 (CLI)

SwitchShuttle 还提供了一个强大的命令行界面，用于无需打开 GUI 的快速命令执行。

### CLI 使用

#### 执行命令
```bash
# 通过命令 ID 执行
switch-shuttle cmd_8

# 通过命令名称执行（不区分大小写）
switch-shuttle "示例命令"
```

#### 列出所有命令
```bash
# 显示所有可用命令及其 ID
switch-shuttle --list
# 或
switch-shuttle -l
```

#### 搜索命令
```bash
# 搜索包含特定文本的命令
switch-shuttle --search "git"
# 或
switch-shuttle -s "docker"
```

### 在不同操作系统上运行 CLI

#### macOS
```bash
# 如果通过 Homebrew 安装
switch-shuttle --list

# 如果手动安装
/Applications/switch-shuttle.app/Contents/MacOS/SwitchShuttle --list

# 创建别名以便更容易访问
echo 'alias switch-shuttle="/Applications/switch-shuttle.app/Contents/MacOS/SwitchShuttle"' >> ~/.zshrc
source ~/.zshrc
```

#### Windows
```bash
# 如果通过安装程序安装
"C:\Program Files\SwitchShuttle\switch-shuttle.exe" --list

# 如果通过 winget 或 chocolatey 安装
switch-shuttle --list

# 添加到 PATH 以便更容易访问
# 将 "C:\Program Files\SwitchShuttle" 添加到系统 PATH
```

#### Linux
```bash
# 如果通过包管理器安装
switch-shuttle --list

# 如果手动安装
./switch-shuttle --list

# 使其可执行并添加到 PATH
chmod +x switch-shuttle
sudo mv switch-shuttle /usr/local/bin/
```

### CLI 示例

```bash
# 快速 Git 操作
switch-shuttle "git status"
switch-shuttle "git pull"

# 开发工作流程
switch-shuttle "npm run dev"
switch-shuttle "docker-compose up"

# 列出所有可用命令
switch-shuttle --list

# 查找与数据库相关的命令
switch-shuttle --search "database"
```

### CLI 功能

- **🚀 快速执行** - 从终端立即执行命令
- **🔍 智能搜索** - 通过 ID 或名称查找命令
- **📋 命令列表** - 查看所有可用命令
- **⚡ 无需 GUI** - 完美用于自动化和脚本
- **🔄 执行后退出** - 干净的终端体验

## 📋 配置指南

### 基本结构

SwitchShuttle 使用 JSON 配置文件，存储在：
- **macOS/Linux**: `~/.config/switch-shuttle/`
- **Windows**: `C:\Users\<用户名>\AppData\Roaming\switch-shuttle\`

### 简单示例

```json
{
  "terminal": "iterm",
  "launch_in": "new_tab",
  "title": "我的命令",
  "commands": [
    {
      "name": "🚀 启动开发服务器",
      "command": "npm run dev",
      "hotkey": "Ctrl+Shift+D"
    },
    {
      "name": "📦 安装依赖",
      "command": "npm install",
      "hotkey": "Ctrl+Shift+I"
    },
    {
      "name": "🔧 开发工具",
      "submenu": [
        {
          "name": "🧪 运行测试",
          "command": "npm test",
          "hotkey": "Ctrl+Shift+T"
        },
        {
          "name": "📊 构建项目",
          "command": "npm run build",
          "hotkey": "Ctrl+Shift+B"
        }
      ]
    }
  ]
}
```

### 高级功能

#### 🔧 动态输入

创建需要用户输入的交互式命令：

```json
{
  "name": "📝 创建新组件",
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

#### 🔄 多命令

执行命令序列：

```json
{
  "name": "🔄 完整开发周期",
  "commands": [
    "git pull origin main",
    "npm install",
    "npm run lint",
    "npm run test",
    "npm run build"
  ],
  "background": true,
  "hotkey": "Ctrl+Shift+F"
}
```

#### ⏰ 计划命令 (Cron)

使用 cron 表达式安排命令自动执行：

```json
{
  "name": "🔄 自动备份",
  "commands": [
    "rsync -av /source/ /backup/"
  ],
  "scheduler": "0 2 * * *",
  "background": true,
  "hotkey": "Ctrl+Shift+B"
}
```

**Cron 表达式格式:**
调度器使用标准的 6 字段 cron 表达式：`秒 分 时 日 月 星期`

**常见 Cron 示例:**
- `"0 0 * * * *"` - 每小时整点
- `"0 0 2 * * *"` - 每天凌晨 2 点
- `"0 30 9 * * 1-5"` - 工作日上午 9:30
- `"0 0 12 * * 1"` - 每周一中午
- `"0 0 0 1 * *"` - 每月 1 号
- `"0 */15 * * * *"` - 每 15 分钟
- `"0 0 0 * * 0"` - 每周日午夜

**调度器功能:**
- **后台执行** - 命令静默运行，不打开终端
- **Cron 支持** - 完整的 cron 表达式解析和执行
- **错误处理** - cron 解析失败时的优雅回退
- **跨平台** - 在 macOS、Windows 和 Linux 上工作

#### 🖥️ 后台执行

控制命令执行方式 - 使用 ConsolePool 在后台或正常终端执行：

```json
{
  "name": "🚀 启动服务器",
  "commands": [
    "npm run dev"
  ],
  "background": true,
  "hotkey": "Ctrl+Shift+S"
}
```

**后台执行选项:**
- `"background": true` - 使用 ConsolePool 执行（后台）
- `"background": false` - 使用正常终端执行
- `"background": null` 或省略 - 基于命令类型自动检测

#### 📁 嵌套子菜单

在层次菜单中组织命令：

```json
{
  "name": "🐳 Docker 操作",
  "submenu": [
    {
      "name": "🚀 启动服务",
      "submenu": [
        {
          "name": "🏗️ 开发",
          "command": "docker-compose -f docker-compose.dev.yml up -d"
        },
        {
          "name": "🏭 生产",
          "command": "docker-compose -f docker-compose.prod.yml up -d"
        }
      ]
    },
    {
      "name": "🛑 停止所有",
      "command": "docker-compose down"
    }
  ]
}
```

#### 🔄 开关命令

通过后台执行切换系统功能：

```json
{
  "name": "🔧 系统控制",
  "submenu": [
    {
      "name": "📶 切换 WiFi",
      "command": "networksetup -setairportpower en0 toggle",
      "switch": "networksetup -getairportpower en0 | grep -q 'On' && echo 'true' || echo 'false'"
    },
    {
      "name": "🔊 切换蓝牙",
      "command": "blueutil -p toggle",
      "switch": "blueutil -p | grep -q '1' && echo 'true' || echo 'false'"
    },
    {
      "name": "🌙 切换深色模式",
      "command": "osascript -e 'tell app \"System Events\" to tell appearance preferences to set dark mode to not dark mode'",
      "switch": "osascript -e 'tell app \"System Events\" to tell appearance preferences to get dark mode'"
    }
  ]
}
```

**开关命令功能:**
- **后台执行** - 命令静默运行，不打开终端
- **状态检查** - 自动检测当前状态
- **视觉反馈** - 在菜单中显示启用/禁用状态
- **跨平台** - 在 macOS、Windows 和 Linux 上工作

#### 📊 监控命令

使用实时信息监控系统资源和服务：

```json
{
  "name": "📊 系统监控",
  "submenu": [
    {
      "name": "💾 内存使用",
      "command": "top -l 1 | head -n 10",
      "monitor": "memory",
      "icon": "🧠"
    },
    {
      "name": "💻 CPU 负载",
      "command": "top -l 1 | grep 'CPU usage'",
      "monitor": "cpu",
      "icon": "⚡"
    },
    {
      "name": "💾 磁盘空间",
      "command": "df -h | grep '/dev/'",
      "monitor": "disk",
      "icon": "💾"
    },
    {
      "name": "🌐 网络状态",
      "command": "ifconfig | grep -E 'inet |status:'",
      "monitor": "network",
      "icon": "🌐"
    }
  ]
}
```

**监控命令功能:**
- **菜单集成** - 向系统托盘菜单添加监控按钮
- **命令执行** - 菜单打开时执行监控命令
- **数据显示** - 直接在菜单界面中显示命令输出
- **视觉指示器** - 菜单中的图标和状态指示器
- **跨平台** - 在 macOS、Windows 和 Linux 上工作

## ⚙️ 配置参考

### 主配置

| 参数 | 类型 | 描述 | 默认值 |
|------|------|------|--------|
| `terminal` | String | 要使用的终端应用程序 | `"terminal"` |
| `launch_in` | String | 启动命令的位置 | `"current"` |
| `theme` | String | 终端主题（如果支持） | - |
| `title` | String | 窗口/标签页标题 | - |
| `menu_hotkey` | String | 打开菜单的全局热键 | - |
| `commands` | Array | 命令配置列表 | `[]` |
| `enabled` | Boolean | 是否应加载此配置 | `true` |

### 终端选项

| 终端 | macOS | Windows | Linux |
|------|-------|---------|-------|
| `iterm` | ✅ | ❌ | ❌ |
| `terminal` | ✅ | ✅ | ✅ |
| `warp` | ✅ | ❌ | ❌ |
| `alacritty` | ✅ | ✅ | ✅ |
| `hyper` | ✅ | ✅ | ✅ |

### 启动模式

| 模式 | 描述 |
|------|------|
| `current` | 在当前终端窗口中执行 |
| `new_tab` | 打开新标签页并执行 |
| `new_window` | 打开新窗口并执行 |

### 命令配置

| 参数 | 类型 | 必需 | 描述 |
|------|------|------|------|
| `name` | String | ✅ | 命令的显示名称 |
| `commands` | Array | ❌ | 要执行的多个命令 |
| `submenu` | Array | ❌ | 嵌套子命令 |
| `switch` | String | ❌ | 检查开关状态的命令（返回 true/false） |
| `monitor` | String | ❌ | 获取监控显示值的命令 |
| `inputs` | Object | ❌ | 动态输入字段 |
| `hotkey` | String | ❌ | 全局热键快捷键 |
| `icon` | String | ❌ | 视觉识别的表情符号图标 |
| `background` | Boolean | ❌ | 在后台执行（ConsolePool）或正常终端 |
| `scheduler` | String | ❌ | 计划执行的 cron 表达式 |

### 配置管理

#### 启用/禁用配置

您可以启用或禁用单个配置文件来控制系统托盘菜单中可用的命令。这对于以下情况很有用：

- **临时禁用** - 禁用配置而不删除
- **测试** - 开发期间启用/禁用配置
- **组织** - 保持多个配置但仅使用特定配置

**在可视化编辑器中:**
- 打开配置编辑器
- 使用"配置状态"部分中的切换开关
- 启用的配置将被加载并在菜单中可用
- 禁用的配置将被忽略

**在 JSON 配置中:**
```json
{
  "terminal": "iterm",
  "launch_in": "current",
  "title": "我的命令",
  "enabled": true,
  "commands": [
    {
      "name": "示例命令",
      "command": "echo Hello World"
    }
  ]
}
```

| 参数 | 类型 | 默认值 | 描述 |
|------|------|--------|------|
| `enabled` | Boolean | `true` | 是否应加载此配置并在菜单中可用 |

**注意:** 当 `enabled` 设置为 `false` 或省略时，配置将被忽略，其命令不会出现在系统托盘菜单中。

## 🎯 使用案例

### 👨‍💻 开发者
- **快速项目导航** - 立即跳转到不同项目
- **构建和测试工作流程** - 一键开发周期
- **Docker 管理** - 使用热键启动/停止容器
- **Git 操作** - 手边的常用 git 命令
- **开发服务器管理** - 启动/停止开发服务器
- **代码质量工具** - 运行 linter、formatter 和测试

### 🛠️ DevOps 工程师
- **服务器管理** - SSH 连接和服务器命令
- **监控工具** - 快速访问日志和指标
- **部署脚本** - 自动化部署工作流程
- **数据库操作** - 常用数据库命令
- **容器编排** - Docker 和 Kubernetes 管理
- **基础设施监控** - 系统资源跟踪

### 🎨 设计师
- **资源优化** - 图像处理和优化
- **设计系统工具** - 组件生成和更新
- **原型服务器** - 快速设计服务器启动
- **设计工具自动化** - 批处理和工作流程

### 🔧 系统管理员
- **系统监控** - 实时资源监控
- **服务管理** - 启动/停止系统服务
- **备份自动化** - 计划备份操作
- **网络工具** - 网络诊断和配置
- **安全工具** - 漏洞扫描和评估
- **维护任务** - 系统清理和优化

## 🔧 从源代码构建

### 先决条件

- [Rust](https://www.rust-lang.org/tools/install) (最新稳定版)
- [Node.js](https://nodejs.org/) (v16 或更高)
- [pnpm](https://pnpm.io/) (推荐的包管理器)
- [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites/)

### 构建步骤

```bash
# 克隆仓库
git clone https://github.com/s00d/switchshuttle.git
cd switchshuttle

# 安装依赖
pnpm install

# 开发模式
pnpm run tauri dev

# 生产构建
pnpm run tauri build
```

### 平台特定说明

#### macOS
```bash
# 如果遇到签名问题
chmod +x /Applications/switch-shuttle.app
xattr -cr /Applications/switch-shuttle.app
codesign --force --deep --sign - /Applications/switch-shuttle.app
```

#### Windows
```bash
# 安装 Rust 和 Node.js
# 然后按照上面的构建步骤
```

#### Linux
```bash
# 安装系统依赖
sudo apt-get update
sudo apt-get install libwebkit2gtk-4.0-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev

# 然后按照上面的构建步骤
```

## 🤝 贡献

我们欢迎贡献！以下是您可以提供帮助的方式：

1. **Fork** 仓库
2. **创建** 功能分支 (`git checkout -b feature/amazing-feature`)
3. **提交** 您的更改 (`git commit -m 'Add amazing feature'`)
4. **推送** 到分支 (`git push origin feature/amazing-feature`)
5. **打开** Pull Request

### 开发指南

- 遵循现有代码风格
- 为新功能添加测试
- 根据需要更新文档
- 确保跨平台兼容性
- 使用 pnpm 进行包管理

### 开发设置

```bash
# 安装依赖
pnpm install

# 启动开发服务器
pnpm run tauri dev

# 运行类型检查
pnpm run type-check

# 生产构建
pnpm run tauri build
```

## 📄 许可证

本项目采用 MIT 许可证 - 详情请参阅 [LICENSE](https://github.com/s00d/switchshuttle/blob/main/LICENSE) 文件。

## 🙏 致谢

- 受原始 [Shuttle](https://github.com/fitztrev/shuttle) 项目启发
- 使用 [Tauri](https://tauri.app/) 构建跨平台桌面应用
- UI powered by [Vue.js](https://vuejs.org/)
- 使用 [Tailwind CSS](https://tailwindcss.com/) 样式化

## 📞 支持

- **Issues**: [GitHub Issues](https://github.com/s00d/switchshuttle/issues)
- **Releases**: [GitHub Releases](https://github.com/s00d/switchshuttle/releases)
- **Documentation**: [GitHub Wiki](https://github.com/s00d/switchshuttle/wiki)

---

<div align="center">
  <p>由 SwitchShuttle 社区用 ❤️ 制作</p>
  <p>⭐ 如果您觉得这个仓库有用，请给它一个星标！</p>
</div>
