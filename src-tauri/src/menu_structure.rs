use crate::config::CommandConfig;
use crate::{console, helpers};
use std::sync::{Arc};
use std::time::Duration;
use std::thread;
use tauri::{AppHandle, Wry};
use tauri::menu::{
    CheckMenuItem, IconMenuItem, MenuBuilder, Submenu as TauriSubmenu, SubmenuBuilder,
};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Mutex;

use crate::helpers::{create_menu_item, create_check_menu_item};

// Глобальная переменная для отслеживания активности трея
lazy_static::lazy_static! {
    pub static ref TRAY_ACTIVE: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
}

/// Представляет элемент меню
#[derive(Clone)]
pub struct MenuItem {
    // Наследуем все поля от CommandConfig
    pub config: crate::config::CommandConfig,
    // Добавляем только специфичные для меню поля
    pub tauri_icon_item: Option<Arc<IconMenuItem<Wry>>>,
    pub stop_flag: Option<Arc<AtomicBool>>,
}

/// Представляет подменю
#[derive(Clone)]
pub struct Submenu {
    pub name: String,
    pub icon: Option<String>,
    pub items: Vec<MenuItem>,
}

/// Представляет полное меню системы
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
        }
    }

    /// Проверяет, является ли элемент переключателем
    pub fn is_switch(&self) -> bool {
        self.config.switch.is_some()
    }

    /// Проверяет, является ли элемент командой мониторинга
    pub fn is_monitor(&self) -> bool {
        self.config.monitor.is_some()
    }

    /// Проверяет, имеет ли элемент подменю
    pub fn has_submenu(&self) -> bool {
        self.config.submenu.is_some() && !self.config.submenu.as_ref().unwrap().is_empty()
    }

    /// Получает отображаемое имя с учетом мониторинга
    pub fn get_display_name(&self, _app: Option<&AppHandle<Wry>>) -> String {
        if self.is_monitor() {
            if let Some(monitor_command) = &self.config.monitor {
                // Выполняем команду мониторинга
                match console::execute_command_silent(monitor_command) {
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
        if self.is_monitor() {
            // Проверяем, что команда мониторинга не пустая
            if let Some(monitor_command) = &self.config.monitor {
                if monitor_command.trim().is_empty() {
                    eprintln!("[Monitor] Empty monitor command for item: {}", self.config.id.as_ref().unwrap_or(&self.config.name));
                    return;
                }
            } else {
                eprintln!("[Monitor] No monitor command for item: {}", self.config.id.as_ref().unwrap_or(&self.config.name));
                return;
            }
            
            // Останавливаем предыдущий поток, если был
            self.stop_monitor_timer();
            let stop_flag = Arc::new(AtomicBool::new(false));
            self.stop_flag = Some(stop_flag.clone());
            if let Some(icon_item) = self.tauri_icon_item.clone() {
                let monitor_command = self.config.monitor.clone().unwrap();
                let name = self.config.name.clone();
                let id = self.config.id.clone().unwrap_or_else(|| self.config.name.clone());
                let icon = self.config.icon.clone();
                thread::spawn(move || {
                    eprintln!("[Monitor] Starting timer for item: {}", id);
                    while !stop_flag.load(Ordering::Relaxed) {
                        // Проверяем активность трея
                        let tray_active = TRAY_ACTIVE.lock().unwrap();
                        if *tray_active {
                            drop(tray_active); // Освобождаем блокировку
                            
                            let new_text = match crate::console::execute_command_silent(&monitor_command) {
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
                            eprintln!("[Monitor] Updating text for {}: {}", id, new_text);
                            
                            // Обновляем текст с сохранением иконки
                            if let Some(icon_str) = &icon {
                                if let Err(e) = icon_item.set_text(&format!("{} {}", icon_str, new_text)) {
                                    eprintln!("[Monitor] Failed to set text with icon for {}: {}", id, e);
                                } else {
                                    eprintln!("[Monitor] Successfully updated text with icon for {}", id);
                                }
                            } else {
                                if let Err(e) = icon_item.set_text(&new_text) {
                                    eprintln!("[Monitor] Failed to set text for {}: {}", id, e);
                                } else {
                                    eprintln!("[Monitor] Successfully updated text for {}", id);
                                }
                            }
                        } else {
                            drop(tray_active); // Освобождаем блокировку
                            eprintln!("[Monitor] Timer paused for item: {}", id);
                        }
                        thread::sleep(Duration::from_secs(1));
                    }
                    eprintln!("[Monitor] Timer stopped for item: {}", id);
                });
            } else {
                eprintln!("[Monitor] No icon_item found for {}", self.config.id.as_ref().unwrap_or(&self.config.name));
            }
        }
    }

    /// Останавливает таймер мониторинга (выставляет флаг)
    pub fn stop_monitor_timer(&mut self) {
        if let Some(flag) = &self.stop_flag {
            flag.store(true, Ordering::Relaxed);
        }
        self.stop_flag = None;
    }

    /// Получает состояние переключателя
    pub fn get_switch_state(&self, app: Option<&AppHandle<Wry>>) -> bool {
        if self.is_switch() {
            let id = self.config.id.as_ref().unwrap_or(&self.config.name);
            helpers::is_switch_enabled(id, app)
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
        
        if self.is_switch() {
            let is_enabled = self.get_switch_state(Some(app));
            MenuItemOrSubmenu::CheckItem(create_check_menu_item(app, id, name, is_enabled, hotkey.map(|s| s.to_string()), icon))
        } else if self.is_monitor() {
            eprintln!("[Monitor] Creating menu item for monitor: {}", id);
            let display_name = self.get_display_name(Some(app));
            let icon_item = create_menu_item(app, id, &display_name, "terminal", hotkey.map(|s| s.to_string()), icon);
            let arc_icon_item = Arc::new(icon_item);
            self.tauri_icon_item = Some(arc_icon_item.clone());
            eprintln!("[Monitor] Saved icon_item for: {}", id);
            MenuItemOrSubmenu::IconItem(Arc::try_unwrap(arc_icon_item).unwrap_or_else(|arc| (*arc).clone()))
        } else {
            MenuItemOrSubmenu::IconItem(create_menu_item(app, id, name, "terminal", hotkey.map(|s| s.to_string()), icon))
        }
    }

    /// Создает Tauri подменю из MenuItem с подменю
    pub fn create_tauri_submenu(&mut self, app: &AppHandle<Wry>) -> TauriSubmenu<Wry> {
        let display_name = if let Some(icon) = &self.config.icon {
            format!("{} {}", icon, self.config.name)
        } else {
            format!("📁 {}", self.config.name)
        };
        
        let mut submenu_builder = SubmenuBuilder::new(app, &display_name);
        
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
    pub fn with_icon(mut self, icon: String) -> Self {
        self.icon = Some(icon);
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
            format!("📁 {}", self.name)
        }
    }

    /// Создает Tauri подменю из структуры Submenu
    pub fn create_tauri_submenu(&mut self, app: &AppHandle<Wry>) -> TauriSubmenu<Wry> {
        let display_name = self.get_display_name();
        let mut submenu_builder = SubmenuBuilder::new(app, &display_name);
        
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
    pub fn from_configs_with_states(configs: &[crate::config::Config], _app: Option<&AppHandle<Wry>>) -> Self {
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
                        .with_icon(command.icon.clone().unwrap_or_else(|| "📁".to_string()));
                    
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
        eprintln!("[Monitor] Starting timers for all monitor items");
        
        // Запускаем таймеры для основных элементов
        for item in &mut self.items {
            if item.is_monitor() {
                eprintln!("[Monitor] Found monitor item in main items: {}", item.config.id.as_ref().unwrap_or(&item.config.name));
                item.start_monitor_timer();
            }
        }
        
        // Запускаем таймеры для элементов в подменю
        for submenu in &mut self.submenus {
            for item in &mut submenu.items {
                if item.is_monitor() {
                    eprintln!("[Monitor] Found monitor item in submenu: {}", item.config.id.as_ref().unwrap_or(&item.config.name));
                    item.start_monitor_timer();
                }
            }
        }
        
        eprintln!("[Monitor] Finished starting timers");
    }

    /// Останавливает все таймеры мониторинга
    pub fn stop_all_monitor_timers(&mut self) {
        eprintln!("[Monitor] Stopping all monitor timers");
        
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
        
        eprintln!("[Monitor] Finished stopping timers");
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