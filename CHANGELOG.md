# Changelog

All notable changes to this project will be documented in this file. See [standard-version](https://github.com/conventional-changelog/standard-version) for commit guidelines.

## [2.0.0](https://github.com/s00d/switchshuttle/compare/v1.13.0...v2.0.0) (2025-01-27)

### ✨ Features
* **interface:** completely redesigned and modernized user interface
* **security:** enhanced security features and improvements
* **notifications:** added comprehensive notification system
* **code:** optimized codebase for better performance and maintainability

### 🔧 Chores
* **deps:** updated dependencies for improved compatibility and security

### ♻️ Code Refactoring
* **interface:** redesigned and modernized user interface
* **code:** optimized codebase structure and performance

### 🐛 Bug Fixes
* **general:** fixed various bugs and issues throughout the application

## [1.13.0](https://github.com/s00d/switchshuttle/compare/v1.12.0...v1.13.0) (2025-07-15)

### ✨ Features
* **components:** add collapsible section component for better UI organization
* **command-item:** enhance command display with advanced options and improved styling
* **commands:** add functionality to create command groups with visual distinction
* **components:** add CommandSubmenu component for managing grouped commands
* **scheduler:** add SchedulerInput component for cron expression management with visual editor
* **HotkeyInput:** add dropdown for selecting key combinations with teleport functionality
* **help:** add FAQs for scheduling commands and background execution
* **templates:** add background support for shell switch commands
* **scheduler:** add new scheduler template for automated tasks
* **types:** add scheduler and background properties to SwitchShuttle type
* **menu:** add scheduler functionality for menu items with cron support
* **config:** add scheduler and background options to CommandConfig
* **docs:** add scheduled commands and background execution sections to README

### 🔧 Chores
* **deps:** update dependencies in Cargo.toml for improved compatibility

### ♻️ Code Refactoring
* **components:** remove CommandTypeSelector.vue component and integrate functionality
* **commands:** remove unused legacy field from command structure
* **execute:** streamline command addition logic and remove redundancy
* **menu:** improve comments and enhance timer management logic
* **refactor(commands):** remove unused legacy field from command structure
* **refactor(execute):** streamline command addition logic and remove redundancy
* **refactor(menu):** improve comments and enhance timer management logic

### 🐛 Bug Fixes
* **ui:** adjust spacing in IconSelector component for better visual alignment

## [1.12.0](https://github.com/s00d/switchshuttle/compare/v1.11.0...v1.12.0) (2025-07-11)

### ✨ Features
* **hotkeys:** implement global hotkey management and execution
* **hotkeys:** implement hotkey management with conflict detection
* **menu:** add command execution by ID with input handling
* **menu_structure:** refactor tray activity tracking and command execution
* **console:** implement console connection pooling
* **execute:** refactor command execution system
* **commands:** refactor command handling and management
* **feat(IconSelector):** add icon selector component with dropdown functionality
* **feat(components):** replace SVG icons with Vue components in CommandsTable
* **feat(components):** replace SVG icons with new components in CommandItem
* **feat(icons):** add new SVG icons for Add, Lightning, Chevron, Trash, and X
* **feat(editor):** add terminal options and loading states to ConfigEditor
* **feat(tauri-commands):** add terminal configuration and retrieval methods
* **feat(config-editor):** update terminal options handling in component
* **feat(terminals):** remove obsolete terminal options and functions
* **feat(development):** update Tauri commands list with new terminal command

### 🔧 Chores
* **deps:** update tokio version and add features
* **chore(deps):** update tokio version and add features

### ♻️ Code Refactoring
* **app:** remove unused hotkey and context menu handling code
* **refactor(app):** remove unused hotkey and context menu handling code
* **refactor(terminals):** remove obsolete terminal options and functions
* **feat(scripts):** replace old iTerm scripts with updated versions
* **helpers:** add debug logging for menu item creation

### 🐛 Bug Fixes
* **hotkeys:** fix global hotkey registration and execution
* **menu:** fix command execution with proper timer management

## [1.11.0](https://github.com/s00d/switchshuttle/compare/v1.10.0...v1.11.0) (2025-07-07)

### ✨ Features
* **terminals:** add RustRover IDE support for macOS, Windows, and Linux
* **scripts:** add AppleScript automation for RustRover terminal integration
* **execute:** implement RustRover command execution across all platforms
* **terminals:** add terminal options by platform with OS detection
* **scripts:** add AppleScript automation for Visual Studio Code
* **capabilities:** add 'settings' and 'os:default' permissions
* **console:** implement asynchronous command execution with threads
* **execute:** implement execute command functionality

### 🔧 Chores
* **deps:** update Tauri OS plugin to version 2.3.0

### ♻️ Code Refactoring
* **settings:** clean up code and improve formatting
* **menu_structure:** reorganize imports and enhance logging statements
* **menu:** reorganize imports and improve code formatting
* **lib:** reorganize module imports and structure
* **config:** remove unnecessary whitespace and clean up formatting
* **commands:** improve code formatting and readability
* **cli:** replace helper function with direct command execution
* **helpers:** clean up and reorganize code structure

### 🐛 Bug Fixes
* **modal:** adjust maximum height of modal component

## [1.10.0](https://github.com/s00d/switchshuttle/compare/v1.9.0...v1.10.0) (2025-07-05)

### ✨ Features
* **icons:** add SettingsIcon component for user settings
* **router:** replace update route with settings route
* **app:** replace notification logic with SwitchShuttleCommands
* **settings:** add settings page for configuring application options
* **inputs:** improve form handling and error management
* **editor:** integrate Tauri commands for configuration management
* **about:** inject Tauri commands plugin for version retrieval
* **tauri-commands:** add Vue plugin for global access to Tauri commands
* **tauri-commands:** add initial implementation of Tauri command handlers
* **components:** add Toggle component for boolean state management
* **input:** add min and max props for input validation
* **components:** replace configuration toggle with Toggle component
* **settings:** add application settings management for notifications and auto-start
* **menu_structure:** add debug logging for switch and monitor checks
* **menu:** add settings option to the tray menu
* **settings:** add settings management to application initialization
* **console:** add timeout handling for console output reading
* **config:** set title based on filename without extension
* **commands:** add execute command with notification sound support

### 🔧 Chores
* **deps:** update Tauri plugins and dependencies in Cargo.toml
* **version:** update version to 1.10.0 and add type-check script

### ♻️ Code Refactoring
* **update:** remove outdated update page component
* **helpers:** remove redundant window navigation emit call

### 🐛 Bug Fixes
* **console:** fix timeout handling for blocking read operations
* **menu_structure:** fix monitor command execution logic
* **switch:** fix switch command execution using proper command field

## [1.9.0](https://github.com/s00d/switchshuttle/compare/v1.8.0...v1.9.0) (2025-07-05)

### ✨ Features
* **menu:** implement comprehensive system menu structure with monitoring support
* **monitoring:** add real-time monitoring capabilities with background timer management
* **tray:** add mouse event handlers for intelligent timer management (pause/resume on hover)
* **console:** implement persistent console instance for improved command execution
* **menu:** enhance menu item creation with individual timer management
* **monitoring:** add new monitoring templates and enhance existing ones

### 🔧 Chores
* **deps:** add lazy_static dependency for global state management

### ♻️ Code Refactoring
* **menu:** simplify menu creation and update logic with structured approach
* **commands:** rename update_system_tray_menu to update_tray_menu_from_commands for clarity
* **helpers:** remove redundant execute_command_silent function
* **commands:** replace helpers with console for command execution
* **console:** add ConsoleInstance for managing interactive console sessions
* **menu:** enhance menu item creation with console command execution

### 🐛 Bug Fixes
* **menu:** fix background timer cleanup when menu items are removed
* **monitoring:** implement proper timer lifecycle management with stop flags
* **menu:** resolve timer conflicts during menu updates

### ⚡ Performance
* **tray:** optimize monitoring timers with pause/resume functionality on mouse events
* **menu:** implement efficient timer management to prevent resource leaks

## [1.8.0](https://github.com/s00d/switchshuttle/compare/v1.7.0...v1.8.0) (2025-07-05)

### ✨ Features
* **components:** add icon input and dynamic inputs section to CommandItem ([8fe1967](https://github.com/s00d/switchshuttle/commit/8fe1967))
* **commands:** add support for 'monitor' command type ([8fe1967](https://github.com/s00d/switchshuttle/commit/8fe1967))
* **components:** add monitor button to command type selector ([8fe1967](https://github.com/s00d/switchshuttle/commit/8fe1967))
* **TemplateCommandsModal:** enhance command display with icon and monitor info ([8fe1967](https://github.com/s00d/switchshuttle/commit/8fe1967))
* **help:** add monitoring commands section with examples and details ([8fe1967](https://github.com/s00d/switchshuttle/commit/8fe1967))
* **editor:** replace Template to json ([8fe1967](https://github.com/s00d/switchshuttle/commit/8fe1967))
* **editor:** add loading delay before saving configuration ([8fe1967](https://github.com/s00d/switchshuttle/commit/8fe1967))
* **types:** add description and additional fields to Command and Template ([8fe1967](https://github.com/s00d/switchshuttle/commit/8fe1967))
* **types:** add TypeScript definitions for JSON modules ([8fe1967](https://github.com/s00d/switchshuttle/commit/8fe1967))
* **vite:** add JSON import support with stringification option ([8fe1967](https://github.com/s00d/switchshuttle/commit/8fe1967))
* **menu:** add icon support to menu items and improve display names ([8fe1967](https://github.com/s00d/switchshuttle/commit/8fe1967))
* **helpers:** add icon support for menu items and notification error handling ([8fe1967](https://github.com/s00d/switchshuttle/commit/8fe1967))
* **config:** add monitor and icon fields to Config struct ([8fe1967](https://github.com/s00d/switchshuttle/commit/8fe1967))
* **commands:** add monitor and icon properties to command structure ([8fe1967](https://github.com/s00d/switchshuttle/commit/8fe1967))
* **tauri.conf:** enhance window configurations for multiple views ([8fe1967](https://github.com/s00d/switchshuttle/commit/8fe1967))
* **menu:** simplify submenu creation and improve error handling ([8fe1967](https://github.com/s00d/switchshuttle/commit/8fe1967))
* **helpers:** enhance window management and menu item creation ([8fe1967](https://github.com/s00d/switchshuttle/commit/8fe1967))
* **commands:** implement save_or_update_configuration function ([8fe1967](https://github.com/s00d/switchshuttle/commit/8fe1967))
* **menu:** add support for toggleable switch commands in system tray ([8fe1967](https://github.com/s00d/switchshuttle/commit/8fe1967))
* **helpers:** add command execution and menu item creation functions ([8fe1967](https://github.com/s00d/switchshuttle/commit/8fe1967))
* **config:** add enabled flag and switch command functionality ([8fe1967](https://github.com/s00d/switchshuttle/commit/8fe1967))
* **commands:** add support for switch commands with notifications ([8fe1967](https://github.com/s00d/switchshuttle/commit/8fe1967))
* **routes:** add help page route and component ([8fe1967](https://github.com/s00d/switchshuttle/commit/8fe1967))
* **ui:** add new button styles and color variables ([8fe1967](https://github.com/s00d/switchshuttle/commit/8fe1967))
* **help:** add help and documentation page with comprehensive guide ([8fe1967](https://github.com/s00d/switchshuttle/commit/8fe1967))
* **editor:** enhance configuration card styling based on enabled state ([8fe1967](https://github.com/s00d/switchshuttle/commit/8fe1967))
* **templates:** add macOS, Windows, and Linux switch categories ([8fe1967](https://github.com/s00d/switchshuttle/commit/8fe1967))
* **TemplateCommandsModal:** add tag filter to command templates ([8fe1967](https://github.com/s00d/switchshuttle/commit/8fe1967))
* **configEditor:** add toggle for enabling/disabling configuration ([8fe1967](https://github.com/s00d/switchshuttle/commit/8fe1967))
* **components:** add 'Switch' option to CommandTypeSelector ([8fe1967](https://github.com/s00d/switchshuttle/commit/8fe1967))
* **commandsTable:** add handling for 'switch' command type ([8fe1967](https://github.com/s00d/switchshuttle/commit/8fe1967))
* **CommandItem:** refactor input classes for improved styling consistency ([8fe1967](https://github.com/s00d/switchshuttle/commit/8fe1967))
* **button:** simplify button styles and introduce utility classes ([8fe1967](https://github.com/s00d/switchshuttle/commit/8fe1967))
* **docs:** add configuration management and switch commands sections ([8fe1967](https://github.com/s00d/switchshuttle/commit/8fe1967))

### 🔧 Chores
* **versioning:** replace .versionrc.js with .versionrc.json ([8fe1967](https://github.com/s00d/switchshuttle/commit/8fe1967))
* **config:** add versioning configuration and release scripts ([8fe1967](https://github.com/s00d/switchshuttle/commit/8fe1967))

### ♻️ Code Refactoring
* **app:** remove AppHeader component and adjust layout ([8fe1967](https://github.com/s00d/switchshuttle/commit/8fe1967))
* **commands:** rename save_configuration to save_or_update_configuration ([8fe1967](https://github.com/s00d/switchshuttle/commit/8fe1967))
* **components:** simplify Card.vue class management and styling ([8fe1967](https://github.com/s00d/switchshuttle/commit/8fe1967))

### 🐛 Bug Fixes
* **docs:** correct regex for keyword matching in documentation files ([8fe1967](https://github.com/s00d/switchshuttle/commit/8fe1967))
* **cli:** skip disabled configurations when searching for commands ([8fe1967](https://github.com/s00d/switchshuttle/commit/8fe1967))
* **orders:** resolve issue with incorrect order total calculation ([8fe1967](https://github.com/s00d/switchshuttle/commit/8fe1967))

## [1.7.0](https://github.com/s00d/switchshuttle/compare/v1.6.0...v1.7.0) (2025-07-04)

### ✨ Features
* **types:** add switch property to Command type and enabled to Config ([8fe1967](https://github.com/s00d/switchshuttle/commit/8fe1967))
* **commands:** add support for switch commands with notifications ([8fe1967](https://github.com/s00d/switchshuttle/commit/8fe1967))
* **menu:** add support for toggleable switch commands in system tray ([8fe1967](https://github.com/s00d/switchshuttle/commit/8fe1967))
* **config:** add enabled flag and switch command functionality ([8fe1967](https://github.com/s00d/switchshuttle/commit/8fe1967))
* **helpers:** add command execution and menu item creation functions ([8fe1967](https://github.com/s00d/switchshuttle/commit/8fe1967))
* **routes:** add help page route and component ([8fe1967](https://github.com/s00d/switchshuttle/commit/8fe1967))
* **ui:** add new button styles and color variables ([8fe1967](https://github.com/s00d/switchshuttle/commit/8fe1967))
* **help:** add help and documentation page with comprehensive guide ([8fe1967](https://github.com/s00d/switchshuttle/commit/8fe1967))
* **editor:** enhance configuration card styling based on enabled state ([8fe1967](https://github.com/s00d/switchshuttle/commit/8fe1967))
* **templates:** add macOS, Windows, and Linux switch categories ([8fe1967](https://github.com/s00d/switchshuttle/commit/8fe1967))
* **TemplateCommandsModal:** add tag filter to command templates ([8fe1967](https://github.com/s00d/switchshuttle/commit/8fe1967))
* **configEditor:** add toggle for enabling/disabling configuration ([8fe1967](https://github.com/s00d/switchshuttle/commit/8fe1967))
* **components:** add 'Switch' option to CommandTypeSelector ([8fe1967](https://github.com/s00d/switchshuttle/commit/8fe1967))
* **commandsTable:** add handling for 'switch' command type ([8fe1967](https://github.com/s00d/switchshuttle/commit/8fe1967))
* **CommandItem:** refactor input classes for improved styling consistency ([8fe1967](https://github.com/s00d/switchshuttle/commit/8fe1967))
* **button:** simplify button styles and introduce utility classes ([8fe1967](https://github.com/s00d/switchshuttle/commit/8fe1967))
* **docs:** add configuration management and switch commands sections ([8fe1967](https://github.com/s00d/switchshuttle/commit/8fe1967))

### 🐛 Bug Fixes
* **docs:** correct regex for keyword matching in documentation files ([8fe1967](https://github.com/s00d/switchshuttle/commit/8fe1967))
* **cli:** skip disabled configurations when searching for commands ([8fe1967](https://github.com/s00d/switchshuttle/commit/8fe1967))
* **orders:** resolve issue with incorrect order total calculation ([8fe1967](https://github.com/s00d/switchshuttle/commit/8fe1967))

### ♻️ Code Refactoring
* **components:** simplify Card.vue class management and styling ([8fe1967](https://github.com/s00d/switchshuttle/commit/8fe1967))

## [1.6.0](https://github.com/s00d/switchshuttle/compare/v1.5.0...v1.6.0) (2025-07-03)

### ✨ Features
* Initial release with basic command execution functionality
* Cross-platform system tray application
* Support for macOS, Windows, and Linux
* Command template system
* Configuration management
* Terminal switching capabilities

### 🔧 Chores
* Initial project setup
* Basic documentation
* Development environment configuration

## [1.5.3](https://github.com/s00d/switchshuttle/compare/v1.5.2...v1.5.3) (2025-06-30)

### Added
- CLI module refactoring for better code organization
- Improved CLI command handling with separate module
- Auto-focus on first input field after loading
- Command line interface (CLI) for executing commands by ID or name
- CLI commands: list all commands and search by name

### Changed
- Updated dependencies to latest versions
- Improved input window logic for better command execution flow
- Enhanced form layout with horizontal label-input arrangement

## [1.5.2](https://github.com/s00d/switchshuttle/compare/v1.5.1...v1.5.2) (2025-06-30)

### Added
- Improved navigation and error handling in Inputs.vue component

## [1.5.1](https://github.com/s00d/switchshuttle/compare/v1.5.0...v1.5.1) (2025-06-30)

### Added
- Modern compact input form design with improved UX
- Keyboard shortcuts support (Enter to submit, Esc to cancel)
- Visual indicators for required fields

### Changed
- Updated dependencies to latest versions
- Improved input window logic for better command execution flow
- Enhanced form layout with horizontal label-input arrangement

## [1.5.0](https://github.com/s00d/switchshuttle/compare/v1.4.1...v1.5.0) (2025-06-18)

### Added
- New modern logo design for better brand recognition
- Loading spinner component for async operations
- CommandTypeSelector component for better command type management
- CommandItem component for improved command rendering
- "Open Config Folder" functionality in editor
- Enhanced gradient background in AppHeader

### Changed
- Updated dependencies in Cargo.toml and package.json
- Simplified command rendering with CommandItem component
- Enhanced UI with no-drag class for better window management
- Updated window settings in tauri.conf for improved UI experience
- Refactored components to remove redundant elements

### Fixed
- Cleaned up unused imports and props definitions
- Removed redundant commands header in ConfigEditor
- Improved component structure and organization

### Removed
- Outdated improvement tasks document (todos)

## [1.4.1](https://github.com/s00d/switchshuttle/compare/v1.3.12...v1.4.1) (2025-05-18)

### Added
- TailwindCSS integration for improved styling

### Changed
- Updated dependencies to latest versions

### Fixed
- Editor functionality issues

## [1.3.12](https://github.com/s00d/switchshuttle/compare/v1.3.11...v1.3.12) (2024-12-21)

### Added
- Opener plugin for enhanced functionality

## [1.3.11](https://github.com/s00d/switchshuttle/compare/v1.3.10...v1.3.11) (2024-12-21)

### Changed
- Updated project dependencies

## [1.3.10](https://github.com/s00d/switchshuttle/compare/v1.3.9...v1.3.10) (2024-11-07)

### Changed
- Updated dependencies to latest versions

## [1.3.9](https://github.com/s00d/switchshuttle/compare/v1.3.8...v1.3.9) (2024-10-17)

### Fixed
- TaoApp activation issues

## [1.3.8](https://github.com/s00d/switchshuttle/compare/v1.3.5...v1.3.8) (2024-10-16)

### Changed
- Migrated to Tauri v2 release

## [1.3.5](https://github.com/s00d/switchshuttle/compare/v1.2.3...v1.3.5) (2024-08-07)

### Added
- Tauri v2 integration
- New application icons
- UI refactoring

### Changed
- Updated user interface design

## [1.2.3](https://github.com/s00d/switchshuttle/compare/v1.2.2...v1.2.3) (2024-08-02)

### Added
- Developer tools in development mode

### Changed
- Updated editor user interface

## [1.2.2](https://github.com/s00d/switchshuttle/compare/v1.2.1...v1.2.2) (2024-08-01)

### Fixed
- Inputs component now retrieves ID from route and fetches corresponding input data correctly
- Fixed various UI issues to improve user experience

## [1.2.1](https://github.com/s00d/switchshuttle/compare/v1.1.8...v1.2.1) (2024-08-01)

### Changed
- Updated user interface design

## [1.1.8](https://github.com/s00d/switchshuttle/compare/v1.1.7...v1.1.8) (2024-08-31)

### Added
- Error display window for validation issues using Tauri

### Fixed
- Configuration file loading now ensures only JSON format files are processed

## [1.1.7](https://github.com/s00d/switchshuttle/compare/v1.1.6...v1.1.7) (2024-08-31)

### Fixed
- Buffer restoration functionality

## [1.1.6](https://github.com/s00d/switchshuttle/compare/v1.1.5...v1.1.6) (2024-08-31)

### Added
- Support for new terminal applications: Alacritty and Hyper

### Fixed
- Text insertion from clipboard for improved reliability
- Warp terminal integration issues for smoother operation

## [1.1.5](https://github.com/s00d/switchshuttle/compare/v1.1.4...v1.1.5) (2024-08-30)

### Changed
- Updated user interface design

## [1.1.4](https://github.com/s00d/switchshuttle/compare/v1.1.3...v1.1.4) (2024-08-30)

### Changed
- Updated user interface design

## [1.1.3](https://github.com/s00d/switchshuttle/compare/v1.1.2...v1.1.3) (2024-07-30)

### Changed
- Configuration files are now sorted by name before processing for consistent execution order
- Added unique IDs to each command and submenu item on loading configurations

## [1.1.2](https://github.com/s00d/switchshuttle/compare/v1.1.1...v1.1.2) (2024-07-30)

### Fixed
- Subcommand search during execution now correctly identifies and executes nested subcommands
- Input form handling - forms are now properly cleared before displaying new inputs

## [1.1.1](https://github.com/s00d/switchshuttle/compare/v1.1.0...v1.1.1) (2024-07-30)

### Added
- Context menu triggered by hotkey at cursor position

### Fixed
- Hotkey event handling now only triggers actions on key release for more precise behavior

## [1.1.0](https://github.com/s00d/switchshuttle/compare/v1.0.7...v1.1.0) (2024-07-29)

### Added
- Input fields for commands with form display for user input
- Values are substituted into commands before execution

### Changed
- Updated menu rendering logic - removed `menu_title` parameter
- Submenus now use `submenu` field directly in configuration
- Enhanced window management - windows created only when needed
- Improved configuration file loading logic

### Fixed
- Configuration update issues when opening for editing

## [1.0.7](https://github.com/s00d/switchshuttle/compare/v1.0.6...v1.0.7) (2024-07-28)

### Added
- Sequential command execution - ability to run multiple commands in order

## [1.0.6](https://github.com/s00d/switchshuttle/compare/v1.0.5...v1.0.6) (2024-07-27)

### Fixed
- Improved compatibility with different terminals on macOS

## [1.0.5](https://github.com/s00d/switchshuttle/compare/v1.0.4...v1.0.5) (2024-07-26)

### Added
- Configuration editor for easier config file management

### Fixed
- Updated window identifiers for routing consistency

---

*Note: Dates in the changelog are for illustrative purposes and should be updated accordingly.*
