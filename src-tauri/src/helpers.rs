use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tauri::image::Image;
use tauri::menu::{CheckMenuItem, IconMenuItem, IconMenuItemBuilder};
use tauri::path::BaseDirectory;
use tauri::{AppHandle, Manager, WebviewWindowBuilder, Wry};
use log::{error, info};

pub fn open_in_default_editor(path: &PathBuf) {
    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg(path)
            .spawn()
            .expect("Failed to open file in default editor");
    }

    #[cfg(target_os = "windows")]
    {
        Command::new("cmd")
            .args(&["/C", "start", path.to_str().unwrap()])
            .spawn()
            .expect("Failed to open file in default editor");
    }

    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open")
            .arg(path)
            .spawn()
            .expect("Failed to open file in default editor");
    }
}

pub fn get_config_path() -> PathBuf {
    let mut config_path = dirs::config_dir().unwrap();
    config_path.push("switch-shuttle");
    fs::create_dir_all(&config_path).unwrap();
    config_path.push("config.json");
    config_path
}

pub fn open_folder_in_default_explorer(path: &PathBuf) {
    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg(path)
            .spawn()
            .expect("Failed to open folder in default explorer");
    }

    #[cfg(target_os = "windows")]
    {
        Command::new("explorer")
            .arg(path)
            .spawn()
            .expect("Failed to open folder in default explorer");
    }

    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open")
            .arg(path)
            .spawn()
            .expect("Failed to open folder in default explorer");
    }
}

pub fn create_window(
    app: &AppHandle,
    window_id: &str,
    title: &str,
    route: &str,
    width: f64,
    height: f64,
    center: bool,
) -> Result<(), String> {
    // Используем отдельное окно
    if let Some(existing_window) = app.get_webview_window(window_id) {
        // Если окно уже существует, проверяем нужно ли изменить роут
        let target_url = format!("tauri://localhost{}", route);

        // Получаем текущий URL и сравниваем с целевым
        if let Ok(current_url) = existing_window.url() {
            if current_url.as_str() != target_url {
                if let Err(e) = existing_window.eval(&format!("window.location.href = '{}'", route))
                {
                    error!("Failed to change route in existing window: {:?}", e);
                }
            }
        }

        // Изменяем размеры окна
        if let Err(e) =
            existing_window.set_size(tauri::Size::Logical(tauri::LogicalSize::new(width, height)))
        {
                    error!("Failed to resize existing window: {:?}", e);
        }

        // Центрируем окно если нужно
        if center {
            if let Err(e) = existing_window.center() {
                error!("Failed to center existing window: {:?}", e);
            }
        }

        // Показываем окно и устанавливаем фокус
        existing_window
            .show()
            .unwrap_or_else(|e| error!("Failed to show existing window: {:?}", e));
        existing_window
            .set_focus()
            .unwrap_or_else(|e| error!("Failed to focus existing window: {:?}", e));
        return Ok(());
    }

    // Создаем новое окно
    let window = WebviewWindowBuilder::new(app, window_id, tauri::WebviewUrl::App(route.into()))
        .title(title)
        .inner_size(width, height)
        .resizable(true)
        .center()
        .build()
        .map_err(|e| format!("Failed to create window: {}", e))?;

    // Устанавливаем иконку если возможно
    if let Ok(icon_path) = app
        .path()
        .resolve("icons/icon.png", BaseDirectory::Resource)
    {
        if let Ok(image) = Image::from_path(icon_path) {
            if let Err(e) = window.set_icon(image) {
                error!("Failed to set window icon: {:?}", e);
            }
        }
    }

    if let Err(e) = window.set_focus() {
        error!("Failed to set window focus: {:?}", e);
    }

    if center {
        if let Err(e) = window.center() {
            error!("Failed to center window: {:?}", e);
        }
    }

    window
        .show()
        .unwrap_or_else(|e| error!("Failed to show existing window: {:?}", e));
    window
        .set_focus()
        .unwrap_or_else(|e| error!("Failed to focus existing window: {:?}", e));

    Ok(())
}

#[cfg(debug_assertions)]
pub fn change_devtools(app: &AppHandle) {
    let window = app.get_webview_window("main").unwrap();
    if !window.is_devtools_open() {
        window.open_devtools();
    } else {
        window.close_devtools();
    }
}

#[cfg(not(debug_assertions))]
pub fn change_devtools(_app: &AppHandle) {}

/// Создает пункт меню с иконкой и опциональной горячей клавишей
pub fn create_menu_item(
    app: &AppHandle<Wry>,
    id: &str,
    text: &str,
    icon_name: &str,
    hotkey: Option<String>,
    icon: Option<&str>,
) -> IconMenuItem<Wry> {
    info!(
        "[Menu Item] Creating menu item with ID: '{}', text: '{}'",
        id, text
    );

    let display_text = if let Some(icon_symbol) = icon {
        format!("{} {}", icon_symbol, text)
    } else {
        text.to_string()
    };

    let mut builder = IconMenuItemBuilder::with_id(id, &display_text);

    // Пытаемся загрузить иконку, но не падаем если её нет
    // Добавляем иконку только если нет пользовательской иконки
    if icon.is_none() {
        if let Ok(icon_path) = app
            .path()
            .resolve(&format!("icons/{}.png", icon_name), BaseDirectory::Resource)
        {
            if let Ok(image) = Image::from_path(icon_path) {
                builder = builder.icon(image);
            }
        }
    }

    // Добавляем горячую клавишу если указана
    if let Some(hotkey) = hotkey {
        builder = builder.accelerator(&hotkey);
    }

    builder.build(app).unwrap()
}

/// Создает чекбокс пункт меню с опциональной горячей клавишей
pub fn create_check_menu_item(
    app: &AppHandle<Wry>,
    id: &str,
    text: &str,
    checked: bool,
    hotkey: Option<String>,
    icon: Option<&str>,
) -> CheckMenuItem<Wry> {
    info!(
        "[Menu Item] Creating check menu item with ID: '{}', text: '{}', checked: {}",
        id, text, checked
    );

    let display_text = if let Some(icon_symbol) = icon {
        format!("{} {}", icon_symbol, text)
    } else {
        text.to_string()
    };

    let hotkey_ref = hotkey.as_ref().map(|s| s.as_str());

    CheckMenuItem::with_id(
        app.app_handle(),
        id,
        &display_text,
        true, // enabled
        checked,
        hotkey_ref,
    )
    .unwrap()
}
