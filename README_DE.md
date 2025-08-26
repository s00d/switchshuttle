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
- **📅 Geplante Befehle** - Automatisierung von Aufgaben mit Cron-Ausdrücken
- **🎯 Template-System** - Vorgefertigte Befehlsvorlagen für häufige Arbeitsabläufe

## 🖥️ Benutzeroberfläche-Übersicht

SwitchShuttle bietet eine moderne, intuitive Benutzeroberfläche mit mehreren Schlüsselkomponenten:

### 🎛️ Hauptkomponenten der Benutzeroberfläche

#### 1. **Konfigurationseditor**
- **Visueller JSON-Editor** - Bearbeiten von Konfigurationen mit Syntaxhervorhebung und Validierung
- **Template-System** - Import vorgefertigter Befehlsvorlagen für häufige Arbeitsabläufe
- **Echtzeit-Validierung** - Sofortiges Feedback zu Konfigurationsfehlern
- **Auto-Save** - Änderungen werden automatisch beim Tippen gespeichert
- **Konfigurationsverwaltung** - Aktivieren/Deaktivieren von Konfigurationen ohne Löschung
- **Suche und Filter** - Schnelles Finden spezifischer Konfigurationen
- **Konfigurationen duplizieren** - Erstellen von Kopien bestehender Konfigurationen für Tests

#### 2. **Befehlsverwaltung**
- **Befehlsgenerator** - Erstellen von Befehlen mit visueller Formularoberfläche
- **Hotkey-Konfiguration** - Festlegen globaler Verknüpfungen für sofortige Befehlsausführung
- **Icon-Auswahl** - Auswahl von Emoji-Icons für bessere visuelle Organisation
- **Eingabefelder** - Konfiguration dynamischer Eingabeaufforderungen für interaktive Befehle
- **Verschachtelte Untermenüs** - Organisation von Befehlen in hierarchische Strukturen
- **Befehlsvalidierung** - Echtzeit-Validierung der Befehls-Syntax

#### 3. **Einstellungsbereich**
- **Terminal-Auswahl** - Auswahl Ihrer bevorzugten Terminal-Anwendung
- **Startmodus** - Konfiguration der Befehlsausführung (aktuell/neuer Tab/neues Fenster)
- **Theme-Einstellungen** - Anpassung des Erscheinungsbilds der Anwendung
- **Auto-Start-Konfiguration** - Aktivieren/Deaktivieren des Systemstarts
- **Globale Hotkey-Einstellungen** - Konfiguration systemweiter Menü-Verknüpfungen

#### 4. **System-Tray-Menü**
- **Schnellzugriff** - Rechtsklick auf Tray-Symbol für sofortigen Befehlszugriff
- **Statusindikatoren** - Visuelles Feedback für Schalter-Befehle und Überwachung
- **Verschachtelte Menüs** - Organisierte Befehls-Hierarchie für einfache Navigation
- **Globale Hotkeys** - Tastenkombinationen für sofortige Befehlsausführung
- **Echtzeit-Überwachung** - Live-Systemressourcen-Indikatoren

### 🎨 Benutzeroberflächen-Features

