### Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.1.2] - 2024-07-31
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