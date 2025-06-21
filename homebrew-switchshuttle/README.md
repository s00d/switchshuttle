# Homebrew SwitchShuttle Tap

This repository contains the Homebrew Cask for [SwitchShuttle](https://github.com/s00d/switchshuttle) - a cross-platform terminal command manager with global hotkeys.

## Installation

```bash
# Add this tap
brew tap s00d/switchshuttle

# Install SwitchShuttle
brew install --cask switchshuttle
```

## What is SwitchShuttle?

SwitchShuttle is a powerful cross-platform system tray application that revolutionizes how you manage and execute terminal commands. Built with modern technologies (Rust + Tauri + Vue.js), it provides a sleek interface for organizing, customizing, and quickly accessing your most-used terminal operations.

### Key Features

- **🖥️ Multi-Platform Support** - Works seamlessly on macOS, Windows, and Linux
- **⚡ Global Hotkeys** - Execute commands instantly from anywhere with keyboard shortcuts
- **🎨 Multiple Terminal Support** - iTerm, Terminal, Warp, Alacritty, Hyper, and more
- **📁 Smart Organization** - Create nested submenus for better command organization
- **🔧 Dynamic Inputs** - Interactive prompts for commands that need user input
- **🔄 Multiple Execution Modes** - Run in current window, new tab, or new window
- **🚀 Auto-Start** - Launch at system startup for instant access
- **🎨 Modern UI** - Beautiful, intuitive interface built with Vue.js

## Quick Start

After installation:

1. **Launch** SwitchShuttle - it will appear in your system tray
2. **Right-click** the tray icon to access the menu
3. **Edit Config** → Opens your configuration file in your default editor
4. **Add your commands** using the JSON format
5. **Save and restart** the application
6. **Enjoy** your organized command shortcuts!

## Configuration

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
    }
  ]
}
```

## Supported Terminals

| Terminal | macOS | Windows | Linux |
|----------|-------|---------|-------|
| `iterm` | ✅ | ❌ | ❌ |
| `terminal` | ✅ | ✅ | ✅ |
| `warp` | ✅ | ❌ | ❌ |
| `alacritty` | ✅ | ✅ | ✅ |
| `hyper` | ✅ | ✅ | ✅ |

## Links

- **GitHub**: https://github.com/s00d/switchshuttle
- **Releases**: https://github.com/s00d/switchshuttle/releases
- **Issues**: https://github.com/s00d/switchshuttle/issues

## License

This tap is licensed under the MIT License. See the [LICENSE](LICENSE) file for details. 