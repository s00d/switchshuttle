use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri_plugin_autostart::ManagerExt;
use tauri_plugin_notification::{NotificationExt, PermissionState};
use serde_json::Value;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppSettings {
    pub general: GeneralSettings,
    pub notifications: NotificationSettings,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GeneralSettings {
    pub auto_start: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NotificationSettings {
    pub show_notifications: bool,
    pub notification_duration: u32,
    pub success_notifications: bool,
    pub error_notifications: bool,
    pub info_notifications: bool,
    pub warning_notifications: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        AppSettings {
            general: GeneralSettings {
                auto_start: false,
            },
            notifications: NotificationSettings {
                show_notifications: true,
                notification_duration: 3000,
                success_notifications: true,
                error_notifications: true,
                info_notifications: true,
                warning_notifications: true,
            },
        }
    }
}

impl AppSettings {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let settings_path = get_settings_path();
        
        if settings_path.exists() {
            let content = fs::read_to_string(&settings_path)?;
            let mut value: Value = serde_json::from_str(&content)?;
            let default = serde_json::to_value(AppSettings::default())?;
            merge_defaults(&mut value, &default);
            // Удаляем лишние поля
            remove_extra_fields(&mut value, &default);
            let settings: AppSettings = serde_json::from_value(value)?;
            Ok(settings)
        } else {
            let settings = AppSettings::default();
            settings.save()?;
            Ok(settings)
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let settings_path = get_settings_path();
        let content = serde_json::to_string_pretty(self)?;
        fs::write(settings_path, content)?;
        Ok(())
    }

    pub fn apply(&self, app: &tauri::AppHandle) -> Result<(), Box<dyn std::error::Error>> {
        // Применяем настройки автозапуска
        let autostart_manager = app.autolaunch();
        let current_autostart = autostart_manager.is_enabled().unwrap_or(false);
        let new_autostart = self.general.auto_start;
        
        if current_autostart != new_autostart {
            if new_autostart {
                autostart_manager.enable()?;
            } else {
                autostart_manager.disable()?;
            }
        }

        
        // Применяем настройки уведомлений
        // (эти настройки будут использоваться в других частях приложения)
        
        // Применяем настройки языка
        // (можно добавить логику для смены языка интерфейса)
        
        Ok(())
    }



    pub fn show_notification(&self, app: &tauri::AppHandle, title: &str, body: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.show_notification_with_icon(app, title, body, None)
    }

    fn show_notification_with_icon(&self, app: &tauri::AppHandle, title: &str, body: &str, icon: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
        println!("[Settings] Attempting to show notification: title='{}', body='{}', icon='{:?}'", title, body, icon);
        
        if !self.notifications.show_notifications {
            println!("[Settings] Notifications are disabled in settings");
            return Ok(());
        }

        // Проверяем права на уведомления
        let permission_state = app.notification().permission_state();
        println!("[Settings] Permission state: {:?}", permission_state);
        
        match permission_state {
            Ok(state) if state == PermissionState::Granted => {
                println!("[Settings] Permission granted, proceeding with notification");
            }
            _ => {
                println!("[Settings] Permission not granted, requesting permission");
                // Запрашиваем права если не предоставлены
                if let Err(e) = app.notification().request_permission() {
                    println!("[Settings] Failed to request permission: {}", e);
                    // Показываем уведомление об ошибке вместо вывода в консоль
                    let _ = self.show_error_notification(app, "Ошибка разрешений", &format!("Не удалось запросить разрешение на уведомления: {}", e));
                }
                return Ok(());
            }
        }

        let mut builder = app.notification().builder()
            .title(title)
            .body(body);

        if let Some(icon) = icon {
            builder = builder.icon(icon);
        }

        println!("[Settings] Building notification with title='{}', body='{}'", title, body);
        
        match builder.show() {
            Ok(_) => {
                println!("[Settings] Notification shown successfully");
                Ok(())
            }
            Err(e) => {
                println!("[Settings] Failed to show notification: {}", e);
                Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)))
            }
        }
    }

    pub fn show_success_notification(&self, app: &tauri::AppHandle, title: &str, body: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("[Settings] show_success_notification called: title='{}', body='{}'", title, body);
        
        if !self.notifications.success_notifications {
            println!("[Settings] Success notifications are disabled in settings");
            return Ok(());
        }

        // Используем базовый метод с иконкой успеха
        self.show_notification_with_icon(app, title, body, Some("✅"))
    }

    pub fn show_error_notification(&self, app: &tauri::AppHandle, title: &str, body: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("[Settings] show_error_notification called: title='{}', body='{}'", title, body);
        
        if !self.notifications.error_notifications {
            println!("[Settings] Error notifications are disabled in settings");
            return Ok(());
        }

        // Используем базовый метод с иконкой ошибки
        self.show_notification_with_icon(app, title, body, Some("❌"))
    }

    pub fn show_info_notification(&self, app: &tauri::AppHandle, title: &str, body: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("[Settings] show_info_notification called: title='{}', body='{}'", title, body);
        
        if !self.notifications.info_notifications {
            println!("[Settings] Info notifications are disabled in settings");
            return Ok(());
        }

        // Используем базовый метод с иконкой информации
        self.show_notification_with_icon(app, title, body, Some("ℹ️"))
    }

    pub fn show_warning_notification(&self, app: &tauri::AppHandle, title: &str, body: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("[Settings] show_warning_notification called: title='{}', body='{}'", title, body);
        
        if !self.notifications.warning_notifications {
            println!("[Settings] Warning notifications are disabled in settings");
            return Ok(());
        }

        // Используем базовый метод с иконкой предупреждения
        self.show_notification_with_icon(app, title, body, Some("⚠️"))
    }

    pub fn get_settings_schema() -> serde_json::Value {
        serde_json::json!({
            "sections": [
                {
                    "id": "general",
                    "title": "General",
                    "description": "Basic application settings",
                    "fields": [
                        {
                            "id": "general.auto_start",
                            "type": "boolean",
                            "label": "Auto-start with system",
                            "description": "Automatically start SwitchShuttle when your computer boots up",
                            "default": false
                        },
                    ]
                },
                {
                    "id": "notifications",
                    "title": "Notifications",
                    "description": "Notification and alert settings",
                    "fields": [
                        {
                            "id": "notifications.show_notifications",
                            "type": "boolean",
                            "label": "Show notifications",
                            "description": "Display system notifications",
                            "default": true
                        },
                        {
                            "id": "notifications.notification_duration",
                            "type": "number",
                            "label": "Notification duration",
                            "description": "How long to show notifications (milliseconds)",
                            "min": 1000,
                            "max": 10000,
                            "default": 3000
                        },
                        {
                            "id": "notifications.success_notifications",
                            "type": "boolean",
                            "label": "Success notifications",
                            "description": "Show notifications for successful operations",
                            "default": true
                        },
                        {
                            "id": "notifications.error_notifications",
                            "type": "boolean",
                            "label": "Error notifications",
                            "description": "Show notifications for errors",
                            "default": true
                        },
                        {
                            "id": "notifications.info_notifications",
                            "type": "boolean",
                            "label": "Info notifications",
                            "description": "Show informational notifications",
                            "default": true
                        },
                        {
                            "id": "notifications.warning_notifications",
                            "type": "boolean",
                            "label": "Warning notifications",
                            "description": "Show warning notifications",
                            "default": true
                        }
                    ]
                }
            ]
        })
    }
}

pub fn get_settings_path() -> PathBuf {
    let config_dir = dirs::config_dir()
        .expect("Failed to get config directory")
        .join("switchshuttle");
    std::fs::create_dir_all(&config_dir).ok();
    config_dir.join("settings.json")
}

fn merge_defaults(current: &mut Value, default: &Value) {
    match (current, default) {
        (Value::Object(cur_map), Value::Object(def_map)) => {
            for (k, v) in def_map {
                match cur_map.get_mut(k) {
                    Some(cur_v) => merge_defaults(cur_v, v),
                    None => { cur_map.insert(k.clone(), v.clone()); },
                }
            }
        }
        _ => {}
    }
}

fn remove_extra_fields(current: &mut Value, default: &Value) {
    if let (Value::Object(cur_map), Value::Object(def_map)) = (current, default) {
        let keys: Vec<_> = cur_map.keys().cloned().collect();
        for k in keys {
            if !def_map.contains_key(&k) {
                cur_map.remove(&k);
            } else {
                remove_extra_fields(cur_map.get_mut(&k).unwrap(), &def_map[&k]);
            }
        }
    }
} 