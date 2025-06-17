<div align="center">
  [![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Windows%20%7C%20Linux-blue?style=for-the-badge)](https://github.com/s00d/switchshuttle)
  [![License](https://img.shields.io/badge/license-MIT-green?style=for-the-badge)](LICENSE)
  [![GitHub release](https://img.shields.io/github/v/release/s00d/switchshuttle?style=for-the-badge)](https://github.com/s00d/switchshuttle/releases)
  [![GitHub downloads](https://img.shields.io/github/downloads/s00d/switchshuttle/total?style=for-the-badge)](https://github.com/s00d/switchshuttle/releases)
  [![GitHub issues](https://img.shields.io/github/issues/s00d/switchshuttle?style=for-the-badge)](https://github.com/s00d/switchshuttle/issues)
  [![GitHub stars](https://img.shields.io/github/stars/s00d/switchshuttle?style=for-the-badge)](https://github.com/s00d/switchshuttle/stargazers)
  [![Donate](https://img.shields.io/badge/Donate-Donationalerts-ff4081?style=for-the-badge)](https://www.donationalerts.com/r/s00d88)

  <img src="https://raw.githubusercontent.com/s00d/switchshuttle/refs/heads/main/icons/logo-min.png" alt="SwitchShuttle Logo" width="600">
  
  # SwitchShuttle
  
  **🚀 Cross-platform terminal command manager with global hotkeys**

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

## ✨ What is SwitchShuttle?

SwitchShuttle is a powerful cross-platform system tray application that revolutionizes how you manage and execute terminal commands. Built with modern technologies (Rust + Tauri + Vue.js), it provides a sleek interface for organizing, customizing, and quickly accessing your most-used terminal operations.

### 🎯 Key Features

- **🖥️ Multi-Platform Support** - Works seamlessly on macOS, Windows, and Linux
- **⚡ Global Hotkeys** - Execute commands instantly from anywhere with keyboard shortcuts
- **🎨 Multiple Terminal Support** - iTerm, Terminal, Warp, Alacritty, Hyper, and more
- **📁 Smart Organization** - Create nested submenus for better command organization
- **🔧 Dynamic Inputs** - Interactive prompts for commands that need user input
- **🔄 Multiple Execution Modes** - Run in current window, new tab, or new window
- **🚀 Auto-Start** - Launch at system startup for instant access
- **🎨 Modern UI** - Beautiful, intuitive interface built with Vue.js

## 🚀 Quick Start

### Download & Install

1. **Download** the latest release for your platform from [GitHub Releases](https://github.com/s00d/switchshuttle/releases)
2. **Install** the application
3. **Launch** SwitchShuttle - it will appear in your system tray
4. **Right-click** the tray icon to access the menu

### First Configuration

1. **Edit Config** → Opens your configuration file in your default editor
2. **Add your commands** using the JSON format (see examples below)
3. **Save and restart** the application
4. **Enjoy** your organized command shortcuts!

## 📋 Configuration Guide

### Basic Structure

SwitchShuttle uses JSON configuration files stored in:
- **macOS/Linux**: `~/.config/switch-shuttle/`
- **Windows**: `C:\Users\<Username>\AppData\Roaming\switch-shuttle\`

### Simple Example

```json
{
  "terminal": "iterm",
  "launch_in": "new_tab",
  "title": "My Commands",
  "commands": [
    {
      "name": "🚀 Start Development Server",
      "command": "npm run dev",
      "hotkey": "Ctrl+Shift+D"
    },
    {
      "name": "📦 Install Dependencies",
      "command": "npm install",
      "hotkey": "Ctrl+Shift+I"
    },
    {
      "name": "🔧 Development Tools",
      "submenu": [
        {
          "name": "🧪 Run Tests",
          "command": "npm test",
          "hotkey": "Ctrl+Shift+T"
        },
        {
          "name": "📊 Build Project",
          "command": "npm run build",
          "hotkey": "Ctrl+Shift+B"
        }
      ]
    }
  ]
}
```

### Advanced Features

#### 🔧 Dynamic Inputs

Create interactive commands that prompt for user input:

```json
{
  "name": "📝 Create New Component",
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

#### 🔄 Multiple Commands

Execute a sequence of commands:

```json
{
  "name": "🔄 Full Development Cycle",
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

#### 📁 Nested Submenus

Organize commands in hierarchical menus:

```json
{
  "name": "🐳 Docker Operations",
  "submenu": [
    {
      "name": "🚀 Start Services",
      "submenu": [
        {
          "name": "🏗️ Development",
          "command": "docker-compose -f docker-compose.dev.yml up -d"
        },
        {
          "name": "🏭 Production",
          "command": "docker-compose -f docker-compose.prod.yml up -d"
        }
      ]
    },
    {
      "name": "🛑 Stop All",
      "command": "docker-compose down"
    }
  ]
}
```

## ⚙️ Configuration Reference

### Main Configuration

| Parameter | Type | Description | Default |
|-----------|------|-------------|---------|
| `terminal` | String | Terminal application to use | `"terminal"` |
| `launch_in` | String | Where to launch commands | `"current"` |
| `theme` | String | Terminal theme (if supported) | - |
| `title` | String | Window/tab title | - |
| `menu_hotkey` | String | Global hotkey to open menu | - |
| `commands` | Array | List of command configurations | `[]` |

### Terminal Options

| Terminal | macOS | Windows | Linux |
|----------|-------|---------|-------|
| `iterm` | ✅ | ❌ | ❌ |
| `terminal` | ✅ | ✅ | ✅ |
| `warp` | ✅ | ❌ | ❌ |
| `alacritty` | ✅ | ✅ | ✅ |
| `hyper` | ✅ | ✅ | ✅ |

### Launch Modes

| Mode | Description |
|------|-------------|
| `current` | Execute in current terminal window |
| `new_tab` | Open new tab and execute |
| `new_window` | Open new window and execute |

### Command Configuration

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `name` | String | ✅ | Display name for the command |
| `command` | String | ❌ | Single command to execute |
| `commands` | Array | ❌ | Multiple commands to execute |
| `submenu` | Array | ❌ | Nested subcommands |
| `inputs` | Object | ❌ | Dynamic input fields |
| `hotkey` | String | ❌ | Global hotkey shortcut |

## 🎯 Use Cases

### 👨‍💻 Developers
- **Quick project navigation** - Jump to different projects instantly
- **Build and test workflows** - One-click development cycles
- **Docker management** - Start/stop containers with hotkeys
- **Git operations** - Common git commands at your fingertips

### 🛠️ DevOps Engineers
- **Server management** - SSH connections and server commands
- **Monitoring tools** - Quick access to logs and metrics
- **Deployment scripts** - Automated deployment workflows
- **Database operations** - Common database commands

### 🎨 Designers
- **Asset optimization** - Image processing and optimization
- **Design system tools** - Component generation and updates
- **Prototype servers** - Quick design server startup

## 🔧 Building from Source

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [Node.js](https://nodejs.org/) (v16 or higher)
- [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites/)

### Build Steps

```bash
# Clone the repository
git clone https://github.com/s00d/switchshuttle.git
cd switchshuttle

# Install dependencies
npm install

# Development mode
npm run tauri dev

# Build for production
npm run tauri build
```

### Platform-Specific Notes

#### macOS
```bash
# If you encounter signing issues
chmod +x /Applications/switch-shuttle.app
xattr -cr /Applications/switch-shuttle.app
codesign --force --deep --sign - /Applications/switch-shuttle.app
```

## 🤝 Contributing

We welcome contributions! Here's how you can help:

1. **Fork** the repository
2. **Create** a feature branch (`git checkout -b feature/amazing-feature`)
3. **Commit** your changes (`git commit -m 'Add amazing feature'`)
4. **Push** to the branch (`git push origin feature/amazing-feature`)
5. **Open** a Pull Request

### Development Guidelines

- Follow the existing code style
- Add tests for new features
- Update documentation as needed
- Ensure cross-platform compatibility

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Inspired by the original [Shuttle](https://github.com/fitztrev/shuttle) project
- Built with [Tauri](https://tauri.app/) for cross-platform desktop apps
- UI powered by [Vue.js](https://vuejs.org/)

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/s00d/switchshuttle/issues)
- **Releases**: [GitHub Releases](https://github.com/s00d/switchshuttle/releases)

---

<div align="center">
  <p>Made with ❤️ by the SwitchShuttle community</p>
  <p>⭐ Star this repository if you find it useful!</p>
</div>
