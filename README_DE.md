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
  
  **🚀 Plattformübergreifender Terminal-Befehlsmanager mit globalen Hotkeys**

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

## ✨ Was ist SwitchShuttle?

SwitchShuttle ist eine leistungsstarke plattformübergreifende System-Tray-Anwendung, die die Art und Weise revolutioniert, wie Sie Terminal-Befehle verwalten und ausführen. Entwickelt mit modernen Technologien (Rust + Tauri + Vue.js) bietet es eine elegante Oberfläche zur Organisation, Anpassung und zum schnellen Zugriff auf Ihre am häufigsten verwendeten Terminal-Operationen.

### 🎯 Hauptfunktionen

- **🖥️ Multi-Plattform-Unterstützung** - Funktioniert nahtlos auf macOS, Windows und Linux
- **⚡ Globale Hotkeys** - Führen Sie Befehle sofort von überall mit Tastenkombinationen aus
- **🎨 Mehrere Terminal-Unterstützung** - iTerm, Terminal, Warp, Alacritty, Hyper und mehr
- **📁 Intelligente Organisation** - Erstellen Sie verschachtelte Untermenüs für bessere Befehlsorganisation
- **🔧 Dynamische Eingaben** - Interaktive Eingabeaufforderungen für Befehle, die Benutzereingaben benötigen
- **🔄 Mehrere Ausführungsmodi** - Ausführung im aktuellen Fenster, neuer Tab oder neues Fenster
- **🚀 Auto-Start** - Start beim Systemstart für sofortigen Zugriff
- **🎨 Moderne UI** - Schöne, intuitive Oberfläche entwickelt mit Vue.js
- **💻 Kommandozeilen-Interface** - Führen Sie Befehle direkt aus dem Terminal mit CLI aus
- **⚙️ Konfigurationsverwaltung** - Aktivieren/Deaktivieren von Konfigurationen ohne Löschung
- **🔄 Schalter-Befehle** - Systemfunktionen mit Hintergrundausführung umschalten
- **📊 Überwachungsbefehle** - Echtzeit-Überwachung von Systemressourcen mit visuellen Indikatoren

## 🚀 Schnellstart

### Download & Installation

#### Option 1: Homebrew (macOS - Empfohlen)
```bash
# Installation über Homebrew
brew tap s00d/switchshuttle
brew install --cask switchshuttle
```

