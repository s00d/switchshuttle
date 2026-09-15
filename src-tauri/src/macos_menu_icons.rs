//! macOS menu icons: muda loads PNGs without `NSImage.setTemplate(true)`,
//! so black template assets stay black in Dark Mode. After attaching the tray
//! menu we walk `NSMenu` and mark every item image as a template so AppKit
//! flips them correctly for light/dark and highlighted rows.

use log::warn;
use objc2::MainThreadMarker;
use objc2_app_kit::{NSMenu, NSStatusItem};
use tauri::tray::TrayIcon;
use tauri::Runtime;

fn mark_nsmenu_images_as_templates(menu: &NSMenu) {
    let items = menu.itemArray();
    for item in items {
        if let Some(image) = item.image() {
            if !image.isTemplate() {
                image.setTemplate(true);
                item.setImage(Some(&image));
            }
        }
        if let Some(submenu) = item.submenu() {
            mark_nsmenu_images_as_templates(&submenu);
        }
    }
}

fn mark_status_item_menu(status_item: &NSStatusItem, mtm: MainThreadMarker) {
    if let Some(menu) = status_item.menu(mtm) {
        mark_nsmenu_images_as_templates(&menu);
    }
}

/// Mark all tray-menu PNG images as AppKit templates. Call after `set_menu`.
pub fn mark_tray_menu_icons_as_templates<R: Runtime>(tray: &TrayIcon<R>) {
    let result = tray.with_inner_tray_icon(|inner| {
        let Some(mtm) = MainThreadMarker::new() else {
            warn!("[Menu Icons] Not on main thread; skipping template mark");
            return;
        };
        let Some(status_item) = inner.ns_status_item() else {
            warn!("[Menu Icons] No NSStatusItem; skipping template mark");
            return;
        };
        mark_status_item_menu(&status_item, mtm);
    });

    if let Err(e) = result {
        warn!("[Menu Icons] Failed to mark tray menu icons as templates: {e}");
    }
}
