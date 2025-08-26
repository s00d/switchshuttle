use crate::config::CommandConfig;
use crate::console;
use chrono;
use cron;
use once_cell::sync::Lazy;
use std::str::FromStr;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use tauri::menu::{
    CheckMenuItem, IconMenuItem, MenuBuilder, Submenu as TauriSubmenu, SubmenuBuilder,
};
use tauri::{AppHandle, Wry, image::Image};
use log::{error, info};

use crate::helpers::{create_check_menu_item, create_menu_item};

// Глобальная переменная для отслеживания активности трея
pub static TRAY_ACTIVE: Lazy<Arc<Mutex<bool>>> = Lazy::new(|| Arc::new(Mutex::new(false)));

/// Представляет элемент меню
#[derive(Clone)]
pub struct MenuItem {
    // Наследуем все поля от CommandConfig
    pub config: crate::config::CommandConfig,
    // Добавляем только специфичные для меню поля
    pub tauri_icon_item: Option<Arc<IconMenuItem<Wry>>>,
    pub stop_flag: Option<Arc<AtomicBool>>,
    pub scheduler_stop_flag: Option<Arc<AtomicBool>>,
}

/// Представляет подменю
#[derive(Clone)]
pub struct Submenu {
    pub name: String,
    pub icon: Option<String>,
    pub items: Vec<MenuItem>,
}

/// Представляет полное меню системы
#[derive(Clone)]
pub struct SystemMenu {
    pub items: Vec<MenuItem>,
    pub submenus: Vec<Submenu>,
}

impl MenuItem {
    /// Создает элемент меню из CommandConfig
    pub fn from_command_config(cmd: &CommandConfig) -> Self {
        Self {
            config: cmd.clone(),
            tauri_icon_item: None,
            stop_flag: None,
            scheduler_stop_flag: None,
        }
    }

    /// Проверяет, является ли элемент переключателем
    pub fn is_switch(&self) -> bool {
        let is_switch = self.config.switch.is_some();
        if is_switch {
            info!(
                "[Switch] is_switch: {} has switch = '{:?}'",
                self.config.name, self.config.switch
            );
        }
        info!(
            "[Switch] is_switch called for: {} = {}",
            self.config.name, is_switch
        );
        is_switch
    }

    /// Проверяет, является ли элемент командой мониторинга
    pub fn is_monitor(&self) -> bool {
        let is_monitor = self.config.monitor.is_some() && 
                        self.config.monitor.as_ref().unwrap().trim().is_empty() == false;
        if is_monitor {
            info!(
                "[Monitor] is_monitor: {} has monitor = '{:?}'",
                self.config.name, self.config.monitor
            );
        }
        info!(
            "[Monitor] is_monitor called for: {} = {}",
            self.config.name, is_monitor
        );
        is_monitor
    }

    /// Проверяет, имеет ли элемент планировщик
    pub fn has_scheduler(&self) -> bool {
        self.config.scheduler.is_some() && 
        !self.config.scheduler.as_ref().unwrap().is_empty()
    }

    /// Проверяет, имеет ли элемент подменю
    pub fn has_submenu(&self) -> bool {
        self.config.submenu.is_some() && !self.config.submenu.as_ref().unwrap().is_empty()
    }

    /// Получает отображаемое имя с учетом мониторинга
    pub fn get_display_name(&self, _app: Option<&AppHandle<Wry>>) -> String {
        info!(
            "[Monitor] get_display_name called for: {}",
            self.config.name
        );
        if self.is_monitor() {
            if let Some(monitor_command) = &self.config.monitor {
                info!(
                    "[Monitor] get_display_name: monitor_command = '{}'",
                    monitor_command
                );

                // Выполняем команду мониторинга
                match console::ConsoleInstance::execute_command_silent(monitor_command) {
                    Ok(output) => {
                        let result = output.trim();
                        if !result.is_empty() {
                            format!("{}: {}", self.config.name, result)
                        } else {
                            self.config.name.clone()
                        }
                    }
                    Err(_) => {
                        // В случае ошибки оставляем оригинальное название
                        self.config.name.clone()
                    }
                }
            } else {
                self.config.name.clone()
            }
        } else {
            self.config.name.clone()
        }
    }

