<div align="center">
  
  [![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Windows%20%7C%20Linux-blue?style=for-the-badge)](https://github.com/s00d/switchshuttle)
  [![License](https://img.shields.io/badge/license-MIT-green?style=for-the-badge)](https://github.com/s00d/switchshuttle/blob/main/LICENSE)
  [![GitHub release](https://img.shields.io/github/v/release/s00d/switchshuttle?style=for-the-badge)](https://github.com/s00d/switchshuttle/releases)
  [![GitHub downloads](https://img.shields.io/github/downloads/s00d/switchshuttle/total?style=for-the-badge)](https://github.com/s00d/switchshuttle/releases)
  [![GitHub issues](https://img.shields.io/badge/github-issues-orange?style=for-the-badge)](https://github.com/s00d/switchshuttle/issues)
  [![GitHub stars](https://img.shields.io/badge/github-stars-yellow?style=for-the-badge)](https://github.com/s00d/switchshuttle/stargazers)
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
- **💻 Command Line Interface** - Execute commands directly from terminal with CLI
- **⚙️ Configuration Management** - Enable/disable configurations without deleting them
- **🔄 Switch Commands** - Toggle system features with background execution
- **📊 Monitor Commands** - Real-time system resource monitoring with visual indicators
- **📅 Scheduled Commands** - Automate tasks with cron expressions
- **🎯 Template System** - Pre-built command templates for common workflows

## 🖥️ User Interface Overview

SwitchShuttle provides a modern, intuitive interface with several key components:

### 🎛️ Main Interface Components

#### 1. **Configuration Editor**
- **Visual JSON Editor** - Edit configurations with syntax highlighting and validation
- **Template System** - Import pre-built command templates for common workflows
- **Real-time Validation** - Instant feedback on configuration errors
- **Auto-save** - Changes are automatically saved as you type
- **Configuration Management** - Enable/disable configurations without deleting them
- **Search and Filter** - Quickly find specific configurations
- **Duplicate Configurations** - Create copies of existing configurations for testing

#### 2. **Command Management**
- **Command Builder** - Create commands with visual form interface
- **Hotkey Configuration** - Set global shortcuts for instant command execution
- **Icon Selector** - Choose emoji icons for better visual organization
- **Input Fields** - Configure dynamic input prompts for interactive commands
- **Nested Submenus** - Organize commands in hierarchical structures
- **Command Validation** - Real-time validation of command syntax

#### 3. **Settings Panel**
- **Terminal Selection** - Choose your preferred terminal application
- **Launch Mode** - Configure how commands are executed (current/new tab/new window)
- **Theme Settings** - Customize the application appearance
- **Auto-start Configuration** - Enable/disable system startup
- **Global Hotkey Settings** - Configure system-wide menu shortcuts

#### 4. **System Tray Menu**
- **Quick Access** - Right-click tray icon for instant command access
- **Status Indicators** - Visual feedback for switch commands and monitoring
- **Nested Menus** - Organized command hierarchy for easy navigation
- **Global Hotkeys** - Keyboard shortcuts for immediate command execution
- **Real-time Monitoring** - Live system resource indicators

### 🎨 Interface Features

#### **Visual Command Builder**
```json
{
  "name": "🚀 Start Development Server",
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

#### **Template System**
SwitchShuttle includes pre-built templates for common development workflows:

- **Development** - Git operations, build tools, testing
- **DevOps** - Docker, Kubernetes, server management
- **Database** - MySQL, PostgreSQL, MongoDB operations
- **Cloud** - AWS, Azure, Google Cloud commands
- **Security** - Network scanning, vulnerability assessment
- **Monitoring** - System resources, logs, metrics
- **Utility** - File operations, system tools
- **Scheduler** - Cron jobs and automated tasks

#### **Smart Organization**
- **Nested Submenus** - Organize commands in logical groups
- **Icon Support** - Visual identification with emoji icons
- **Hotkey Management** - Global shortcuts for instant access
- **Status Indicators** - Real-time feedback for switch commands
- **Search Functionality** - Quick command discovery

## 🔒 Security Manager

SwitchShuttle includes a comprehensive Security Manager that protects your system from potentially harmful commands and provides fine-grained control over command execution.

### 🛡️ Security Features

#### **Command Validation**
- **Length Limits**: Maximum command length (1000 characters) and input length (500 characters) prevent overly long commands
- **Blocked Commands**: Define a list of dangerous commands that should never be executed
- **Suspicious Patterns**: Use regex patterns to detect and block potentially harmful command patterns
- **Real-time Validation**: Commands are validated during editing to ensure safety

#### **Security Settings**
- **Enable/Disable Security**: Toggle security features on or off as needed
- **Custom Block Lists**: Add specific commands to the blocked commands list
- **Pattern Matching**: Define regex patterns to catch suspicious command structures
- **Length Restrictions**: Configure maximum lengths for commands and user inputs

#### **How It Works**
1. **Editor Validation**: SecurityManager validates commands in the configuration editor before saving
2. **Pattern Matching**: Commands are checked against blocked patterns and suspicious regex patterns
3. **Length Validation**: Commands and inputs are verified against maximum length limits
4. **Block List Check**: Commands are compared against the user-defined blocked commands list
5. **Safe Configuration**: Only validated configurations are allowed to be saved and used



## 🚀 Quick Start

### Download & Install

#### Option 1: Homebrew (macOS - Recommended)
```bash
# Install via Homebrew
brew tap s00d/switchshuttle
brew install --cask switchshuttle
```

#### Option 2: Manual Download
1. **Download** the latest release for your platform from [GitHub Releases](https://github.com/s00d/switchshuttle/releases)
2. **Install** the application
3. **Launch** SwitchShuttle - it will appear in your system tray
4. **Right-click** the tray icon to access the menu

### First Configuration

1. **Open Configuration Editor** - Click "Edit Config" in the system tray menu
2. **Choose Terminal** - Select your preferred terminal application
3. **Add Commands** - Use the visual editor or import templates
4. **Set Hotkeys** - Configure global shortcuts for quick access
5. **Save and Restart** - Your commands are now available in the tray menu

### Interface Walkthrough

#### **Step 1: Configuration Editor**
- Open SwitchShuttle and navigate to the "Editor" tab
- Choose your terminal application (iTerm, Terminal, Warp, etc.)
- Set the launch mode (current window, new tab, or new window)
- Configure global hotkeys for menu access

#### **Step 2: Adding Commands**
- Click "Add Command" to create a new command
- Fill in the command details:
  - **Name**: Display name for the command
  - **Command**: The actual terminal command to execute
  - **Hotkey**: Global shortcut (optional)
  - **Icon**: Emoji icon for visual identification
  - **Background**: Whether to run in background
  - **Inputs**: Dynamic input fields for interactive commands

#### **Step 3: Using Templates**
- Click "Import Template" to access pre-built command collections
- Browse categories like Development, DevOps, Database, etc.
- Select and import the templates you need
- Customize the imported commands as needed

#### **Step 4: System Tray Access**
- Right-click the SwitchShuttle tray icon
- Browse your organized command menu
- Use global hotkeys for instant command execution
- Monitor system status with real-time indicators

#### **Step 5: Advanced Features**
- **Switch Commands**: Toggle system features with visual status indicators
- **Monitor Commands**: Real-time system resource monitoring
- **Scheduled Commands**: Automate tasks with cron expressions
- **Nested Menus**: Organize commands in hierarchical structures

## 🖥️ Command Line Interface (CLI)

SwitchShuttle also provides a powerful command-line interface for quick command execution without opening the GUI.

### CLI Usage

#### Execute Commands
```bash
# Execute by command ID
switch-shuttle cmd_8

# Execute by command name (case-insensitive)
switch-shuttle "Example Command"
```

#### List All Commands
```bash
# Show all available commands with their IDs
switch-shuttle --list
# or
switch-shuttle -l
```

#### Search Commands
```bash
# Search for commands containing specific text
switch-shuttle --search "git"
# or
switch-shuttle -s "docker"
```

### Running CLI on Different Operating Systems

#### macOS
```bash
# If installed via Homebrew
switch-shuttle --list

# If installed manually
/Applications/switch-shuttle.app/Contents/MacOS/SwitchShuttle --list

# Create an alias for easier access
echo 'alias switch-shuttle="/Applications/switch-shuttle.app/Contents/MacOS/SwitchShuttle"' >> ~/.zshrc
source ~/.zshrc
```

#### Windows
```bash
# If installed via installer
"C:\Program Files\SwitchShuttle\switch-shuttle.exe" --list

# If installed via winget or chocolatey
switch-shuttle --list

# Add to PATH for easier access
# Add "C:\Program Files\SwitchShuttle" to your system PATH
```

#### Linux
```bash
# If installed via package manager
switch-shuttle --list

# If installed manually
./switch-shuttle --list

# Make executable and add to PATH
chmod +x switch-shuttle
sudo mv switch-shuttle /usr/local/bin/
```

### CLI Examples

```bash
# Quick git operations
switch-shuttle "git status"
switch-shuttle "git pull"

# Development workflows
switch-shuttle "npm run dev"
switch-shuttle "docker-compose up"

# List all available commands
switch-shuttle --list

# Find commands related to database
switch-shuttle --search "database"
```

### CLI Features

- **🚀 Fast Execution** - Run commands instantly from terminal
- **🔍 Smart Search** - Find commands by ID or name
- **📋 Command Listing** - View all available commands
- **⚡ No GUI Required** - Perfect for automation and scripts
- **🔄 Exit After Execution** - Clean terminal experience

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
  "background": true,
  "hotkey": "Ctrl+Shift+F"
}
```

#### ⏰ Scheduled Commands (Cron)

Schedule commands to run automatically using cron expressions:

```json
{
  "name": "🔄 Auto Backup",
  "commands": [
    "rsync -av /source/ /backup/"
  ],
  "scheduler": "0 2 * * *",
  "background": true,
  "hotkey": "Ctrl+Shift+B"
}
```

**Cron Expression Format:**
The scheduler uses standard cron expressions with 6 fields: `second minute hour day month weekday`

**Common Cron Examples:**
- `"0 0 * * * *"` - Every hour at minute 0
- `"0 0 2 * * *"` - Every day at 2:00 AM
- `"0 30 9 * * 1-5"` - Weekdays at 9:30 AM
- `"0 0 12 * * 1"` - Every Monday at noon
- `"0 0 0 1 * *"` - First day of every month
- `"0 */15 * * * *"` - Every 15 minutes
- `"0 0 0 * * 0"` - Every Sunday at midnight

**Scheduler Features:**
- **Background Execution** - Commands run silently without opening terminal
- **Cron Support** - Full cron expression parsing and execution
- **Error Handling** - Graceful fallback if cron parsing fails
- **Cross-Platform** - Works on macOS, Windows, and Linux

#### 🖥️ Background Execution

Control how commands are executed - in background using ConsolePool or normal terminal execution:

```json
{
  "name": "🚀 Start Server",
  "commands": [
    "npm run dev"
  ],
  "background": true,
  "hotkey": "Ctrl+Shift+S"
}
```

**Background Execution Options:**
- `"background": true` - Execute using ConsolePool (background)
- `"background": false` - Execute using normal terminal execution
- `"background": null` or omit - Auto-detect based on command type

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

#### 🔄 Switch Commands

Toggle system features with background execution:

```json
{
  "name": "🔧 System Controls",
  "submenu": [
    {
      "name": "📶 Toggle WiFi",
      "command": "networksetup -setairportpower en0 toggle",
      "switch": "networksetup -getairportpower en0 | grep -q 'On' && echo 'true' || echo 'false'"
    },
    {
      "name": "🔊 Toggle Bluetooth",
      "command": "blueutil -p toggle",
      "switch": "blueutil -p | grep -q '1' && echo 'true' || echo 'false'"
    },
    {
      "name": "🌙 Toggle Dark Mode",
      "command": "osascript -e 'tell app \"System Events\" to tell appearance preferences to set dark mode to not dark mode'",
      "switch": "osascript -e 'tell app \"System Events\" to tell appearance preferences to get dark mode'"
    }
  ]
}
```

**Switch Command Features:**
- **Background Execution** - Commands run silently without opening terminal
- **Status Checking** - Automatically detects current state
- **Visual Feedback** - Shows enabled/disabled status in menu
- **Cross-Platform** - Works on macOS, Windows, and Linux

#### 📊 Monitor Commands

Monitor system resources and services with real-time information:

```json
{
  "name": "📊 System Monitoring",
  "submenu": [
    {
      "name": "💾 Memory Usage",
      "command": "top -l 1 | head -n 10",
      "monitor": "memory",
      "icon": "🧠"
    },
    {
      "name": "💻 CPU Load",
      "command": "top -l 1 | grep 'CPU usage'",
      "monitor": "cpu",
      "icon": "⚡"
    },
    {
      "name": "💾 Disk Space",
      "command": "df -h | grep '/dev/'",
      "monitor": "disk",
      "icon": "💾"
    },
    {
      "name": "🌐 Network Status",
      "command": "ifconfig | grep -E 'inet |status:'",
      "monitor": "network",
      "icon": "🌐"
    }
  ]
}
```

**Monitor Command Features:**
- **Menu Integration** - Add monitoring buttons to system tray menu
- **Command Execution** - Execute monitoring commands when menu opens
- **Data Display** - Show command output directly in menu interface
- **Visual Indicators** - Icons and status indicators in menu
- **Cross-Platform** - Works on macOS, Windows, and Linux

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
| `enabled` | Boolean | Whether this configuration should be loaded | `true` |

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
| `commands` | Array | ❌ | Multiple commands to execute |
| `submenu` | Array | ❌ | Nested subcommands |
| `switch` | String | ❌ | Command to check switch state (returns true/false) |
| `monitor` | String | ❌ | Command to get display value for monitoring |
| `inputs` | Object | ❌ | Dynamic input fields |
| `hotkey` | String | ❌ | Global hotkey shortcut |
| `icon` | String | ❌ | Emoji icon for visual identification |
| `background` | Boolean | ❌ | Execute in background (ConsolePool) or normal terminal |
| `scheduler` | String | ❌ | Cron expression for scheduled execution |

### Configuration Management

#### Enable/Disable Configurations

You can enable or disable individual configuration files to control which commands are available in the system tray menu. This is useful for:

- **Temporary disabling** - Disable configurations without deleting them
- **Testing** - Enable/disable configurations during development
- **Organization** - Keep multiple configurations but only use specific ones

**In the Visual Editor:**
- Open the Configuration Editor
- Use the toggle switch in the "Configuration Status" section
- Enabled configurations will be loaded and available in the menu
- Disabled configurations will be ignored

**In JSON Configuration:**
```json
{
  "terminal": "iterm",
  "launch_in": "current",
  "title": "My Commands",
  "enabled": true,
  "commands": [
    {
      "name": "Example Command",
      "command": "echo Hello World"
    }
  ]
}
```

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `enabled` | Boolean | `true` | Whether this configuration should be loaded and available in the menu |

**Note:** When `enabled` is set to `false` or omitted, the configuration will be ignored and its commands won't appear in the system tray menu.

## 🎯 Use Cases

### 👨‍💻 Developers
- **Quick project navigation** - Jump to different projects instantly
- **Build and test workflows** - One-click development cycles
- **Docker management** - Start/stop containers with hotkeys
- **Git operations** - Common git commands at your fingertips
- **Development server management** - Start/stop development servers
- **Code quality tools** - Run linters, formatters, and tests

### 🛠️ DevOps Engineers
- **Server management** - SSH connections and server commands
- **Monitoring tools** - Quick access to logs and metrics
- **Deployment scripts** - Automated deployment workflows
- **Database operations** - Common database commands
- **Container orchestration** - Docker and Kubernetes management
- **Infrastructure monitoring** - System resource tracking

### 🎨 Designers
- **Asset optimization** - Image processing and optimization
- **Design system tools** - Component generation and updates
- **Prototype servers** - Quick design server startup
- **Design tool automation** - Batch processing and workflows

### 🔧 System Administrators
- **System monitoring** - Real-time resource monitoring
- **Service management** - Start/stop system services
- **Backup automation** - Scheduled backup operations
- **Network tools** - Network diagnostics and configuration
- **Security tools** - Vulnerability scanning and assessment
- **Maintenance tasks** - System cleanup and optimization

## 🔧 Building from Source

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [Node.js](https://nodejs.org/) (v16 or higher)
- [pnpm](https://pnpm.io/) (recommended package manager)
- [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites/)

### Build Steps

```bash
# Clone the repository
git clone https://github.com/s00d/switchshuttle.git
cd switchshuttle

# Install dependencies
pnpm install

# Development mode
pnpm run tauri dev

# Build for production
pnpm run tauri build
```

### Platform-Specific Notes

#### macOS
```bash
# If you encounter signing issues
chmod +x /Applications/switch-shuttle.app
xattr -cr /Applications/switch-shuttle.app
codesign --force --deep --sign - /Applications/switch-shuttle.app
```

#### Windows
```bash
# Install Rust and Node.js
# Then follow the build steps above
```

#### Linux
```bash
# Install system dependencies
sudo apt-get update
sudo apt-get install libwebkit2gtk-4.0-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev

# Then follow the build steps above
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
- Use pnpm for package management

### Development Setup

```bash
# Install dependencies
pnpm install

# Start development server
pnpm run tauri dev

# Run type checking
pnpm run type-check

# Build for production
pnpm run tauri build
```

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](https://github.com/s00d/switchshuttle/blob/main/LICENSE) file for details.

## 🙏 Acknowledgments

- Inspired by the original [Shuttle](https://github.com/fitztrev/shuttle) project
- Built with [Tauri](https://tauri.app/) for cross-platform desktop apps
- UI powered by [Vue.js](https://vuejs.org/)
- Styled with [Tailwind CSS](https://tailwindcss.com/)

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/s00d/switchshuttle/issues)
- **Releases**: [GitHub Releases](https://github.com/s00d/switchshuttle/releases)
- **Documentation**: [GitHub Wiki](https://github.com/s00d/switchshuttle/wiki)

---

<div align="center">
  <p>Made with ❤️ by the SwitchShuttle community</p>
  <p>⭐ Star this repository if you find it useful!</p>
</div>
