![SwitchShuttle](https://raw.githubusercontent.com/s00d/switchshuttle/main/logo.webp)

# SwitchShuttle

SwitchShuttle is a cross-platform system tray application that allows users to run predefined commands in various terminal applications. It supports macOS, Windows, and Linux, offering a simple and customizable way to manage and execute your frequently used commands.


## About

SwitchShuttle is a reimagining and extension of the [Shuttle](https://github.com/fitztrev/shuttle) application. While Shuttle provides a simple and effective way to manage commands shortcuts in macOS, SwitchShuttle expands upon this concept, offering support for multiple operating systems and terminal emulators, along with enhanced configuration capabilities and user customization options.


## Features

- Supports multiple terminal applications: iTerm, Terminal, Warp, Starship.
- Run commands in different modes: current window, new tab, new window.
- Toggle launch at login.
- Edit configuration directly from the tray menu.
- Open configuration folder from the tray menu.
- Supports submenus for better organization of commands.

## Configuration

The configuration is stored in a JSON file located in the user's configuration directory. The default path is `~/.config/switch-shuttle/config.json` on Linux and macOS, and `C:\Users\<Username>\AppData\Roaming\switch-shuttle\config.json` on Windows.

Here is an example of the configuration file:

```json
{
  "terminal": "iterm",
  "launch_in": "new_tab",
  "theme": "Homebrew",
  "title": "New tab",
  "commands": [
    {
      "name": "Example Command",
      "command": "echo Hello, world!",
      "submenu": null
    },
    {
      "name": "Example Submenu",
      "command": null,
      "submenu": [
        {
          "name": "Subcommand 1",
          "command": "echo Subcommand 1",
          "submenu": null
        },
        {
          "name": "Subcommand 2",
          "command": "echo Subcommand 2",
          "submenu": null
        }
      ]
    }
  ]
}
```

### Configuration Parameters

| Parameter | Type   | Description                                     | Valid Values                            |
|-----------|--------|-------------------------------------------------|-----------------------------------------|
| terminal  | String | The terminal application to use                 | "iterm", "terminal", "warp", "starship" |
| launch_in | String | Where to launch the command                     | "current", "new_tab", "new_window"      |
| theme     | String | The theme to use (if supported by the terminal) | Any string value representing a theme   |
| title     | String | The title to set for the terminal window/tab    | Any string value                        |
| commands  | Array  | List of command configurations                  | See below for command parameters        |

### Command Parameters

| Parameter | Type              | Description                                   | Valid Values                            |
|-----------|-------------------|-----------------------------------------------|-----------------------------------------|
| name      | String            | The name of the command or submenu            | Any string value                        |
| command   | String (Optional) | The command to execute (if this is a command) | Any string value representing a command |
| submenu   | Array (Optional)  | List of subcommands (if this is a submenu)    | See above for command parameters        |

## How to Use

1. **Edit Config**: Right-click the tray icon and select "Edit Config" to open the configuration file in your default editor. Modify the configuration as needed.
2. **Open Config Folder**: Right-click the tray icon and select "Open Config Folder" to open the configuration directory in your file explorer.
3. **Toggle Launch at Login**: Right-click the tray icon and select "Toggle Launch at Login" to enable or disable the application to start at login.
4. **Execute Command**: Left-click the tray icon and select the command you want to run from the menu. The command will be executed in the specified terminal application.

### Creating Submenus

To create submenus, set the `command` field to `null` and provide a list of subcommands in the `submenu` field. Subcommands can also have their own submenus, allowing for nested menus.

## Building the Application

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Tauri](https://tauri.app/v1/guides/getting-started/prerequisites/)

### Steps

1. Clone the repository:
   ```sh
   git clone https://github.com/yourusername/switchshuttle.git
   cd switchshuttle
   ```

2. Build the application:
   ```sh
   cargo build --release
   ```

3. Run the application:
   ```sh
   cargo run
   ```

## Download

The latest version of SwitchShuttle can be downloaded from the [GitHub Releases](https://github.com/s00d/switchshuttle/releases) page.

### macOS Sign

If you are on macOS, you may need to sign the application before running it. Here are the steps:

1. Make the binary executable:

    ```bash
    chmod +x ./switch-shuttle.app
    ```

2. Clear extended attributes and sign the binary:

    ```bash
    xattr -cr ./switch-shuttle.app && codesign --force --deep --sign - ./switch-shuttle.app
    ```


## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue on GitHub.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

Enjoy using SwitchShuttle for managing your terminal commands easily!