    /// Запускает таймер обновления для элемента с мониторингом
    pub fn start_monitor_timer(&mut self) {
        info!(
            "[Monitor] start_monitor_timer called for: {}",
            self.config.name
        );
        if self.is_monitor() {
            // Проверяем, что команда мониторинга не пустая
            if let Some(monitor_command) = &self.config.monitor {
                if monitor_command.trim().is_empty() {
                    error!(
                        "[Monitor] Empty monitor command for item: {}",
                        self.config.id.as_ref().unwrap_or(&self.config.name)
                    );
                    return;
                }
            } else {
                error!(
                    "[Monitor] No monitor command for item: {}",
                    self.config.id.as_ref().unwrap_or(&self.config.name)
                );
                return;
            }

            // Останавливаем предыдущий поток, если был
            self.stop_monitor_timer();
            let stop_flag = Arc::new(AtomicBool::new(false));
            self.stop_flag = Some(stop_flag.clone());
            if let Some(icon_item) = self.tauri_icon_item.clone() {
                let monitor_id = self.config.monitor.clone().unwrap();
                info!(
                    "[Monitor] start_monitor_timer: monitor_id = '{}'",
                    monitor_id
                );
                let name = self.config.name.clone();
                let id = self
                    .config
                    .id
                    .clone()
                    .unwrap_or_else(|| self.config.name.clone());
                let icon = self.config.icon.clone();
                let hotkey = self.config.hotkey.clone();

                // Используем monitor_id как команду для выполнения
                let monitor_command = monitor_id.clone();

                info!("[Monitor] Resolved monitor_command: '{}'", monitor_command);

                thread::spawn(move || {
                    error!("[Monitor] Starting timer for item: {}", id);
                    while !stop_flag.load(Ordering::Relaxed) {
                        // Проверяем активность трея
                        if let Ok(tray_active) = TRAY_ACTIVE.lock() {
                            if *tray_active {
                                drop(tray_active); // Освобождаем блокировку

                                info!(
                                    "[Monitor] spawn: monitor_command = '{}'",
                                    monitor_command
                                );
                                // Используем пул соединений для выполнения команды
                                let new_text =
                                    match console::ConsoleInstance::execute_command_via_pool(
                                        &id,
                                        &monitor_command,
                                    ) {
                                        Ok(output) => {
                                            let result = output.trim();
                                            if !result.is_empty() {
                                                format!("{}: {}", name, result)
                                            } else {
                                                name.clone()
                                            }
                                        }
                                        Err(_) => name.clone(),
                                    };
                                error!("[Monitor] Updating text for {}: {}", id, new_text);

                                if let Err(e) = icon_item.set_accelerator(hotkey.clone()) {
                                    error!(
                                        "[Monitor] Failed to set accelerator for {}: {}",
                                        id, e
                                    );
                                } else {
                                    info!(
                                        "[Monitor] Successfully updated accelerator for {}",
                                        id
                                    );
                                }

                                // Обновляем текст с сохранением иконки
                                if let Some(icon_str) = &icon {
                                    if let Err(e) =
                                        icon_item.set_text(&format!("{} {}", icon_str, new_text))
                                    {
                                        error!(
                                            "[Monitor] Failed to set text with icon for {}: {}",
                                            id, e
                                        );
                                    } else {
                                        info!(
                                            "[Monitor] Successfully updated text with icon for {}",
                                            id
                                        );
                                    }
                                } else {
                                    if let Err(e) = icon_item.set_text(&new_text) {
                                        error!("[Monitor] Failed to set text for {}: {}", id, e);
                                    } else {
                                        info!("[Monitor] Successfully updated text for {}", id);
                                    }
                                }
                            } else {
                                drop(tray_active); // Освобождаем блокировку
                                error!("[Monitor] Timer paused for item: {}", id);
                            }
                        }
                        thread::sleep(Duration::from_secs(1));
                    }
                    error!("[Monitor] Timer stopped for item: {}", id);
                });
            } else {
                error!(
                    "[Monitor] No icon_item found for {}",
                    self.config.id.as_ref().unwrap_or(&self.config.name)
                );
            }
        }
    }

