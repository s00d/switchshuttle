### Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.6.0] - 2025-07-04

### Added
- Enhanced CommandItem layout with improved submenu toggle functionality
- Template commands modal for better command management
- CustomSelect component for option selection
- Support for custom input class styling
- Button to create new configuration in editor
- TemplateCommandsModal component for template commands selection

### Changed
- Replaced Input components with CustomSelect for terminal and launch options
- Adjusted modal padding and height for better visibility
- Updated z-index for AppHeader component

### Fixed
- Removed unnecessary styling from CommandTypeSelector component

## [1.5.3] - 2025-06-30

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

## [1.5.2] - 2025-06-30

### Added
- Improved navigation and error handling in Inputs.vue component

## [1.5.1] - 2025-06-30

### Added
- Modern compact input form design with improved UX
- Keyboard shortcuts support (Enter to submit, Esc to cancel)
- Visual indicators for required fields

### Changed
- Updated dependencies to latest versions
- Improved input window logic for better command execution flow
- Enhanced form layout with horizontal label-input arrangement

## [1.5.0] - 2025-06-18

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

## [1.4.1] - 2025-05-18

### Added
- TailwindCSS integration for improved styling

### Changed
- Updated dependencies to latest versions

### Fixed
- Editor functionality issues

## [1.3.12] - 2024-12-21

### Added
- Opener plugin for enhanced functionality

## [1.3.11] - 2024-12-21

### Changed
- Updated project dependencies

## [1.3.10] - 2024-11-07

### Changed
- Updated dependencies to latest versions

## [1.3.9] - 2024-10-17

### Fixed
- TaoApp activation issues

## [1.3.8] - 2024-10-16

### Changed
- Migrated to Tauri v2 release

## [1.3.5] - 2024-08-07

### Added
- Tauri v2 integration
- New application icons
- UI refactoring

### Changed
- Updated user interface design

## [1.2.3] - 2024-08-02

### Added
- Developer tools in development mode

### Changed
- Updated editor user interface

## [1.2.2] - 2024-08-01

### Fixed
- Inputs component now retrieves ID from route and fetches corresponding input data correctly
- Fixed various UI issues to improve user experience

## [1.2.1] - 2024-08-01

### Changed
- Updated user interface design

## [1.1.8] - 2024-08-31

### Added
- Error display window for validation issues using Tauri

### Fixed
- Configuration file loading now ensures only JSON format files are processed

## [1.1.7] - 2024-08-31

### Fixed
- Buffer restoration functionality

## [1.1.6] - 2024-08-31

### Added
- Support for new terminal applications: Alacritty and Hyper

### Fixed
- Text insertion from clipboard for improved reliability
- Warp terminal integration issues for smoother operation

## [1.1.5] - 2024-08-30

### Changed
- Updated user interface design

## [1.1.4] - 2024-08-30

### Changed
- Updated user interface design

## [1.1.3] - 2024-07-30

### Changed
- Configuration files are now sorted by name before processing for consistent execution order
- Added unique IDs to each command and submenu item on loading configurations

## [1.1.2] - 2024-07-30

### Fixed
- Subcommand search during execution now correctly identifies and executes nested subcommands
- Input form handling - forms are now properly cleared before displaying new inputs

## [1.1.1] - 2024-07-30

### Added
- Context menu triggered by hotkey at cursor position

### Fixed
- Hotkey event handling now only triggers actions on key release for more precise behavior

## [1.1.0] - 2024-07-29

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

## [1.0.7] - 2024-07-28

### Added
- Sequential command execution - ability to run multiple commands in order

## [1.0.6] - 2024-07-27

### Fixed
- Improved compatibility with different terminals on macOS

## [1.0.5] - 2024-07-26

### Added
- Configuration editor for easier config file management

### Fixed
- Updated window identifiers for routing consistency

---

*Note: Dates in the changelog are for illustrative purposes and should be updated accordingly.*