#### **Visueller Befehlsgenerator**
```json
{
  "name": "🚀 Entwicklungsserver starten",
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

#### **Template-System**
SwitchShuttle enthält vorgefertigte Vorlagen für häufige Entwicklungsarbeitsabläufe:

- **Entwicklung** - Git-Operationen, Build-Tools, Tests
- **DevOps** - Docker, Kubernetes, Serververwaltung
- **Datenbank** - MySQL, PostgreSQL, MongoDB-Operationen
- **Cloud** - AWS, Azure, Google Cloud-Befehle
- **Sicherheit** - Netzwerk-Scanning, Schwachstellenbewertung
- **Überwachung** - Systemressourcen, Logs, Metriken
- **Utilities** - Dateioperationen, Systemtools
- **Scheduler** - Cron-Jobs und automatisierte Aufgaben

#### **Intelligente Organisation**
- **Verschachtelte Untermenüs** - Organisieren Sie Befehle in logische Gruppen
- **Icon-Unterstützung** - Visuelle Identifikation mit Emoji-Icons
- **Hotkey-Verwaltung** - Globale Verknüpfungen für sofortigen Zugriff
- **Statusindikatoren** - Echtzeit-Feedback für Schalter-Befehle
- **Suchfunktion** - Schnelle Befehlsentdeckung

## 🔒 Sicherheits-Manager

SwitchShuttle enthält einen umfassenden Sicherheits-Manager, der Ihr System vor potenziell schädlichen Befehlen schützt und eine detaillierte Kontrolle über die Befehlsausführung bietet.

### 🛡️ Sicherheitsfunktionen

#### **Befehlsvalidierung**
- **Längenlimits**: Maximale Befehlslänge (1000 Zeichen) und Eingabelänge (500 Zeichen) verhindern zu lange Befehle
- **Blockierte Befehle**: Definieren Sie eine Liste gefährlicher Befehle, die niemals ausgeführt werden sollten
- **Verdächtige Muster**: Verwenden Sie Regex-Muster, um potenziell schädliche Befehlsmuster zu erkennen und zu blockieren
- **Echtzeit-Validierung**: Befehle werden während der Bearbeitung validiert, um Sicherheit zu gewährleisten

#### **Sicherheitseinstellungen**
- **Sicherheit aktivieren/deaktivieren**: Schalten Sie Sicherheitsfunktionen nach Bedarf ein oder aus
- **Benutzerdefinierte Blocklisten**: Fügen Sie spezifische Befehle zur Liste der blockierten Befehle hinzu
- **Mustervergleich**: Definieren Sie Regex-Muster, um verdächtige Befehlsstrukturen zu erkennen
- **Längenbeschränkungen**: Konfigurieren Sie maximale Längen für Befehle und Benutzereingaben

#### **Wie es funktioniert**
1. **Editor-Validierung**: Der SecurityManager validiert Befehle im Konfigurationseditor vor dem Speichern
2. **Mustervergleich**: Befehle werden gegen blockierte Muster und verdächtige Regex-Muster geprüft
3. **Längenvalidierung**: Befehle und Eingaben werden gegen maximale Längenlimits verifiziert
4. **Blocklistenprüfung**: Befehle werden mit der benutzerdefinierten Liste blockierter Befehle verglichen
5. **Sichere Konfiguration**: Nur validierte Konfigurationen dürfen gespeichert und verwendet werden



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

1. **Konfigurationseditor öffnen** - Klicken Sie auf "Konfiguration bearbeiten" im System-Tray-Menü
2. **Terminal auswählen** - Wählen Sie Ihre bevorzugte Terminal-Anwendung
3. **Befehle hinzufügen** - Verwenden Sie den visuellen Editor oder importieren Sie Vorlagen
4. **Hotkeys einrichten** - Konfigurieren Sie globale Verknüpfungen für schnellen Zugriff
5. **Speichern und neu starten** - Ihre Befehle sind jetzt im Tray-Menü verfügbar

### Schritt-für-Schritt-Anleitung zur Benutzeroberfläche

#### **Schritt 1: Konfigurationseditor**
- Öffnen Sie SwitchShuttle und navigieren Sie zum "Editor"-Tab
- Wählen Sie Ihre Terminal-Anwendung (iTerm, Terminal, Warp, etc.)
- Stellen Sie den Startmodus ein (aktuelles Fenster, neuer Tab oder neues Fenster)
- Konfigurieren Sie globale Hotkeys für Menüzugriff

#### **Schritt 2: Befehle hinzufügen**
- Klicken Sie auf "Befehl hinzufügen" um einen neuen Befehl zu erstellen
- Füllen Sie die Befehlsdetails aus:
  - **Name**: Anzeigename für den Befehl
  - **Befehl**: Der tatsächliche Terminal-Befehl zum Ausführen
  - **Hotkey**: Globale Verknüpfung (optional)
  - **Icon**: Emoji-Icon für visuelle Identifikation
  - **Hintergrund**: Ob im Hintergrund ausgeführt werden soll
  - **Eingaben**: Dynamische Eingabefelder für interaktive Befehle

#### **Schritt 3: Vorlagen verwenden**
- Klicken Sie auf "Vorlage importieren" für Zugriff auf vorgefertigte Befehls-Sammlungen
- Durchsuchen Sie Kategorien wie Entwicklung, DevOps, Datenbank, etc.
- Wählen und importieren Sie die benötigten Vorlagen
- Passen Sie die importierten Befehle nach Bedarf an

#### **Schritt 4: System-Tray-Zugriff**
- Rechtsklick auf das SwitchShuttle-Tray-Symbol
- Durchsuchen Sie Ihr organisiertes Befehlsmenü
- Verwenden Sie globale Hotkeys für sofortige Befehlsausführung
- Überwachen Sie den Systemstatus mit Echtzeit-Indikatoren

#### **Schritt 5: Erweiterte Funktionen**
- **Schalter-Befehle**: Umschalten von Systemfunktionen mit visuellen Statusindikatoren
- **Überwachungsbefehle**: Echtzeit-Überwachung von Systemressourcen
- **Geplante Befehle**: Automatisierung von Aufgaben mit Cron-Ausdrücken
- **Verschachtelte Menüs**: Organisation von Befehlen in hierarchische Strukturen

## 💻 Kommandozeilen-Interface (CLI)

SwitchShuttle bietet auch ein leistungsstarkes Kommandozeilen-Interface für schnelle Befehlsausführung ohne GUI.

### CLI-Verwendung

#### Befehle ausführen
```bash
# Ausführung nach Befehls-ID
switch-shuttle cmd_8