    /// Останавливает таймер мониторинга
    pub fn stop_monitor_timer(&mut self) {
        if let Some(stop_flag) = &self.stop_flag {
            stop_flag.store(true, Ordering::Relaxed);
            self.stop_flag = None;
        }
    }

    /// Запускает планировщик для элемента
    pub fn start_scheduler(&mut self) {
        info!(
            "[Scheduler] start_scheduler called for: {}",
            self.config.name
        );
        if self.has_scheduler() {
            if let Some(scheduler) = &self.config.scheduler {
                let schedule = scheduler.clone();
                let commands = self.config.commands.clone();
                let _name = self.config.name.clone();
                let id = self
                    .config
                    .id
                    .clone()
                    .unwrap_or_else(|| self.config.name.clone());
                
                // Останавливаем предыдущий планировщик, если был
                self.stop_scheduler();
                let stop_flag = Arc::new(AtomicBool::new(false));
                self.scheduler_stop_flag = Some(stop_flag.clone());

                info!("[Scheduler] Starting scheduler for item: {} with schedule: {}", id, schedule);

                thread::spawn(move || {
                    error!("[Scheduler] Starting scheduler for item: {}", id);
                    
                    // Парсим cron выражение
                    let cron_expr = match cron::Schedule::from_str(&schedule) {
                        Ok(schedule) => schedule,
                        Err(e) => {
                            error!("[Scheduler] Failed to parse cron expression '{}': {}", schedule, e);
                            // Используем fallback - каждую минуту
                            cron::Schedule::from_str("0 * * * * *").unwrap()
                        }
                    };
                    
                    info!("[Scheduler] Parsed cron expression: {}", schedule);
                    
                    while !stop_flag.load(Ordering::Relaxed) {
                        // Получаем следующее время выполнения
                        let now = chrono::Utc::now();
                        let next = cron_expr.after(&now).next();
                        
                        if let Some(next_time) = next {
                            let wait_duration = next_time.signed_duration_since(now);
                            let wait_seconds = wait_duration.num_seconds() as u64;
                            
                            info!("[Scheduler] Next execution at: {}, waiting {} seconds", next_time, wait_seconds);
                            
                            // Ждем до следующего выполнения
                            thread::sleep(Duration::from_secs(wait_seconds));
                            
                            // Выполняем команды по расписанию
                            if let Some(cmds) = &commands {
                                for cmd in cmds {
                                    info!("[Scheduler] Executing scheduled command: {}", cmd);
                                    match console::ConsoleInstance::execute_command_silent(cmd) {
                                        Ok(_) => info!("[Scheduler] Command executed successfully"),
                                        Err(e) => error!("[Scheduler] Failed to execute command: {}", e),
                                    }
                                }
                            }
                        } else {
                            // Если не удалось получить следующее время, ждем минуту
                            thread::sleep(Duration::from_secs(60));
                        }
                    }
                    error!("[Scheduler] Scheduler stopped for item: {}", id);
                });
            }
        }
    }

    /// Останавливает планировщик
    pub fn stop_scheduler(&mut self) {
        if let Some(stop_flag) = &self.scheduler_stop_flag {
            stop_flag.store(true, Ordering::Relaxed);
            self.scheduler_stop_flag = None;
        }
    }

    /// Получает состояние переключателя
    pub fn get_switch_state(&self, app: Option<&AppHandle<Wry>>) -> bool {
        if self.is_switch() {
            if let Some(switch_command) = &self.config.switch {
                info!(
                    "[Switch] get_switch_state: switch_command = '{}'",
                    switch_command
                );
                console::ConsoleInstance::is_switch_enabled(switch_command, app)
            } else {
                info!("[Switch] No switch command found for: {}", self.config.name);
                false
            }
        } else {
            false
        }
    }

