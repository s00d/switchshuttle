<div align="center">
  
  [![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Windows%20%7C%20Linux-blue?style=for-the-badge)](https://github.com/s00d/switchshuttle)
  [![License](https://img.shields.io/badge/license-MIT-green?style=for-the-badge)](https://github.com/s00d/switchshuttle/blob/main/LICENSE)
  [![GitHub release](https://img.shields.io/github/v/release/s00d/switchshuttle?style=for-the-badge)](https://github.com/s00d/switchshuttle/releases)
  [![GitHub downloads](https://img.shields.io/github/downloads/s00d/switchshuttle/total?style=for-the-badge)](https://github.com/s00d/switchshuttle/releases)
  [![GitHub issues](https://img.shields.io/github/issues/s00d/switchshuttle?style=for-the-badge)](https://github.com/s00d/switchshuttle/issues)
  [![GitHub stars](https://img.shields.io/github/stars/s00d/switchshuttle?style=for-the-badge)](https://github.com/s00d/switchshuttle/stargazers)
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

1. **编辑配置** → 在默认编辑器中打开配置文件
2. **添加命令** 使用 JSON 格式（见下面的示例）
3. **保存并重启** 应用程序
4. **享受** 您组织的命令快捷方式！

## 📋 配置指南

### 基本结构

SwitchShuttle 使用 JSON 配置文件，存储在：
- **macOS/Linux**: `~/.config/switch-shuttle/`
- **Windows**: `C:\Users\<Username>\AppData\Roaming\switch-shuttle\`

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

创建提示用户输入的交互式命令：

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
  "hotkey": "Ctrl+Shift+F"
}
```

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

**开关命令功能：**
- **后台执行** - 命令静默运行，不打开终端
- **状态检查** - 自动检测当前状态
- **视觉反馈** - 在菜单中显示启用/禁用状态
- **跨平台** - 在 macOS、Windows 和 Linux 上工作

#### 📊 监控命令

监控系统资源和服务，提供实时信息：

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

**监控命令功能：**
- **菜单集成** - 在系统托盘菜单中添加监控按钮
- **命令执行** - 打开菜单时执行监控命令
- **数据显示** - 直接在菜单界面中显示命令输出
- **视觉指示器** - 菜单中的图标和状态指示器
- **跨平台** - 在 macOS、Windows 和 Linux 上运行

#### ⏰ 计划命令 (Cron)

使用 cron 表达式自动计划命令执行：

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

**调度器功能：**
- **Cron 表达式** - 使用标准 cron 格式进行调度
- **后台执行** - 静默运行计划命令
- **跨平台** - 在 macOS、Windows 和 Linux 上运行
- **持久性** - 即使菜单关闭，计划也会继续运行

#### 🖥️ 后台执行

控制命令的执行方式 - 使用 ConsolePool 在后台或普通终端执行：

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

**后台执行选项：**
- `"background": true` - 使用 ConsolePool 执行（后台）
- `"background": false` - 使用普通终端执行
- `"background": null` 或省略 - 根据命令类型自动检测

## ⚙️ 配置参考

### 主配置

| 参数 | 类型 | 描述 | 默认值 |
|------|------|------|--------|
| `terminal` | String | 使用的终端应用程序 | `"terminal"` |
| `launch_in` | String | 命令启动位置 | `"current"` |
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
| `hotkey` | String | ❌ | 全局热键 |
| `icon` | String | ❌ | 用于视觉识别的表情符号图标 |
| `background` | Boolean | ❌ | 在后台执行（ConsolePool）或普通终端 |
| `scheduler` | String | ❌ | 计划执行的 cron 表达式 |

### 配置管理

#### 启用/禁用配置

您可以启用或禁用单个配置文件来控制系统托盘菜单中可用的命令。这对于以下情况很有用：

- **临时禁用** - 禁用配置而不删除它们
- **测试** - 在开发过程中启用/禁用配置
- **组织** - 保持多个配置但仅使用特定配置

**在可视化编辑器中：**
- 打开配置编辑器
- 在"配置状态"部分使用切换开关
- 启用的配置将被加载并在菜单中可用
- 禁用的配置将被忽略

**在 JSON 配置中：**
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
| `enabled` | Boolean | `true` | 此配置是否应加载并在菜单中可用 |

**注意：** 当 `enabled` 设置为 `false` 或省略时，配置将被忽略，其命令不会出现在系统托盘菜单中。

## 🎯 使用场景

### 👨‍💻 开发者
- **快速项目导航** - 立即跳转到不同项目
- **构建和测试工作流** - 一键开发周期
- **Docker 管理** - 使用热键启动/停止容器
- **Git 操作** - 常用 Git 命令触手可及

### 🛠️ DevOps 工程师
- **服务器管理** - SSH 连接和服务器命令
- **监控工具** - 快速访问日志和指标
- **部署脚本** - 自动化部署工作流
- **数据库操作** - 常用数据库命令

### 🎨 设计师
- **资源优化** - 图像处理和优化
- **设计系统工具** - 组件生成和更新
- **原型服务器** - 快速设计服务器启动

## 🔧 从源码构建

### 先决条件

- [Rust](https://www.rust-lang.org/tools/install) (最新稳定版)
- [Node.js](https://nodejs.org/) (v16 或更高)
- [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites/)

### 构建步骤

```bash
# 克隆仓库
git clone https://github.com/s00d/switchshuttle.git
cd switchshuttle

# 安装依赖
npm install

# 开发模式
npm run tauri dev

# 生产构建
npm run tauri build
```

### 平台特定说明

#### macOS
```bash
# 如果遇到签名问题
chmod +x /Applications/switch-shuttle.app
xattr -cr /Applications/switch-shuttle.app
codesign --force --deep --sign - /Applications/switch-shuttle.app
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

## 📄 许可证

本项目在 MIT 许可证下授权 - 详情请参阅 [LICENSE](https://github.com/s00d/switchshuttle/blob/main/LICENSE) 文件。

## 🙏 致谢

- 受原始 [Shuttle](https://github.com/fitztrev/shuttle) 项目启发
- 使用 [Tauri](https://tauri.app/) 构建跨平台桌面应用
- UI 由 [Vue.js](https://vuejs.org/) 驱动

## 📞 支持

- **Issues**: [GitHub Issues](https://github.com/s00d/switchshuttle/issues)
- **Releases**: [GitHub Releases](https://github.com/s00d/switchshuttle/releases)

---

<div align="center">
  <p>由 SwitchShuttle 社区用 ❤️ 制作</p>
  <p>⭐ 如果这个仓库对您有用，请给它一个星标！</p>
</div>
