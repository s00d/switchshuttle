![SwitchShuttle](https://raw.githubusercontent.com/s00d/switchshuttle/main/icons/logo.webp)
![intro](https://github.com/s00d/switchshuttle/blob/main/icons/intro.gif?raw=true)

[[English](README.md)] [[中文](README_ZH.md)] [[Русский](README_RU.md)] [[Deutsch](README_DE.md)] [[日本語](README_JA.md)]

## SwitchShuttle

SwitchShuttle ist eine plattformübergreifende System-Tray-Anwendung, die es Benutzern ermöglicht, vordefinierte Befehle in verschiedenen Terminalanwendungen auszuführen. Es unterstützt macOS, Windows und Linux und bietet eine einfache und anpassbare Möglichkeit, Ihre häufig verwendeten Befehle zu verwalten und auszuführen.

## Über

SwitchShuttle ist eine Neuinterpretation und Erweiterung der [Shuttle](https://github.com/fitztrev/shuttle) Anwendung. Während Shuttle eine einfache und effektive Möglichkeit bietet, Befehlskurzbefehle in macOS zu verwalten, erweitert SwitchShuttle dieses Konzept, indem es Unterstützung für mehrere Betriebssysteme und Terminalemulatoren bietet, zusammen mit erweiterten Konfigurationsmöglichkeiten und Anpassungsoptionen für den Benutzer.

## Funktionen

- Unterstützt mehrere Terminalanwendungen: iTerm, Terminal, Warp.
- Befehle in verschiedenen Modi ausführen: aktuelles Fenster, neuer Tab, neues Fenster.
- Start beim Login umschalten.
- Konfiguration direkt über das Tray-Menü bearbeiten.
- Konfigurationsordner aus dem Tray-Menü öffnen.
- Unterstützung von Untermenüs für eine bessere Organisation der Befehle.
- Unterstützung dynamischer Eingaben für Befehle.
- **Neu**: Kontextmenü mit einer Hotkey-Tastenkombination aufrufen.

## Konfiguration

Die Konfiguration wird in JSON-Dateien im Konfigurationsverzeichnis des Benutzers gespeichert. Der Standardpfad ist `~/.config/switch-shuttle/` auf Linux und macOS und `C:\Users\<Username>\AppData\Roaming\switch-shuttle\` auf Windows. Sie können mehrere Konfigurationsdateien in diesem Verzeichnis speichern, wobei jede eine andere Sammlung von Befehlen und Einstellungen darstellt.

Hier ist ein Beispiel für eine Konfigurationsdatei:

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

### Konfigurationsparameter

| Parameter   | Typ               | Beschreibung                                                       | Gültige Werte                                        |
|-------------|-------------------|--------------------------------------------------------------------|------------------------------------------------------|
| terminal    | String            | Das zu verwendende Terminalprogramm                                | "iterm", "terminal", "warp"                          |
| launch_in   | String            | Wo der Befehl gestartet werden soll                                | "current", "new_tab", "new_window"                   |
| theme       | String            | Das zu verwendende Thema (falls unterstützt)                       | Jeder Zeichenfolgenwert, der ein Thema darstellt     |
| title       | String            | Der Titel für das Terminalfenster/-tab                             | Jeder Zeichenfolgenwert                              |
| menu_hotkey | String (Optional) | Die globale Hotkey-Tastenkombination zum Aufrufen des Kontextmenüs | Jede gültige Hotkey-Kombination, z.B. "Ctrl+Shift+M" |
| commands    | Array             | Liste der Befehlskonfigurationen                                   | Siehe unten für Befehl-Parameter                     |

### Befehl-Parameter

| Parameter | Typ               | Beschreibung                                                  | Gültige Werte                                                   |
|-----------|-------------------|---------------------------------------------------------------|-----------------------------------------------------------------|
| name      | String            | Der Name des Befehls oder Untermenüs                          | Jeder Zeichenfolgenwert                                         |
| inputs    | Object (Optional) | Schlüssel-Wert-Paare für Eingaben                             | {"key1": "default1", "key2": "default2"}                        |
| command   | String (Optional) | Der auszuführende Befehl (falls dies ein Befehl ist)          | Jeder Zeichenfolgenwert, der einen Befehl darstellt             |
| commands  | Array (Optional)  | Liste der nacheinander auszuführenden Befehle                 | Jeder Array von Zeichenfolgen, jede Zeichenfolge ist ein Befehl |
| submenu   | Array (Optional)  | Liste der Unterbefehle (falls dies ein Untermenü ist)         | Siehe oben für Befehl-Parameter                                 |
| hotkey    | String (Optional) | Die globale Hotkey-Tastenkombination zum Auslösen des Befehls | Jede gültige Hotkey-Kombination, z.B. "Ctrl+Shift+E"            |

### Ausführungslogik für Befehle

SwitchShuttle unterstützt die Definition eines einzelnen Befehls mit dem Parameter `command`, einer Liste von Befehlen mit dem Parameter `commands` oder beides. Wenn sowohl `command` als auch `commands` angegeben sind, wird der einzelne Befehl zuerst ausgeführt, gefolgt von den Befehlen in der Liste.

#### Beispiel für die Ausführung

1. **Einzelbefehl**: Wenn nur `command` angegeben ist, wird dieser Befehl ausgeführt.
2. **Mehrere Befehle**: Wenn nur `commands` angegeben ist, wird jeder Befehl in der Liste nacheinander ausgeführt.
3. **Beide Parameter**: Wenn sowohl `command` als auch `commands` angegeben sind, wird der einzelne Befehl zuerst ausgeführt, gefolgt von jedem Befehl in der Liste `commands`.

### Dynamische Eingaben

SwitchShuttle ermöglicht die Definition dynamischer Eingaben für Befehle. Diese Eingaben werden vom Benutzer angefordert, bevor der Befehl ausgeführt wird. Sie können Eingaben mit dem Parameter `inputs` in der Befehlskonfiguration definieren.

#### Beispielkonfiguration mit Eingaben

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

### Hotkeys

Sie können globale Hotkeys für Befehle zuweisen, indem Sie den Parameter `hotkey` zur Befehlskonfiguration hinzufügen. Die Hotkey-Kombination muss dem Format der Modifikatortasten (Ctrl, Shift, Alt, Win) in Kombination mit einer Taste (A-Z, 0-9 usw.) folgen. Zum Beispiel, um "Ctrl+Shift+E" als Hotkey für einen Befehl festzulegen:

```json
{
  "name": "Example Command",
  "command": "echo Hello, world!",
  "submenu": null,
  "hotkey": "Ctrl+Shift+E",
  "commands": null
}
```

Der Parameter `hotkey` ist optional. Wenn er nicht angegeben ist, hat der Befehl keinen globalen Hotkey zugewiesen.

### Verwendung von Hotkeys

1. **Hotkeys zuweisen**: Bearbeiten Sie die Konfigurationsdatei, um den Parameter `hotkey` für die Befehle hinzuzufügen, die Sie mit globalen Hotkeys auslösen möchten.
2. **Hotkeys verwenden**: Verwenden Sie nach dem Neustart der Anwendung die zugewiesenen Hotkeys, um die entsprechenden Befehle auszulösen, unabhängig davon, welche Anwendung derzeit im Fokus steht.

## Anwendung

1. **Edit Config**: Klicken Sie mit der rechten Maustaste auf das Tray-Symbol und wählen Sie Konfiguration, um die Konfigurationsdatei in Ihrem Standardeditor zu öffnen. Passen Sie die Konfiguration nach Bedarf an.
2. **KShow Config Folder**: Klicken Sie mit der rechten Maustaste auf das Tray-Symbol und wählen Sie "Show Config Folder", um das Konfigurationsverzeichnis in Ihrem Dateimanager zu öffnen.
3. **Toggle Launch at Logi**: Klicken Sie mit der rechten Maustaste auf das Tray-Symbol und wählen Sie "Toggle Launch at Login", um das automatische Starten der Anwendung beim Login zu aktivieren oder zu deaktivieren.
4. **Execute Command**: Klicken Sie mit der linken Maustaste auf das Tray-Symbol und wählen Sie den Befehl aus, den Sie ausführen möchten, aus dem Menü. Der Befehl wird im angegebenen Terminalprogramm ausgeführt.

### Erstellen von Untermenüs

Um Untermenüs zu erstellen, setzen Sie das Feld `command` auf `null` und geben Sie eine Liste von Unterbefehlen im Feld `submenu` an. Unterbefehle können auch eigene Untermenüs haben, was verschachtelte Menüs ermöglicht.

## Erstellung der Anwendung

### Voraussetzungen

- [Rust](https://www.rust-lang.org/tools/install)
- [Tauri](https://tauri.app/v1/guides/getting-started/prerequisites/)

### Schritte

1. Klonen Sie das Repository:
   ```sh
   git clone https://github.com/s00d/switchshuttle.git
   cd switchshuttle
   ```

2. Bauen Sie die Anwendung:
   ```sh
   cargo build --release
   ```

3. Führen Sie die Anwendung aus:
   ```sh
   cargo run
   ```

## Download

Die neueste Version von SwitchShuttle kann auf der [GitHub Releases](https://github.com/s00d/switchshuttle/releases) Seite heruntergeladen werden.

### macOS

Anwendung signieren

Wenn Sie macOS verwenden, müssen Sie möglicherweise die Anwendung vor dem Ausführen signieren. Hier sind die Schritte:

1. Machen Sie die Binärdatei ausführbar:

```bash
chmod +x /Applications/switch-shuttle.app
```

2. Löschen Sie erweiterte Attribute und signieren Sie die Binärdatei:

```bash
xattr -cr /Applications/switch-shuttle.app && codesign --force --deep --sign - /Applications/switch-shuttle.app
```

## Beitragen

Beiträge sind willkommen! Bitte senden Sie eine Pull-Request oder öffnen Sie ein Issue auf GitHub.

## Lizenz

Dieses Projekt ist unter der MIT-Lizenz lizenziert. Siehe die [LICENSE](LICENSE) Datei für Details.

---

Viel Spaß beim Verwenden von SwitchShuttle zur einfachen Verwaltung Ihrer Terminalbefehle!