# Ausführung nach Befehlsname (Groß-/Kleinschreibung ignorierend)
switch-shuttle "Beispiel-Befehl"
```

#### Alle Befehle auflisten
```bash
# Alle verfügbaren Befehle mit ihren IDs anzeigen
switch-shuttle --list
# oder
switch-shuttle -l
```

#### Befehle suchen
```bash
# Nach Befehlen suchen, die bestimmten Text enthalten
switch-shuttle --search "git"
# oder
switch-shuttle -s "docker"
```

### CLI auf verschiedenen Betriebssystemen ausführen

#### macOS
```bash
# Wenn über Homebrew installiert
switch-shuttle --list

# Wenn manuell installiert
/Applications/switch-shuttle.app/Contents/MacOS/SwitchShuttle --list

# Alias für einfacheren Zugriff erstellen
echo 'alias switch-shuttle="/Applications/switch-shuttle.app/Contents/MacOS/SwitchShuttle"' >> ~/.zshrc
source ~/.zshrc
```

#### Windows
```bash
# Wenn über Installer installiert
"C:\Program Files\SwitchShuttle\switch-shuttle.exe" --list

# Wenn über winget oder chocolatey installiert
switch-shuttle --list

# Zu PATH hinzufügen für einfacheren Zugriff
# Fügen Sie "C:\Program Files\SwitchShuttle" zu Ihrem System-PATH hinzu
```

#### Linux
```bash
# Wenn über Paketmanager installiert
switch-shuttle --list

# Wenn manuell installiert
./switch-shuttle --list

# Ausführbar machen und zu PATH hinzufügen
chmod +x switch-shuttle
sudo mv switch-shuttle /usr/local/bin/
```

### CLI-Beispiele

```bash
# Schnelle Git-Operationen
switch-shuttle "git status"
switch-shuttle "git pull"

# Entwicklungs-Workflows
switch-shuttle "npm run dev"
switch-shuttle "docker-compose up"

# Alle verfügbaren Befehle auflisten
switch-shuttle --list