#### Option 2: Manueller Download
1. **Laden Sie** die neueste Version für Ihre Plattform von [GitHub Releases](https://github.com/s00d/switchshuttle/releases) herunter
2. **Installieren Sie** die Anwendung
3. **Starten Sie** SwitchShuttle - es erscheint in Ihrem System-Tray
4. **Rechtsklick** auf das Tray-Symbol für den Menüzugriff

### Erste Konfiguration

1. **Konfiguration bearbeiten** → Öffnet Ihre Konfigurationsdatei in Ihrem Standardeditor
2. **Fügen Sie Ihre Befehle** mit dem JSON-Format hinzu (siehe Beispiele unten)
3. **Speichern und neu starten** Sie die Anwendung
4. **Genießen Sie** Ihre organisierten Befehlsverknüpfungen!

## 📋 Konfigurationsanleitung

### Grundstruktur

SwitchShuttle verwendet JSON-Konfigurationsdateien, gespeichert in:
- **macOS/Linux**: `~/.config/switch-shuttle/`
- **Windows**: `C:\Users\<Username>\AppData\Roaming\switch-shuttle\`

### Einfaches Beispiel

```json
{
  "terminal": "iterm",
  "launch_in": "new_tab",
  "title": "Meine Befehle",
  "commands": [
    {
      "name": "🚀 Entwicklungsserver starten",
      "command": "npm run dev",
      "hotkey": "Ctrl+Shift+D"
    },
    {
      "name": "📦 Abhängigkeiten installieren",
      "command": "npm install",
      "hotkey": "Ctrl+Shift+I"
    },
    {
      "name": "🔧 Entwicklungstools",
      "submenu": [
        {
          "name": "🧪 Tests ausführen",
          "command": "npm test",
          "hotkey": "Ctrl+Shift+T"
        },
        {
          "name": "📊 Projekt bauen",
          "command": "npm run build",
          "hotkey": "Ctrl+Shift+B"
        }
      ]
    }
  ]
}
```

### Erweiterte Funktionen

#### 🔧 Dynamische Eingaben

Erstellen Sie interaktive Befehle, die nach Benutzereingaben fragen:

```json
{
  "name": "📝 Neue Komponente erstellen",
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

#### 🔄 Mehrere Befehle

Führen Sie eine Befehlssequenz aus:

```json
{
  "name": "🔄 Vollständiger Entwicklungszyklus",
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

#### 📁 Verschachtelte Untermenüs

Organisieren Sie Befehle in hierarchischen Menüs:

```json
{
  "name": "🐳 Docker-Operationen",
  "submenu": [
    {
      "name": "🚀 Dienste starten",
      "submenu": [
        {
          "name": "🏗️ Entwicklung",
          "command": "docker-compose -f docker-compose.dev.yml up -d"
        },
        {
          "name": "🏭 Produktion",
          "command": "docker-compose -f docker-compose.prod.yml up -d"
        }
      ]
    },
    {
      "name": "🛑 Alle stoppen",
      "command": "docker-compose down"
    }
  ]
}
```

#### 🔄 Schalter-Befehle

Systemfunktionen mit Hintergrundausführung umschalten:

```json
{
  "name": "🔧 Systemsteuerung",
  "submenu": [
    {
      "name": "📶 WiFi umschalten",
      "command": "networksetup -setairportpower en0 toggle",
      "switch": "networksetup -getairportpower en0 | grep -q 'On' && echo 'true' || echo 'false'"
    },
    {
      "name": "🔊 Bluetooth umschalten",
      "command": "blueutil -p toggle",
      "switch": "blueutil -p | grep -q '1' && echo 'true' || echo 'false'"
    },
    {
      "name": "🌙 Dunklen Modus umschalten",
      "command": "osascript -e 'tell app \"System Events\" to tell appearance preferences to set dark mode to not dark mode'",
      "switch": "osascript -e 'tell app \"System Events\" to tell appearance preferences to get dark mode'"
    }
  ]
}
```

**Schalter-Befehl-Funktionen:**
- **Hintergrundausführung** - Befehle werden leise ausgeführt ohne Terminal zu öffnen
- **Statusprüfung** - Erkennt automatisch den aktuellen Zustand
- **Visuelles Feedback** - Zeigt aktiviert/deaktiviert Status im Menü
- **Plattformübergreifend** - Funktioniert auf macOS, Windows und Linux

#### 📊 Überwachungsbefehle

Überwachung von Systemressourcen und Diensten mit Echtzeit-Informationen:

```json
{
  "name": "📊 Systemüberwachung",
  "submenu": [
    {
      "name": "💾 Speichernutzung",
      "command": "top -l 1 | head -n 10",
      "monitor": "memory",
      "icon": "🧠"
    },
    {
      "name": "💻 CPU-Auslastung",
      "command": "top -l 1 | grep 'CPU usage'",
      "monitor": "cpu",
      "icon": "⚡"
    },
    {
      "name": "💾 Festplattenspeicher",
      "command": "df -h | grep '/dev/'",
      "monitor": "disk",
      "icon": "💾"
    },
    {
      "name": "🌐 Netzwerkstatus",
      "command": "ifconfig | grep -E 'inet |status:'",
      "monitor": "network",
      "icon": "🌐"
    }
  ]
}
```

**Überwachungsbefehle Funktionen:**
- **Menüintegration** - Hinzufügen von Überwachungsbuttons zum System-Tray-Menü
- **Befehlsausführung** - Ausführung von Überwachungsbefehlen beim Öffnen des Menüs
- **Datenanzeige** - Anzeige der Befehlsausgabe direkt in der Menüoberfläche
- **Visuelle Indikatoren** - Icons und Statusindikatoren im Menü
- **Plattformübergreifend** - Funktioniert auf macOS, Windows und Linux

## ⚙️ Konfigurationsreferenz

### Hauptkonfiguration

| Parameter | Typ | Beschreibung | Standard |
|-----------|-----|--------------|----------|
| `terminal` | String | Zu verwendende Terminal-Anwendung | `"terminal"` |
| `launch_in` | String | Wo Befehle gestartet werden sollen | `"current"` |
| `theme` | String | Terminal-Thema (falls unterstützt) | - |
| `title` | String | Fenster/Tab-Titel | - |
| `menu_hotkey` | String | Globaler Hotkey zum Öffnen des Menüs | - |
| `commands` | Array | Liste der Befehls-Konfigurationen | `[]` |
| `enabled` | Boolean | Ob diese Konfiguration geladen werden soll | `true` |

### Terminal-Optionen

| Terminal | macOS | Windows | Linux |
|----------|-------|---------|-------|
| `iterm` | ✅ | ❌ | ❌ |
| `terminal` | ✅ | ✅ | ✅ |
| `warp` | ✅ | ❌ | ❌ |
| `alacritty` | ✅ | ✅ | ✅ |
| `hyper` | ✅ | ✅ | ✅ |

### Startmodi

| Modus | Beschreibung |
|-------|--------------|
| `current` | Im aktuellen Terminal-Fenster ausführen |
| `new_tab` | Neuen Tab öffnen und ausführen |
| `new_window` | Neues Fenster öffnen und ausführen |

### Befehls-Konfiguration

| Parameter | Typ | Erforderlich | Beschreibung |
|-----------|-----|--------------|--------------|
| `name` | String | ✅ | Anzeigename für den Befehl |
| `command` | String | ❌ | Einzelner auszuführender Befehl |
| `commands` | Array | ❌ | Mehrere auszuführende Befehle |
| `submenu` | Array | ❌ | Verschachtelte Unterbefehle |
| `inputs` | Object | ❌ | Dynamische Eingabefelder |
| `hotkey` | String | ❌ | Globaler Hotkey |
| `switch` | String | ❌ | Befehl zum Überprüfen des aktuellen Status (für Schalter-Befehle) |
| `monitor` | String | ❌ | Überwachungstyp für Echtzeit-Ressourcenverfolgung |
| `icon` | String | ❌ | Emoji-Icon zur visuellen Identifikation |

### Konfigurationsverwaltung

#### Konfigurationen aktivieren/deaktivieren

Sie können einzelne Konfigurationsdateien aktivieren oder deaktivieren, um zu steuern, welche Befehle im System-Tray-Menü verfügbar sind. Dies ist nützlich für:

- **Temporäres Deaktivieren** - Deaktivieren Sie Konfigurationen ohne Löschung
- **Tests** - Aktivieren/Deaktivieren von Konfigurationen während der Entwicklung
- **Organisation** - Behalten Sie mehrere Konfigurationen, verwenden Sie aber nur bestimmte

**Im visuellen Editor:**
- Öffnen Sie den Konfigurationseditor
- Verwenden Sie den Umschalter im Abschnitt "Konfigurationsstatus"
- Aktivierte Konfigurationen werden geladen und im Menü verfügbar sein
- Deaktivierte Konfigurationen werden ignoriert

**In JSON-Konfiguration:**
```json
{
  "terminal": "iterm",
  "launch_in": "current",
  "title": "Meine Befehle",
  "enabled": true,
  "commands": [
    {
      "name": "Beispiel-Befehl",
      "command": "echo Hello World"
    }
  ]
}
```

| Parameter | Typ | Standard | Beschreibung |
|-----------|-----|----------|--------------|
| `enabled` | Boolean | `true` | Ob diese Konfiguration geladen und im Menü verfügbar sein soll |

**Hinweis:** Wenn `enabled` auf `false` gesetzt oder weggelassen wird, wird die Konfiguration ignoriert und ihre Befehle erscheinen nicht im System-Tray-Menü.

## 🎯 Anwendungsfälle

### 👨‍💻 Entwickler
- **Schnelle Projektnavigation** - Springen Sie sofort zu verschiedenen Projekten
- **Build- und Test-Workflows** - Entwicklungszyklen mit einem Klick
- **Docker-Verwaltung** - Container mit Hotkeys starten/stoppen
- **Git-Operationen** - Häufige Git-Befehle griffbereit

### 🛠️ DevOps-Ingenieure
- **Serververwaltung** - SSH-Verbindungen und Server-Befehle
- **Überwachungstools** - Schneller Zugriff auf Logs und Metriken
- **Deployment-Skripte** - Automatisierte Deployment-Workflows
- **Datenbankoperationen** - Häufige Datenbank-Befehle

### 🎨 Designer
- **Asset-Optimierung** - Bildverarbeitung und -optimierung
- **Design-System-Tools** - Komponenten-Generierung und -Updates
- **Prototyp-Server** - Schneller Design-Server-Start

## 🔧 Build aus Quellcode

### Voraussetzungen

- [Rust](https://www.rust-lang.org/tools/install) (neueste stabile Version)
- [Node.js](https://nodejs.org/) (v16 oder höher)
- [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites/)

### Build-Schritte

```bash
# Repository klonen
git clone https://github.com/s00d/switchshuttle.git
cd switchshuttle

# Abhängigkeiten installieren
npm install

# Entwicklungsmodus
npm run tauri dev

# Für Produktion bauen
npm run tauri build
```

### Plattformspezifische Hinweise

#### macOS
```bash
# Bei Signierungsproblemen
chmod +x /Applications/switch-shuttle.app
xattr -cr /Applications/switch-shuttle.app
codesign --force --deep --sign - /Applications/switch-shuttle.app
```

## 🤝 Beitragen

Wir freuen uns über Beiträge! So können Sie helfen:

1. **Forken Sie** das Repository
2. **Erstellen Sie** einen Feature-Branch (`git checkout -b feature/amazing-feature`)
3. **Committen Sie** Ihre Änderungen (`git commit -m 'Add amazing feature'`)
4. **Pushen Sie** zum Branch (`git push origin feature/amazing-feature`)
5. **Öffnen Sie** einen Pull Request

### Entwicklungsrichtlinien

- Folgen Sie dem bestehenden Codestil
- Fügen Sie Tests für neue Features hinzu
- Aktualisieren Sie die Dokumentation nach Bedarf
- Stellen Sie plattformübergreifende Kompatibilität sicher

## 📄 Lizenz

Dieses Projekt ist unter der MIT-Lizenz lizenziert - siehe [LICENSE](https://github.com/s00d/switchshuttle/blob/main/LICENSE) Datei für Details.

## 🙏 Danksagungen

- Inspiriert vom ursprünglichen [Shuttle](https://github.com/fitztrev/shuttle) Projekt
- Entwickelt mit [Tauri](https://tauri.app/) für plattformübergreifende Desktop-Apps
- UI powered by [Vue.js](https://vuejs.org/)

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/s00d/switchshuttle/issues)
- **Releases**: [GitHub Releases](https://github.com/s00d/switchshuttle/releases)

---

<div align="center">
  <p>Entwickelt mit ❤️ von der SwitchShuttle-Community</p>
  <p>⭐ Geben Sie diesem Repository einen Stern, wenn es Ihnen nützlich ist!</p>
</div>
