# Changelog

All notable changes to this project will be documented in this file. See [standard-version](https://github.com/conventional-changelog/standard-version) for commit guidelines.

## [2.1.0](https://github.com/s00d/switchshuttle/compare/v1.7.0...v2.1.0) (2025-09-29)


### 📚 Documentation

* **ADDING_WINDOWS:** add guide for creating new windows in demo app ([32294e6](https://github.com/s00d/switchshuttle/commit/32294e6b97f03a96fdb12b28d189d58550256eac))
* **development:** update Tauri commands list with new terminal command ([d7803f9](https://github.com/s00d/switchshuttle/commit/d7803f90e1650524a775b8f6d32fd7a528a604d4))
* **nuxt.config:** add changelog API endpoint to readme routes ([1d2f091](https://github.com/s00d/switchshuttle/commit/1d2f091ddadd2c076b3bfc3b0c9b841cf175fe9b))
* **readme:** add scheduled commands and background execution sections ([1337bee](https://github.com/s00d/switchshuttle/commit/1337bee8113f2ecc30639c064747f81b3281ae99))
* update changelog with proper structure ([0a5f0dc](https://github.com/s00d/switchshuttle/commit/0a5f0dcdc94c590c52d8e81ac39f0db403421736))


### 🔧 Chores

* **changelog:** update CHANGELOG for version 1.12.0 release ([0351067](https://github.com/s00d/switchshuttle/commit/03510674e75b36ca9c85f1b2016e5ffa7a33a9a7))
* **changelog:** update CHANGELOG for version 1.13.0 release ([d6e8605](https://github.com/s00d/switchshuttle/commit/d6e8605643e54dd25ee9321ccbcd3a2840d4c00d))
* **changelog:** update changelog for version 1.9.0 release ([a8185c2](https://github.com/s00d/switchshuttle/commit/a8185c2df9e989ccae8509d8526db950da0231da))
* **changelog:** update to version 1.11.0 with new features and fixes ([08da02b](https://github.com/s00d/switchshuttle/commit/08da02bfa8a2bd2d519e136840db167929ca9e4c))
* **deps:** add `lazy_static` dependency to `Cargo.toml` ([c1d9192](https://github.com/s00d/switchshuttle/commit/c1d9192816580d427940661217845a3eb427eeca))
* **deps:** remove unused dependencies from `Cargo.toml` and `Cargo.lock` ([017fd90](https://github.com/s00d/switchshuttle/commit/017fd90b0c558a2101cd8177675867b0ba6f1ed6))
* **deps:** update `tokio` version and add features ([fbe71d9](https://github.com/s00d/switchshuttle/commit/fbe71d9436705cbff51aab1617967831442b622d))
* **deps:** update dependencies in `Cargo.toml` ([d62fbc8](https://github.com/s00d/switchshuttle/commit/d62fbc824e603d98a2f2f9c096e2894507cdb343))
* **deps:** update Tauri OS plugin to version 2.3.0 ([216c7f7](https://github.com/s00d/switchshuttle/commit/216c7f70139e5846232749e70abfc9339a689461))
* **deps:** update Tauri plugins and dependencies in `Cargo.toml` ([c366ce5](https://github.com/s00d/switchshuttle/commit/c366ce5d420269dd664249d7c0007ce21b533332))
* **package:** bump version from 1.10.0 to 1.11.0 ([3a679df](https://github.com/s00d/switchshuttle/commit/3a679df974f0cc469dfc09bf7a1e9cba16c42a4c))
* **package:** bump version to 1.12.0 ([91d89e8](https://github.com/s00d/switchshuttle/commit/91d89e81510a02e3464355e3cc4278aca1af76e9))
* **package:** bump version to 1.9.0 ([d767f1d](https://github.com/s00d/switchshuttle/commit/d767f1de320e0aa26bbcb60945ac3e34eb887741))
* **release:** update version to 1.13.0 and add changelog component ([8d8001f](https://github.com/s00d/switchshuttle/commit/8d8001f7c3baa0c05f135338b5eabff55c723969))


### ♻️ Code Refactoring

* **about:** remove commented-out header section from About.vue ([f1e1065](https://github.com/s00d/switchshuttle/commit/f1e1065a5bae7116ae77f58d3ce61ea5ae20070b))
* **api:** remove outdated README and changelog handlers ([1786e95](https://github.com/s00d/switchshuttle/commit/1786e953ea377e649fc526122f2b036ac31a05e3))
* **app:** remove unused hotkey and context menu handling code ([81d3894](https://github.com/s00d/switchshuttle/commit/81d3894c66b44fbc17faa9751394eb53db8044c8))
* **changelog:** rename `changelog.md` to `CHANGELOG.md` and update format ([1575c55](https://github.com/s00d/switchshuttle/commit/1575c5529612c90a8802b593ebf2ca594d76f9a3))
* **cli:** replace helper function with direct command execution ([c4dc2d9](https://github.com/s00d/switchshuttle/commit/c4dc2d9c566eeb074502fdb8aee170df404a58e7))
* **commands:** improve code formatting and readability ([e98cace](https://github.com/s00d/switchshuttle/commit/e98cace4f609bf6569c39dcd2f6616269083ca86))
* **commands:** improve code style and formatting in `commands.rs` ([ab34598](https://github.com/s00d/switchshuttle/commit/ab34598a3c2a0efeba95f36290a666cf3585f9de))
* **commands:** remove notification sound playback from command execution ([5a52474](https://github.com/s00d/switchshuttle/commit/5a5247423e19579e3e92a027b24ce5fe0f9cbc77))
* **commands:** remove unused legacy field from command structure ([6b2de92](https://github.com/s00d/switchshuttle/commit/6b2de92f2908d6fcdba7ef7413e28b3a8d3d72a8))
* **commands:** rename update_system_tray_menu to update_tray_menu_from_commands ([858b4d9](https://github.com/s00d/switchshuttle/commit/858b4d9c423cbe22f8ff135f95e7607d852819fc))
* **commands:** replace `helpers` with `console` for command execution ([3348913](https://github.com/s00d/switchshuttle/commit/33489138fa94b823782ec1b2290ab5f760583640))
* **components:** remove `CommandTypeSelector.vue` component ([21ec574](https://github.com/s00d/switchshuttle/commit/21ec574da7abee8640986fa508e94ab709575f74))
* **components:** simplify event handlers and clean unused code ([b3a1895](https://github.com/s00d/switchshuttle/commit/b3a1895ad5ddd5d2fb6c0a1f5afcfb1c83d71e90))
* **config:** remove unnecessary whitespace and clean up formatting ([cdd7ecd](https://github.com/s00d/switchshuttle/commit/cdd7ecdc7d6afede84902164f08e9943c5a42c0a))
* **console:** clean up formatting in `console.rs` ([d419fd7](https://github.com/s00d/switchshuttle/commit/d419fd78cc9f4625cbd2113f9a8130f9e32c0be0))
* **execute:** clean up formatting and improve readability ([af27622](https://github.com/s00d/switchshuttle/commit/af276221f6fe82820af123c4dbdf4152b29717b0))
* **execute:** streamline command addition logic and remove redundancy ([d8967bd](https://github.com/s00d/switchshuttle/commit/d8967bd61ee611b282d33f532df0c5aabeea98a1))
* **helpers:** clean up and reorganize code structure ([636c3f0](https://github.com/s00d/switchshuttle/commit/636c3f0a7f3b76f4bd0e13e5a8b8b9d0660ab0ca))
* **helpers:** format println! statements for better readability ([a71d6b4](https://github.com/s00d/switchshuttle/commit/a71d6b48b6ed41d188ea72f2382b69d1e9d91dea))
* **helpers:** remove redundant `execute_command_silent` function ([05ebd22](https://github.com/s00d/switchshuttle/commit/05ebd226865cfc4ddefd6e832dd5c8a4649a4420))
* **helpers:** remove redundant window navigation emit call ([f6c569b](https://github.com/s00d/switchshuttle/commit/f6c569bfe563eb785406f568d2f03bf8eff286fe))
* **hotkeys:** reorder imports and clean up code formatting ([e7ebf65](https://github.com/s00d/switchshuttle/commit/e7ebf65cb8fc754fbbdb75c1ee1e13174826041a))
* **lib:** clean up import statements and format global shortcut handler ([c0bf32e](https://github.com/s00d/switchshuttle/commit/c0bf32edd5fd1a4b2c4a596575ef56fde975cf71))
* **lib:** remove unused `play_notification_sound` function ([252afee](https://github.com/s00d/switchshuttle/commit/252afeeec929e43ea27017dfa5ff31cea85d3e25))
* **lib:** reorganize module imports and structure ([fae4e7d](https://github.com/s00d/switchshuttle/commit/fae4e7db75e812a67ba8517367a842958923aa53))
* **menu_structure:** clean up formatting and improve readability ([6c90bd3](https://github.com/s00d/switchshuttle/commit/6c90bd3ef91eb5899e77f4f59db9dfc33675c39a))
* **menu_structure:** reorganize imports and enhance logging statements ([a8023f9](https://github.com/s00d/switchshuttle/commit/a8023f9c6d7de22b90e651bba3cb4c63ac83af2f))
* **menu_structure:** simplify MenuItem by inheriting from CommandConfig ([5f21256](https://github.com/s00d/switchshuttle/commit/5f212560f18bf0a7d0fc51f9a462e7e75457cb2b))
* **menu:** clean up whitespace and formatting in `menu.rs` ([6880dd5](https://github.com/s00d/switchshuttle/commit/6880dd5425086fcfb0ed11fa79b067dd0e3d1173))
* **menu:** improve comments and enhance timer management logic ([6d23eaa](https://github.com/s00d/switchshuttle/commit/6d23eaad1f232b56c7e1a43de167c3d17869791f))
* **menu:** remove unused `openChangelog` function ([797bf7d](https://github.com/s00d/switchshuttle/commit/797bf7d370c96e6f69007f6627dd98c5d1a23eb7))
* **menu:** reorganize imports and improve code formatting ([c19bf0e](https://github.com/s00d/switchshuttle/commit/c19bf0ee08e6546df043881b78e9e94ce177748e))
* **menu:** simplify menu creation and update logic ([72e5c9e](https://github.com/s00d/switchshuttle/commit/72e5c9e7fdffe327afbafda607ee295b5969e4b1))
* **performance:** remove unused computed property for button class ([7ef6019](https://github.com/s00d/switchshuttle/commit/7ef6019d5e747580a201be6121a8f84ac777c640))
* **settings:** clean up code and improve formatting ([cc565ba](https://github.com/s00d/switchshuttle/commit/cc565baa11dd6a551ee3f68e4ebe5fcf0497a82b))
* **settings:** remove sound notifications configuration ([ff6a017](https://github.com/s00d/switchshuttle/commit/ff6a01752edb3591c24b285e144fddf9e1b253a0))
* **settings:** remove unused import for `ShieldIcon` ([965c220](https://github.com/s00d/switchshuttle/commit/965c220ad3a686fd6cf44dbb7be7ea69a3b7c5d8))
* **settings:** remove unused notification sound functionality ([c40b59f](https://github.com/s00d/switchshuttle/commit/c40b59fd33b6bf7627171639e463fa68ca9160eb))
* **tauri-commands:** remove unused play_notification_sound method ([cd144d4](https://github.com/s00d/switchshuttle/commit/cd144d46810352e4413ff0870e9654da4f99918d))
* **terminals:** remove obsolete terminal options and functions ([e5b474e](https://github.com/s00d/switchshuttle/commit/e5b474e18c8df124c72b8ffc035734d2e6518ce3))
* **update:** remove outdated update page component ([cee4f14](https://github.com/s00d/switchshuttle/commit/cee4f14a2442b3a3471c6470b518695257213692))


### 🐛 Bug Fixes

* **components:** correct title translation in TaskBar.vue ([5d0302e](https://github.com/s00d/switchshuttle/commit/5d0302eceacb2a361c5cc779b55456954fcb9885))
* **console:** handle output reading differently for Windows and non-Windows ([199f958](https://github.com/s00d/switchshuttle/commit/199f958bdb1726c11fe80e06298fb852ef36af01))
* **deps:** update dependency versions in package.json and Cargo.toml ([e5c6520](https://github.com/s00d/switchshuttle/commit/e5c652042a6a72b3d20dfee56197a5eab02131bf))
* **docs:** correct API endpoint in `ADDING_WINDOWS.md` ([3aa7409](https://github.com/s00d/switchshuttle/commit/3aa7409a956ed0b0735ec020ee23cae8d1559776))
* **docs:** fix formatting in `menu.ts` action type declaration ([40c9680](https://github.com/s00d/switchshuttle/commit/40c968014be1c347f08442fd8d44b2790d176096))
* **docs:** update API endpoints to use JSON files ([bde566b](https://github.com/s00d/switchshuttle/commit/bde566b0da6af3057f5f7d106e7d603b1d386ef8))
* **docs:** update API routes to use JSON endpoints ([96741ed](https://github.com/s00d/switchshuttle/commit/96741ed80170651644f483b5afe4dd406cb32a19))
* **helpers:** skip adding empty commands to execution list ([e0a4943](https://github.com/s00d/switchshuttle/commit/e0a4943dea078d46ef3a29427002988f33d31b22))
* **IconSelector:** remove unnecessary variable assignment in props ([f4439b2](https://github.com/s00d/switchshuttle/commit/f4439b2cda061223a2168b74466ff87cea8ba8b2))
* **modal:** adjust maximum height of modal component ([791649f](https://github.com/s00d/switchshuttle/commit/791649f66615674711827dc06775881bc4277b1c))
* **package:** update version to 1.9.2 ([c64643f](https://github.com/s00d/switchshuttle/commit/c64643fe72989010a4cc07cff2b194e767b153ab))
* **tray:** remove unused import in tray icon event handler ([b39d27d](https://github.com/s00d/switchshuttle/commit/b39d27df85ea35122c6ee1b2b45c764d0db5ef9e))
* **ui:** adjust spacing in IconSelector component ([ae8f8cc](https://github.com/s00d/switchshuttle/commit/ae8f8ccd51cb20b17748aa61b32abc9661758a53))


### ✨ Features

* **about:** inject Tauri commands plugin for version retrieval ([169a655](https://github.com/s00d/switchshuttle/commit/169a655b6939927e441244b3414151553e2d03c2))
* **api:** set Content-Type header to application/json in responses ([2ae3097](https://github.com/s00d/switchshuttle/commit/2ae3097463d311834228155add3e7b676105024a))
* **app:** replace notification logic with SwitchShuttleCommands ([f49983b](https://github.com/s00d/switchshuttle/commit/f49983bf1134c4d90b3cdd93a7116055741375ad))
* **capabilities:** add 'settings' and 'os:default' permissions ([8bf7985](https://github.com/s00d/switchshuttle/commit/8bf79850d48b6a90ef481124719fdd8ed17a7e35))
* **capabilities:** add new permissions for Windows in migrated.json ([4bdcfb4](https://github.com/s00d/switchshuttle/commit/4bdcfb47c3e1efa4e8ad309e54abebbdea729108))
* **command-item:** enhance command display and add advanced options ([e1a268c](https://github.com/s00d/switchshuttle/commit/e1a268c0cecf200cad513e63948ab4302a62aae5))
* **commands:** add execute command with notification sound support ([5e27047](https://github.com/s00d/switchshuttle/commit/5e270470877e217b7bdd35fff48059b1a1199989))
* **commands:** add functionality to create command groups ([1bf352f](https://github.com/s00d/switchshuttle/commit/1bf352f7b1d41bf5672ed8060fdccca5513cccf9))
* **commands:** refactor ([3a6c7db](https://github.com/s00d/switchshuttle/commit/3a6c7db62e8efa59051920210a3650e119b62904))
* **components:** add changelog icon with window functionality ([855e2a5](https://github.com/s00d/switchshuttle/commit/855e2a595158be1b8b4a9569b15f3e3c0ae19694))
* **components:** add collapsible section component ([2ee033f](https://github.com/s00d/switchshuttle/commit/2ee033f95d4d76039a4e542f9bceffab7944e55f))
* **components:** add CommandSubmenu component for managing commands ([2e3340f](https://github.com/s00d/switchshuttle/commit/2e3340f6f9c8eafb060a73dab4c20d7a716bdb77))
* **components:** add Toggle component for boolean state management ([fc76d31](https://github.com/s00d/switchshuttle/commit/fc76d31a00c971210108ec624059dc82a9dd88e8))
* **components:** improve layout of CommandSubmenu component ([1b8565c](https://github.com/s00d/switchshuttle/commit/1b8565cdf813edc1e92a7ecf5b37fa33c94f43fa))
* **components:** replace configuration toggle with `Toggle` component ([d0a75ea](https://github.com/s00d/switchshuttle/commit/d0a75ea4bb75f0826667771f16a3df2c065e0426))
* **components:** replace SVG icons with new components in CommandItem ([94aec5e](https://github.com/s00d/switchshuttle/commit/94aec5e1ef61f238dbbdbc3d935b4c4a2812f979))
* **components:** replace SVG icons with Vue components in CommandsTable ([9e108a4](https://github.com/s00d/switchshuttle/commit/9e108a4293312f547c19e3f96243590e7fe92772))
* **config-editor:** update terminal options handling in component ([00fb6da](https://github.com/s00d/switchshuttle/commit/00fb6da9601aea8bb76197f78cd16a1988b1fefa))
* **config:** add scheduler and background options to CommandConfig ([72476d5](https://github.com/s00d/switchshuttle/commit/72476d523610a463f796ee1c6f04da75cae37f78))
* **config:** set title based on filename without extension ([69e1a6b](https://github.com/s00d/switchshuttle/commit/69e1a6bbfa9c4725cbd5f37a6502e2df37d3a919))
* **config:** update window configuration to use dynamic component names ([d720647](https://github.com/s00d/switchshuttle/commit/d7206477e72de99f5682d132131a06c1cf1dbeaf))
* **console:** add ConsoleInstance for managing interactive console sessions ([dc4c731](https://github.com/s00d/switchshuttle/commit/dc4c731ae5356deb939665f5808350982754321a))
* **console:** add empty command check in execute_command methods ([bb6ecdb](https://github.com/s00d/switchshuttle/commit/bb6ecdb124734de571fa9a79743091dff0d8c05c))
* **console:** add timeout handling for console output reading ([0d77ce9](https://github.com/s00d/switchshuttle/commit/0d77ce9c8b03211c54a2c0528a906f0b5fa865a1))
* **console:** implement asynchronous command execution with threads ([7366406](https://github.com/s00d/switchshuttle/commit/7366406113da72564fcdc01335bb2c328ef54c28))
* **console:** implement console connection pooling ([f8841da](https://github.com/s00d/switchshuttle/commit/f8841dae56af275263ded17b32bc17b01b292836))
* **console:** initialize console instance during application startup ([0143f77](https://github.com/s00d/switchshuttle/commit/0143f77b20b227334194bf656145187446798c34))
* **docs:** add changelog API endpoint ([676b8bf](https://github.com/s00d/switchshuttle/commit/676b8bf103e31cbdda973b9f072bc37907047a19))
* **docs:** add changelog window configuration ([74e8a27](https://github.com/s00d/switchshuttle/commit/74e8a2799e7715db00127d01e544a350fba35bac))
* **docs:** add monitor commands for real-time system resource tracking ([039e8fd](https://github.com/s00d/switchshuttle/commit/039e8fd7199de38a39ed45383b1d4e85ddb1444b))
* **editor:** add terminal options and loading states to ConfigEditor ([f7738cd](https://github.com/s00d/switchshuttle/commit/f7738cd0512916e5a796b4247c15e198ad138da2))
* **editor:** integrate Tauri commands for configuration management ([d11c8e6](https://github.com/s00d/switchshuttle/commit/d11c8e6a70d2883b833ea1028fcb8b77040140c4))
* **execute:** implement command placeholder processing with quotes ([5ceee4b](https://github.com/s00d/switchshuttle/commit/5ceee4b0bf7f0e37e8a277d86e94d3254aebb9da))
* **execute:** implement execute ([8abfd3a](https://github.com/s00d/switchshuttle/commit/8abfd3a9aed2c935b81c615b28ecdf83c8a72777))
* **execute:** refactor ([10ac224](https://github.com/s00d/switchshuttle/commit/10ac224c6a7d7e3e42406e1d673442ddcdbf6302))
* **help:** add FAQs for scheduling commands and background execution ([264cdfd](https://github.com/s00d/switchshuttle/commit/264cdfd59d9cda6e0718e6b93fe218a15f6baf22))
* **helpers:** add debug logging for menu item creation ([39a911b](https://github.com/s00d/switchshuttle/commit/39a911becdea1d92966e651e8c65e77fe88cfc33))
* **help:** update ([861f24b](https://github.com/s00d/switchshuttle/commit/861f24b10272cb0e9a5d4dec0b89df3414a5b758))
* **HotkeyInput:** add dropdown for selecting key combinations ([141926e](https://github.com/s00d/switchshuttle/commit/141926e4b0b678ce32130a6d75fb73c6cea4df01))
* **hotkeys:** implement global hotkey management and execution ([3db9481](https://github.com/s00d/switchshuttle/commit/3db9481d8e416dda7d0a80cee1e52d141c1f0cda))
* **hotkeys:** implement hotkey management with conflict detection ([4d515f8](https://github.com/s00d/switchshuttle/commit/4d515f800510540660f65f1f6863f8d614791870))
* **html:** add changelog modal to HTML generation ([4668ded](https://github.com/s00d/switchshuttle/commit/4668ded552ff33ac17affa99d408d3d67080ddd2))
* **i18n:** add 'changelog' entry to multiple language files ([a21b7fe](https://github.com/s00d/switchshuttle/commit/a21b7fe1a46d16304c20db50b72c1989569873d1))
* **i18n:** update window titles for improved localization ([ab0fbae](https://github.com/s00d/switchshuttle/commit/ab0fbae0ba06181e73a3bbf85ebf69361bc58593))
* **icons:** add new SVG icons for Add, Lightning, Chevron, Trash, and X ([dc26be2](https://github.com/s00d/switchshuttle/commit/dc26be2eab89e4740e9d2a245aab7c63e917ae7e))
* **icons:** add SettingsIcon component for user settings ([01f33a7](https://github.com/s00d/switchshuttle/commit/01f33a7099f95f5e8334d9247ea49cad827338c2))
* **IconSelector:** add icon selector component with dropdown functionality ([031083d](https://github.com/s00d/switchshuttle/commit/031083d6ae80d5e7557b24f688a976dcc7d11232))
* **input:** add min and max props for input validation ([1d1f2a5](https://github.com/s00d/switchshuttle/commit/1d1f2a5d5c9421542b941b10da8e36b49fba3faf))
* **inputs:** improve form handling and error management ([b1f2cfd](https://github.com/s00d/switchshuttle/commit/b1f2cfd0b3b8f18549c03cd2652a79627d4b6e6a))
* **menu_structure:** add debug logging for switch and monitor checks ([abc31d6](https://github.com/s00d/switchshuttle/commit/abc31d621dbcf5e6e3d3ae9ecb2fcd8e450681a3))
* **menu_structure:** add method to include items in submenu ([0ed83d2](https://github.com/s00d/switchshuttle/commit/0ed83d2973d181b9ddf96197f899148bfa424929))
* **menu_structure:** refactor tray activity tracking and command execution ([bdd5011](https://github.com/s00d/switchshuttle/commit/bdd5011e8d354d36fdb1d2861d0bd41a02db8006))
* **menu:** add command execution by ID with input handling ([febd72a](https://github.com/s00d/switchshuttle/commit/febd72ad3dcde636dea73958728ed302b28274ae))
* **menu:** add scheduler functionality for menu items ([223700d](https://github.com/s00d/switchshuttle/commit/223700d444828c8cbbd215425f48fa1268bec3cf))
* **menu:** add settings option to the tray menu ([ed985bc](https://github.com/s00d/switchshuttle/commit/ed985bc9f36d70e93824bb146f9ee556b7afa37d))
* **menu:** enhance menu item creation with console command execution ([ce92755](https://github.com/s00d/switchshuttle/commit/ce92755ebac791cb9c96d0a47b8111b696e7e257))
* **menu:** implement system menu structure and functionality ([de1ba10](https://github.com/s00d/switchshuttle/commit/de1ba101dfccd824088027be41758009d2da904b))
* **monitoring:** add new monitoring templates and enhance existing ones ([0f1669f](https://github.com/s00d/switchshuttle/commit/0f1669f11156c5fdc8a20357ddfe7cd2eff4c229))
* **router:** replace update route with settings route ([3a3bc26](https://github.com/s00d/switchshuttle/commit/3a3bc26af615a6a2aa4af04af7fe2574cde4c487))
* **scheduler:** add new scheduler template for automated tasks ([f8f0df0](https://github.com/s00d/switchshuttle/commit/f8f0df0a56342b4921e43266410931b59638d253))
* **scheduler:** add SchedulerInput component for cron expression management ([1fbfed8](https://github.com/s00d/switchshuttle/commit/1fbfed86e3624c916571d4bdeb7fd48b6a59058d))
* **scripts:** add AppleScript automation for Visual Studio Code ([8413071](https://github.com/s00d/switchshuttle/commit/8413071cd735e915e7555bd5ef6f3879954ed24a))
* **scripts:** replace old iTerm scripts with updated versions ([c40d9b9](https://github.com/s00d/switchshuttle/commit/c40d9b9badcd1360fc8db67fd8c651fa711e9864))
* **settings:** add application settings management for notifications and auto-start ([154f26f](https://github.com/s00d/switchshuttle/commit/154f26f460f17835c057c4b1b3c1dbddcfc88ac1))
* **settings:** add settings management and user options page ([9b32c3d](https://github.com/s00d/switchshuttle/commit/9b32c3d24d10cc34877c08bb81e37e2c066eeb9e))
* **settings:** add settings management to application initialization ([d3920fd](https://github.com/s00d/switchshuttle/commit/d3920fddb3c790eb37d8fb23e951a0f7890ed53f))
* **settings:** add settings page for configuring application options ([39c2b17](https://github.com/s00d/switchshuttle/commit/39c2b17b803c3dea99ad0cd202305393d3a1b6bd))
* **tauri-commands:** add initial implementation of Tauri command handlers ([5418147](https://github.com/s00d/switchshuttle/commit/5418147dbf3adf7757312259ab2b8d38b22b6b8f))
* **tauri-commands:** add terminal configuration and retrieval methods ([1252f78](https://github.com/s00d/switchshuttle/commit/1252f78d1f5d5c5a233010e210958060f3272a3e))
* **tauri-commands:** add Vue plugin for global access to Tauri commands ([6cbbb5c](https://github.com/s00d/switchshuttle/commit/6cbbb5ce9327bcda2e9357da4cfbba827647e7f4))
* **tauri.conf:** add editor URL to main window configuration ([d9bfe59](https://github.com/s00d/switchshuttle/commit/d9bfe59d06df6f9fc8ae5b127b73d5d285b608fd))
* **tauri.conf:** update window configurations for inputs and settings ([ffaadde](https://github.com/s00d/switchshuttle/commit/ffaadde93cb82eb07955a16cee9b6ff5ea436bc9))
* **TemplateCommandsModal:** add command description display ([7e77a89](https://github.com/s00d/switchshuttle/commit/7e77a89e8635d053b223a2cc79db74f78f3c8790))
* **templates:** add background support for shell switch commands ([a6877fd](https://github.com/s00d/switchshuttle/commit/a6877fdfda29279ff39e791ad6d79ca74b5a2386))
* **templates:** add SSH and network command templates ([db326c4](https://github.com/s00d/switchshuttle/commit/db326c4ac30a2a0f76536580e2dbf3c3b128245c))
* **terminals:** add terminal options by platform ([4e4a846](https://github.com/s00d/switchshuttle/commit/4e4a846dfd1e6f796a281a88c7ec8b090cae153e))
* **tray:** add mouse event handlers for timer management ([a5e1a67](https://github.com/s00d/switchshuttle/commit/a5e1a676bf6e1395a80187662d92125da0a65377))
* **types:** add scheduler and background properties to SwitchShuttle type ([a4fd086](https://github.com/s00d/switchshuttle/commit/a4fd086bede4866e0a201b970d010bf1930e5a85))
* **ui:** add changelog window to the main interface ([009d31d](https://github.com/s00d/switchshuttle/commit/009d31d3147049d33e183894ffcd5e81c931968b))
* **ui:** refactor window management and improve component loading ([3b78816](https://github.com/s00d/switchshuttle/commit/3b788165fc24858b35898d6fbfb01776aea319ac))

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
