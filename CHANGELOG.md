# Changelog

All notable changes to this project will be documented in this file. See [standard-version](https://github.com/conventional-changelog/standard-version) for commit guidelines.

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