    /// Создает Tauri элемент меню из структуры MenuItem
    pub fn create_tauri_menu_item(&mut self, app: &AppHandle<Wry>) -> MenuItemOrSubmenu {
        let id = self.config.id.as_ref().unwrap_or(&self.config.name);
        let name = &self.config.name;
        let hotkey = self.config.hotkey.as_deref();
        let icon = self.config.icon.as_deref();

        info!(
            "[Menu Structure] Creating tauri menu item with ID: '{}', name: '{}'",
            id, name
        );
        info!(
            "[Menu Structure] Is switch: {}, is monitor: {}",
            self.is_switch(),
            self.is_monitor()
        );

        if self.is_switch() {
            let is_enabled = self.get_switch_state(Some(app));
            MenuItemOrSubmenu::CheckItem(create_check_menu_item(
                app,
                id,
                name,
                is_enabled,
                hotkey.map(|s| s.to_string()),
                icon,
            ))
        } else if self.is_monitor() {
            error!("[Monitor] Creating menu item for monitor: {}", id);
            let display_name = self.get_display_name(Some(app));
            let icon_item = create_menu_item(
                app,
                id,
                &display_name,
                "terminal",
                hotkey.map(|s| s.to_string()),
                icon,
            );
            let arc_icon_item = Arc::new(icon_item);
            self.tauri_icon_item = Some(arc_icon_item.clone());
            error!("[Monitor] Saved icon_item for: {}", id);
            MenuItemOrSubmenu::IconItem(
                Arc::try_unwrap(arc_icon_item).unwrap_or_else(|arc| (*arc).clone()),
            )
        } else {
            MenuItemOrSubmenu::IconItem(create_menu_item(
                app,
                id,
                name,
                "terminal",
                hotkey.map(|s| s.to_string()),
                icon,
            ))
        }
    }

    /// Создает Tauri подменю из MenuItem с подменю
    pub fn create_tauri_submenu(&mut self, app: &AppHandle<Wry>) -> TauriSubmenu<Wry> {
        let display_name = if let Some(icon) = &self.config.icon {
            format!("{} {}", icon, self.config.name)
        } else {
            self.config.name.clone()
        };

        let mut submenu_builder = SubmenuBuilder::new(app, &display_name);

        if self.config.icon.is_none() {
            let folder_icon = Image::from_bytes(include_bytes!("../icons/folder.png")).unwrap();
            submenu_builder = submenu_builder.submenu_icon(folder_icon);
        }

        if let Some(submenu_items) = &self.config.submenu {
            for sub_item in submenu_items.iter() {
                let mut menu_item = MenuItem::from_command_config(sub_item);
                if menu_item.has_submenu() {
                    let nested_submenu = menu_item.create_tauri_submenu(app);
                    submenu_builder = submenu_builder.item(&nested_submenu);
                } else {
                    let tauri_item = menu_item.create_tauri_menu_item(app);
                    match tauri_item {
                        MenuItemOrSubmenu::IconItem(icon_item) => {
                            submenu_builder = submenu_builder.item(&icon_item);
                        }
                        MenuItemOrSubmenu::CheckItem(check_item) => {
                            submenu_builder = submenu_builder.item(&check_item);
                        }
                        MenuItemOrSubmenu::Submenu(_) => {
                            // Это не должно происходить здесь
                        }
                    }
                }
            }
        }

        submenu_builder.build().unwrap()
    }
}

impl Submenu {
    /// Создает новое подменю
    pub fn new(name: String) -> Self {
        Self {
            name,
            icon: None,
            items: Vec::new(),
        }
    }

    /// Создает подменю с иконкой
    pub fn with_icon(mut self, icon: Option<String>) -> Self {
        self.icon = icon;
        self
    }

    /// Добавляет элемент в подменю
    pub fn add_item(mut self, item: MenuItem) -> Self {
        self.items.push(item);
        self
    }

    /// Добавляет несколько элементов в подменю
    // pub fn add_items(mut self, items: Vec<MenuItem>) -> Self {
    //     self.items.extend(items);
    //     self
    // }

    /// Получает отображаемое имя подменю с иконкой
    pub fn get_display_name(&self) -> String {
        if let Some(icon) = &self.icon {
            format!("{} {}", icon, self.name)
        } else {
            self.name.clone()
        }
    }

