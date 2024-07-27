use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use serde::Deserialize;
use tauri::Manager;


#[derive(Deserialize)]
struct GitHubRelease {
    tag_name: String,
    html_url: String,
}

pub fn show_about_dialog(app: &tauri::AppHandle, message: String) {
    let about_window = app.get_window("about").unwrap();
    about_window.show().unwrap_or_else(|e| println!("Failed to show loading window: {:?}", e));
    about_window.set_focus().unwrap_or_else(|e| println!("Failed to show loading window: {:?}", e));
    about_window.emit("about_message", message).unwrap_or_else(|e| println!("Failed to show loading window: {:?}", e));
}

pub fn show_update_dialog(app: &tauri::AppHandle, message: String, url: String) {
    let update_window = app.get_window("update").unwrap();
    update_window.show().unwrap_or_else(|e| println!("Failed to show loading window: {:?}", e));
    update_window.set_focus().unwrap_or_else(|e| println!("Failed to show loading window: {:?}", e));
    update_window.emit("update_message", message).unwrap_or_else(|e| println!("Failed to show loading window: {:?}", e));
    update_window.emit("update_url", url).unwrap_or_else(|e| println!("Failed to show loading window: {:?}", e));
}

pub fn check_for_updates(app: &tauri::AppHandle, current_version: String) {
    println!("current_version {:?}", current_version);
    let latest_release_url = "https://api.github.com/repos/s00d/switchshuttle/releases/latest";

    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_str("switchshuttle").unwrap());

    let client = Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();

    match client.get(latest_release_url).send() {
        Ok(response) => {
            println!("response {:?}", response);
            if let Ok(latest_release) = response.json::<GitHubRelease>() {
                println!("latest_release {:?}", latest_release.tag_name);
                let version = latest_release.tag_name.replace("app-v", "");
                if version != current_version {
                    let update_message = format!(
                        "A new version (v{}) is available! You are currently using v{}.",
                        version, current_version
                    );
                    show_update_dialog(app, update_message, latest_release.html_url);
                } else {
                    show_about_dialog(app, "You are using the latest version.".to_string());
                }
            }
        },
        Err(err) => {
            println!("Error: {:?}", err);
            show_about_dialog(app, "Failed to check for updates.".to_string());
        }
    }
}