# Nach datenbankbezogenen Befehlen suchen
switch-shuttle --search "database"
```

### CLI-Features

- **🚀 Schnelle Ausführung** - Befehle sofort aus dem Terminal ausführen
- **🔍 Intelligente Suche** - Befehle nach ID oder Namen finden
- **📋 Befehlsauflistung** - Alle verfügbaren Befehle anzeigen
- **⚡ Keine GUI erforderlich** - Perfekt für Automatisierung und Skripte
- **🔄 Beenden nach Ausführung** - Saubere Terminal-Erfahrung

## 📋 Konfigurationsanleitung

### Grundstruktur

SwitchShuttle verwendet JSON-Konfigurationsdateien, gespeichert in:
- **macOS/Linux**: `~/.config/switch-shuttle/`
- **Windows**: `C:\Users\<Benutzername>\AppData\Roaming\switch-shuttle\`

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
      "name": "🔧 Entwicklungs-Tools",
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
  "background": true,
  "hotkey": "Ctrl+Shift+F"
}
```

#### ⏰ Geplante Befehle (Cron)

Planen Sie Befehle für automatische Ausführung mit Cron-Ausdrücken:

```json
{
  "name": "🔄 Auto-Backup",
  "commands": [
    "rsync -av /source/ /backup/"
  ],
  "scheduler": "0 2 * * *",
  "background": true,
  "hotkey": "Ctrl+Shift+B"
}
```

**Cron-Ausdrucksformat:**
Der Scheduler verwendet Standard-Cron-Ausdrücke mit 6 Feldern: `Sekunde Minute Stunde Tag Monat Wochentag`

**Häufige Cron-Beispiele:**
- `"0 0 * * * *"` - Jede Stunde zur Minute 0
- `"0 0 2 * * *"` - Jeden Tag um 2:00 Uhr morgens
- `"0 30 9 * * 1-5"` - Werktags um 9:30 Uhr morgens
- `"0 0 12 * * 1"` - Jeden Montag um 12:00 Uhr
- `"0 0 0 1 * *"` - Ersten Tag jedes Monats
- `"0 */15 * * * *"` - Alle 15 Minuten
- `"0 0 0 * * 0"` - Jeden Sonntag um Mitternacht

**Scheduler-Features:**
- **Hintergrundausführung** - Befehle laufen leise ohne Terminal-Öffnung
- **Cron-Unterstützung** - Vollständiges Cron-Ausdrucks-Parsing und -Ausführung
- **Fehlerbehandlung** - Graceful Fallback bei Cron-Parsing-Fehlern
- **Plattformübergreifend** - Funktioniert auf macOS, Windows und Linux

#### 🖥️ Hintergrundausführung

Kontrollieren Sie die Art der Befehlsausführung - im Hintergrund mit ConsolePool oder normale Terminal-Ausführung:

```json
{
  "name": "🚀 Server starten",
  "commands": [
    "npm run dev"
  ],
  "background": true,
  "hotkey": "Ctrl+Shift+S"
}
```

**Hintergrundausführungsoptionen:**
- `"background": true` - Ausführung mit ConsolePool (Hintergrund)
- `"background": false` - Ausführung mit normaler Terminal-Ausführung
- `"background": null` oder weglassen - Auto-Erkennung basierend auf Befehlsart

#### 📁 Verschachtelte Untermenüs

Organisieren Sie Befehle in hierarchische Menüs:

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

Schalten Sie Systemfunktionen mit Hintergrundausführung um:

