### Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.3.12] - 2024-12-21
### Feat
- add opener plugin

## [1.3.11] - 2024-12-21
### Chore
- update dependencies

## [1.3.10] - 2024-11-07
### Fixed
- update deps

## [1.3.9] - 2024-10-17
### Fixed
- TaoApp activate fix

## [1.3.8] - 2024-10-16
### Changed
- tauri v2 release

## [1.3.5] - 2024-08-07
### Added
- tauri v2
- icons
- refactor
- update ui

## [1.2.3] - 2024-08-02
### Added
- add devtools in dev

### Changed
- update editor ui

## [1.2.2] - 2024-08-01
### Fixed
- Inputs component now retrieves ID from route and fetches corresponding input data correctly.
- Fixed various UI issues to improve user experience.

## [1.2.1] - 2024-08-01
### Changed
- update ui

## [1.1.8] - 2024-08-31
### Added
- Added a window to display errors in case of validation issues using Tauri.
### Fixed
- Fixed a check to ensure that only JSON format configuration files are loaded.

## [1.1.7] - 2024-08-31
### Fixed
- Buffer restore.

## [1.1.6] - 2024-08-31
### Added
- Added support for new terminal applications: Alacritty and Hyper.

### Fixed
- Fixed text insertion from clipboard for improved reliability.
- Fixed issues with Warp terminal integration, ensuring smoother operation.

## [1.1.5] - 2024-08-30
### Changed
- update ui

## [1.1.4] - 2024-08-30
### Changed
- update ui

## [1.1.3] - 2024-07-30
### Changed
- Sorted configuration files by name before processing, ensuring a consistent order of execution.
- Added unique IDs to each command and submenu item on loading configurations, ensuring distinct identifiers across different configurations.

## [1.1.2] - 2024-07-30
### Fixed
- Fixed subcommand search during execution. The application now correctly identifies and executes subcommands nested within submenus.
- Fixed input form handling for commands. Input forms are now properly cleared before displaying new inputs, ensuring that old inputs do not persist across different commands.

## [1.1.1] - 2024-07-30
### Added
- Introduced context menu triggered by a hotkey. Users can now open a context menu at the cursor's position using a predefined hotkey.

### Fixed
- Fixed hotkey event handling to only trigger actions on key release, ensuring more precise and expected behavior.

## [1.1.0] - 2024-07-29
### Added
- Introduced input fields for commands. Now commands can have input fields specified in the configuration file. When a command with inputs is triggered, a form will be displayed to the user to provide the necessary values. These values are then substituted into the command before execution.

### Changed
- Updated menu rendering logic. The `menu_title` parameter has been removed, and submenus are now created using the `submenu` field directly in the configuration file.
- Enhanced window management logic. Windows are now created only when needed, rather than being pre-created.
- Improved configuration file loading logic for better performance and reliability.

### Fixed
- Fixed issues with updating the configuration when opening it for editing.

## [1.0.7] - 2024-07-28
### Added
- Added the ability to run multiple commands sequentially. This feature allows defining a series of commands that execute in order, enhancing the automation capabilities of the application.

## [1.0.6] - 2024-07-27
### Fixed
- Improved compatibility with different terminals on macOS. Adjustments were made to ensure smooth operation across various terminal applications.

## [1.0.5] - 2024-07-26
### Added
- Introduced a configuration editor for easier management of config files.

---

Dates in the changelog are for illustrative purposes and should be updated accordingly.