    /// Создает Tauri подменю из структуры Submenu
    pub fn create_tauri_submenu(&mut self, app: &AppHandle<Wry>) -> TauriSubmenu<Wry> {
        let display_name = self.get_display_name();
        
        let mut submenu_builder = SubmenuBuilder::new(app, &display_name);

        if self.icon.is_none() {
            let folder_icon = Image::from_bytes(include_bytes!("../icons/folder.png")).unwrap();
            submenu_builder = submenu_builder.submenu_icon(folder_icon);
        }

        for item in self.items.iter_mut() {
            if item.has_submenu() {
                // Рекурсивно создаем вложенное подменю
                let nested_submenu = item.create_tauri_submenu(app);
                submenu_builder = submenu_builder.item(&nested_submenu);
            } else {
                let tauri_item = item.create_tauri_menu_item(app);
                match tauri_item {
                    MenuItemOrSubmenu::IconItem(icon_item) => {
                        submenu_builder = submenu_builder.item(&icon_item);
                    }
                    MenuItemOrSubmenu::CheckItem(check_item) => {
                        submenu_builder = submenu_builder.item(&check_item);
                    }
                    MenuItemOrSubmenu::Submenu(_) => {
                        // Это не должно происходить здесь
                    }
                }
            }
        }

        submenu_builder.build().unwrap()
    }
}