```json
{
  "name": "🔧 Systemsteuerungen",
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

**Schalter-Befehls-Features:**
- **Hintergrundausführung** - Befehle laufen leise ohne Terminal-Öffnung
- **Statusprüfung** - Erkennt automatisch den aktuellen Zustand
- **Visuelles Feedback** - Zeigt aktiviert/deaktiviert-Status im Menü
- **Plattformübergreifend** - Funktioniert auf macOS, Windows und Linux

#### 📊 Überwachungsbefehle

Überwachen Sie Systemressourcen und -dienste mit Echtzeit-Informationen:

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
      "name": "💻 CPU-Last",
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

**Überwachungsbefehls-Features:**
- **Menü-Integration** - Überwachungsbuttons zum System-Tray-Menü hinzufügen
- **Befehlsausführung** - Überwachungsbefehle ausführen, wenn Menü geöffnet wird
- **Datenanzeige** - Befehlsausgabe direkt in Menü-Oberfläche anzeigen
- **Visuelle Indikatoren** - Icons und Statusindikatoren im Menü
- **Plattformübergreifend** - Funktioniert auf macOS, Windows und Linux

## ⚙️ Konfigurationsreferenz

### Hauptkonfiguration

| Parameter | Typ | Beschreibung | Standard |
|-----------|-----|--------------|----------|
| `terminal` | String | Zu verwendende Terminal-Anwendung | `"terminal"` |
| `launch_in` | String | Wo Befehle gestartet werden sollen | `"current"` |
| `theme` | String | Terminal-Theme (falls unterstützt) | - |
| `title` | String | Fenster/Tab-Titel | - |
| `menu_hotkey` | String | Globale Hotkey zum Öffnen des Menüs | - |
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
| `commands` | Array | ❌ | Mehrere auszuführende Befehle |
| `submenu` | Array | ❌ | Verschachtelte Unterbefehle |
| `switch` | String | ❌ | Befehl zum Prüfen des Schalter-Status (gibt true/false zurück) |
| `monitor` | String | ❌ | Befehl zum Abrufen des Anzeigewerts für Überwachung |
| `inputs` | Object | ❌ | Dynamische Eingabefelder |
| `hotkey` | String | ❌ | Globale Hotkey-Verknüpfung |
| `icon` | String | ❌ | Emoji-Icon für visuelle Identifikation |
| `background` | Boolean | ❌ | Im Hintergrund ausführen (ConsolePool) oder normales Terminal |
| `scheduler` | String | ❌ | Cron-Ausdruck für geplante Ausführung |

### Konfigurationsverwaltung

#### Konfigurationen aktivieren/deaktivieren

Sie können einzelne Konfigurationsdateien aktivieren oder deaktivieren, um zu steuern, welche Befehle im System-Tray-Menü verfügbar sind. Dies ist nützlich für:

- **Temporäre Deaktivierung** - Konfigurationen deaktivieren ohne Löschung
- **Tests** - Konfigurationen während der Entwicklung aktivieren/deaktivieren
- **Organisation** - Mehrere Konfigurationen behalten, aber nur bestimmte verwenden

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
- **Schnelle Projektnavigation** - Sofort zu verschiedenen Projekten springen
- **Build- und Test-Workflows** - Ein-Klick-Entwicklungszyklen
- **Docker-Verwaltung** - Container mit Hotkeys starten/stoppen
- **Git-Operationen** - Häufige Git-Befehle zur Hand
- **Entwicklungsserver-Verwaltung** - Entwicklungs-Server starten/stoppen
- **Code-Qualitäts-Tools** - Linter, Formatter und Tests ausführen

### 🛠️ DevOps-Ingenieure
- **Serververwaltung** - SSH-Verbindungen und Server-Befehle
- **Überwachungs-Tools** - Schneller Zugriff auf Logs und Metriken
- **Deployment-Skripte** - Automatisierte Deployment-Workflows
- **Datenbank-Operationen** - Häufige Datenbank-Befehle
- **Container-Orchestrierung** - Docker- und Kubernetes-Verwaltung
- **Infrastruktur-Überwachung** - Systemressourcen-Tracking

### 🎨 Designer
- **Asset-Optimierung** - Bildverarbeitung und -optimierung
- **Design-System-Tools** - Komponenten-Generierung und -Updates
- **Prototyp-Server** - Schneller Design-Server-Start
- **Design-Tool-Automatisierung** - Batch-Verarbeitung und Workflows

### 🔧 Systemadministratoren
- **Systemüberwachung** - Echtzeit-Ressourcenüberwachung
- **Dienstverwaltung** - Systemdienste starten/stoppen
- **Backup-Automatisierung** - Geplante Backup-Operationen
- **Netzwerk-Tools** - Netzwerk-Diagnose und -Konfiguration
- **Sicherheits-Tools** - Schwachstellen-Scanning und -Bewertung
- **Wartungsaufgaben** - System-Bereinigung und -Optimierung

## 🔧 Build aus Quellcode

### Voraussetzungen

- [Rust](https://www.rust-lang.org/tools/install) (neueste stabile Version)
- [Node.js](https://nodejs.org/) (v16 oder höher)
- [pnpm](https://pnpm.io/) (empfohlener Paketmanager)
- [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites/)

### Build-Schritte

```bash
# Repository klonen
git clone https://github.com/s00d/switchshuttle.git
cd switchshuttle

