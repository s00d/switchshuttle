# SwitchShuttle Development Guide

## Prerequisites

Before starting development, ensure that you have the following software installed:

- **Rust**: Install Rust using [rustup](https://rustup.rs/).
- **Node.js**: Download and install Node.js from the [official website](https://nodejs.org/).
- **Tauri CLI**: Install Tauri CLI globally by running:
```sh
cargo install tauri-cli
```

## Setting Up the Development Environment

### 1. Clone the Repository

Clone the repository to your local machine:

```sh
git clone https://github.com/s00d/SwitchShuttle.git
cd SwitchShuttle
```

### 2. Install Node.js Dependencies

Navigate to the project directory and install the required Node.js dependencies:

```sh
npm install
```

### 3. Build and Run the Application

To start the development server and run the application, use:

```sh
npm run tauri dev
```

This command will build the Tauri application in development mode and launch it.

## Project Structure

The project is organized into several modules, each serving a specific purpose:

- **src-tauri/**: Contains the main Rust source code for the application.
    - **commands.rs**: Defines the Tauri commands used by the application.
    - **config.rs**: Manages configurations and command definitions.
    - **helpers.rs**: Contains helper functions for various utilities, including command execution.
    - **hotkeys.rs**: Manages global hotkey registration and handling.
    - **menu.rs**: Handles the creation and management of the system tray menu.
    - **main.rs**: The entry point of the application.
- **scripts/**: Contains AppleScript files used for managing terminals on macOS.
- **src/**: Contains the frontend source code for the application.

## Configuration Management

### Config Structure

The application uses JSON configuration files to manage terminal commands. The default configuration file is located at `~/.config/switch-shuttle/config.json`. The configuration structure is defined in `config.rs` with `Config` and `CommandConfig` structs.

### Loading Configurations

Configurations are loaded at startup and managed within a `ConfigManager`. The configuration manager handles loading, saving, and finding commands by ID.

## Tauri Commands

Tauri commands are defined in `commands.rs` and include:

- `create_new_config`: Creates a new configuration file.
- `about_message`: Returns the application's about message.
- `check_for_updates`: Checks for updates from the GitHub repository.
- `get_version`: Returns the current version of the application.
- `execute_command_with_inputs`: Executes a command with provided inputs.
- `get_menu_data`: Retrieves data for the system tray menu.
- `execute`: Executes a specified command.
- `fetch_input_data`: Fetches input data for a command.

### Paths to Structs and Functions

- **ConfigManager**: `src/config.rs`
    - Manages configurations and commands.
- **CommandConfig**: `src/config.rs`
    - Defines individual command configurations.
- **execute_command**: `src/helpers.rs`
    - Contains logic to execute a command in the specified terminal.
- **create_system_tray_menu**: `src/menu.rs`
    - Generates the system tray menu based on the configuration.
- **handle_system_tray_event**: `src/menu.rs`
    - Handles events related to the system tray, such as menu item clicks.
- **register_global_hotkeys**: `src/hotkeys.rs`
    - Registers global hotkeys based on the configuration.
- **handle_hotkey_events**: `src/hotkeys.rs`
    - Listens for and handles global hotkey events.

## System Tray Menu

The system tray menu is created and managed using the `SystemTray` API from Tauri. Menu items are dynamically generated based on the loaded configuration. The menu structure is defined in `menu.rs`.

### Menu Items

Menu items are individual commands or actions that can be executed from the system tray menu. Each item can have an optional hotkey.

### Submenu

A submenu is a nested menu within the system tray menu. Submenus are used to organize related commands under a single parent item.

## Global Hotkeys

Global hotkeys are managed using the `global_hotkey` crate. Hotkeys can be specified in the configuration file and are registered at startup. When a hotkey is pressed, the corresponding command is executed.

### Registering Hotkeys

Hotkeys are registered in `hotkeys.rs`. The `register_global_hotkeys` function iterates through the configuration and registers each hotkey with the global hotkey manager.

### Handling Hotkey Events

Hotkey events are handled in a separate thread that listens for global hotkey events. When an event is detected, the corresponding command is executed.

## Building for Production

To build the application for production, run:

```sh
npm run tauri build
```

This will create a production-ready executable in the `src-tauri/target/release` directory.

## Contributing

Contributions to SwitchShuttle are welcome! If you encounter any issues or have suggestions for improvements, please open an issue or submit a pull request on GitHub.

---

Thank you for contributing to SwitchShuttle! If you have any questions or need further assistance, please refer to the [Tauri documentation](https://tauri.app/v1/guides/) or open an issue on GitHub.