impl SystemMenu {
    /// Создает новое системное меню
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            submenus: Vec::new(),
        }
    }

    /// Добавляет элемент в меню
    pub fn add_item(mut self, item: MenuItem) -> Self {
        self.items.push(item);
        self
    }

    /// Добавляет подменю в меню
    pub fn add_submenu(mut self, submenu: Submenu) -> Self {
        self.submenus.push(submenu);
        self
    }

    /// Создает SystemMenu из ConfigManager с обновлением состояний
    pub fn from_configs_with_states(
        configs: &[crate::config::Config],
        _app: Option<&AppHandle<Wry>>,
    ) -> Self {
        let mut system_menu = SystemMenu::new();

        for config in configs {
            // Пропускаем отключенные конфигурации
            if let Some(enabled) = config.enabled {
                if !enabled {
                    continue;
                }
            }

            for command in &config.commands {
                let menu_item = MenuItem::from_command_config(command);

                if menu_item.has_submenu() {
                    // Если у элемента есть подменю, создаем Submenu
                    let mut submenu = Submenu::new(command.name.clone())
                        .with_icon(command.icon.clone());

                    // Добавляем элементы подменю
                    if let Some(submenu_items) = &command.submenu {
                        for sub_command in submenu_items {
                            let sub_menu_item = MenuItem::from_command_config(sub_command);
                            submenu = submenu.add_item(sub_menu_item);
                        }
                    }

                    system_menu = system_menu.add_submenu(submenu);
                } else {
                    // Иначе добавляем как обычный элемент
                    system_menu = system_menu.add_item(menu_item);
                }
            }
        }

        system_menu
    }

    /// Запускает таймеры для всех элементов с мониторингом
    pub fn start_all_monitor_timers(&mut self) {
        info!("[Monitor] start_all_monitor_timers called");
        error!("[Monitor] Starting timers for all monitor items");

        // Запускаем таймеры для основных элементов
        for item in &mut self.items {
            if item.is_monitor() {
                error!(
                    "[Monitor] Found monitor item in main items: {}",
                    item.config.id.as_ref().unwrap_or(&item.config.name)
                );
                item.start_monitor_timer();
            }
        }

        // Запускаем таймеры для элементов в подменю
        for submenu in &mut self.submenus {
            for item in &mut submenu.items {
                if item.is_monitor() {
                    error!(
                        "[Monitor] Found monitor item in submenu: {}",
                        item.config.id.as_ref().unwrap_or(&item.config.name)
                    );
                    item.start_monitor_timer();
                }
            }
        }

        error!("[Monitor] Finished starting timers");
    }

    /// Останавливает все таймеры мониторинга
    pub fn stop_all_monitor_timers(&mut self) {
        error!("[Monitor] Stopping all monitor timers");

        // Останавливаем таймеры для основных элементов
        for item in &mut self.items {
            if item.is_monitor() {
                item.stop_monitor_timer();
            }
        }

        // Останавливаем таймеры для элементов в подменю
        for submenu in &mut self.submenus {
            for item in &mut submenu.items {
                if item.is_monitor() {
                    item.stop_monitor_timer();
                }
            }
        }

        error!("[Monitor] Finished stopping timers");
    }

    /// Запускает планировщики для всех элементов
    pub fn start_all_schedulers(&mut self) {
        info!("[Scheduler] start_all_schedulers called");
        error!("[Scheduler] Starting schedulers for all scheduled items");

        // Запускаем планировщики для основных элементов
        for item in &mut self.items {
            if item.has_scheduler() {
                error!(
                    "[Scheduler] Found scheduled item in main items: {}",
                    item.config.id.as_ref().unwrap_or(&item.config.name)
                );
                item.start_scheduler();
            }
        }

        // Запускаем планировщики для элементов в подменю
        for submenu in &mut self.submenus {
            for item in &mut submenu.items {
                if item.has_scheduler() {
                    error!(
                        "[Scheduler] Found scheduled item in submenu: {}",
                        item.config.id.as_ref().unwrap_or(&item.config.name)
                    );
                    item.start_scheduler();
                }
            }
        }

        error!("[Scheduler] Finished starting schedulers");
    }

    /// Останавливает все планировщики
    pub fn stop_all_schedulers(&mut self) {
        error!("[Scheduler] Stopping all schedulers");

        // Останавливаем планировщики для основных элементов
        for item in &mut self.items {
            if item.has_scheduler() {
                item.stop_scheduler();
            }
        }

        // Останавливаем планировщики для элементов в подменю
        for submenu in &mut self.submenus {
            for item in &mut submenu.items {
                if item.has_scheduler() {
                    item.stop_scheduler();
                }
            }
        }

        error!("[Scheduler] Finished stopping schedulers");
    }

    /// Периодически очищает неактивные соединения в пуле
    pub fn cleanup_console_pool_periodically() {
        thread::spawn(|| {
            loop {
                thread::sleep(Duration::from_secs(60)); // Очистка каждую минуту
                console::ConsoleInstance::cleanup_console_pool();
            }
        });
    }

    /// Создает Tauri меню из SystemMenu
    pub fn create_tauri_menu(&mut self, app: &AppHandle<Wry>) -> tauri::menu::Menu<Wry> {
        let mut tray_menu_builder = MenuBuilder::new(app);
        let mut menu_items = Vec::new();

        // Добавляем основные элементы меню
        for item in &mut self.items {
            if item.has_submenu() {
                let submenu = item.create_tauri_submenu(app);
                menu_items.push(MenuItemOrSubmenu::Submenu(submenu));
            } else {
                let tauri_item = item.create_tauri_menu_item(app);
                menu_items.push(tauri_item);
            }
        }

        // Добавляем подменю
        for submenu in &mut self.submenus {
            let tauri_submenu = submenu.create_tauri_submenu(app);
            menu_items.push(MenuItemOrSubmenu::Submenu(tauri_submenu));
        }

        // Добавляем элементы в меню
        for item in menu_items {
            match item {
                MenuItemOrSubmenu::Submenu(submenu) => {
                    tray_menu_builder = tray_menu_builder.item(&submenu);
                }
                MenuItemOrSubmenu::IconItem(icon_item) => {
                    tray_menu_builder = tray_menu_builder.item(&icon_item);
                }
                MenuItemOrSubmenu::CheckItem(check_item) => {
                    tray_menu_builder = tray_menu_builder.item(&check_item);
                }
            }
        }

        tray_menu_builder.build().unwrap()
    }
}

impl Default for SystemMenu {
    fn default() -> Self {
        Self::new()
    }
}

/// Enum для представления различных типов элементов меню
pub enum MenuItemOrSubmenu {
    Submenu(TauriSubmenu<Wry>),
    IconItem(IconMenuItem<Wry>),
    CheckItem(CheckMenuItem<Wry>),
}