# Abhängigkeiten installieren
pnpm install

# Entwicklungsmodus
pnpm run tauri dev

# Für Produktion bauen
pnpm run tauri build
```

### Plattformspezifische Hinweise

#### macOS
```bash
# Bei Signierungsproblemen
chmod +x /Applications/switch-shuttle.app
xattr -cr /Applications/switch-shuttle.app
codesign --force --deep --sign - /Applications/switch-shuttle.app
```

#### Windows
```bash
# Rust und Node.js installieren
# Dann den Build-Schritten oben folgen
```

#### Linux
```bash
# Systemabhängigkeiten installieren
sudo apt-get update
sudo apt-get install libwebkit2gtk-4.0-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev

# Dann den Build-Schritten oben folgen
```

## 🤝 Beitragen

Wir begrüßen Beiträge! So können Sie helfen:

1. **Forken** Sie das Repository
2. **Erstellen** Sie einen Feature-Branch (`git checkout -b feature/amazing-feature`)
3. **Committen** Sie Ihre Änderungen (`git commit -m 'Add amazing feature'`)
4. **Pushen** Sie zum Branch (`git push origin feature/amazing-feature`)
5. **Öffnen** Sie einen Pull Request

### Entwicklungsrichtlinien

- Folgen Sie dem bestehenden Codestil
- Fügen Sie Tests für neue Features hinzu
- Aktualisieren Sie die Dokumentation nach Bedarf
- Stellen Sie plattformübergreifende Kompatibilität sicher
- Verwenden Sie pnpm für Paketverwaltung

### Entwicklungs-Setup

```bash
# Abhängigkeiten installieren
pnpm install

# Entwicklungsserver starten
pnpm run tauri dev

# Typ-Prüfung ausführen
pnpm run type-check

# Für Produktion bauen
pnpm run tauri build
```

## 📄 Lizenz

Dieses Projekt ist unter der MIT-Lizenz lizenziert - siehe die [LICENSE](https://github.com/s00d/switchshuttle/blob/main/LICENSE)-Datei für Details.

## 🙏 Danksagungen

- Inspiriert vom ursprünglichen [Shuttle](https://github.com/fitztrev/shuttle)-Projekt
- Entwickelt mit [Tauri](https://tauri.app/) für plattformübergreifende Desktop-Apps
- UI powered by [Vue.js](https://vuejs.org/)
- Gestylt mit [Tailwind CSS](https://tailwindcss.com/)

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/s00d/switchshuttle/issues)
- **Releases**: [GitHub Releases](https://github.com/s00d/switchshuttle/releases)
- **Dokumentation**: [GitHub Wiki](https://github.com/s00d/switchshuttle/wiki)

---

<div align="center">
  <p>Erstellt mit ❤️ von der SwitchShuttle-Community</p>
  <p>⭐ Sternen Sie dieses Repository, wenn Sie es nützlich finden!</p>
</div>
