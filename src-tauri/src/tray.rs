use tauri::{
    image::Image,
    menu::{Menu, MenuItem, MenuEvent},
    tray::{MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager, Result,
};

use crate::window::{toggle_window, toggle_window_visibility};

// 編譯時嵌入 tray icon（依平台選擇不同圖標）
#[cfg(target_os = "macos")]
const TRAY_ICON: &[u8] = include_bytes!("../icons/tray-icon-macos.png"); // 黑色 template

#[cfg(not(target_os = "macos"))]
const TRAY_ICON: &[u8] = include_bytes!("../icons/tray-icon-win.png"); // 彩色

/// 創建系統托盤
pub fn setup_system_tray(app: &AppHandle) -> Result<()> {
    let show_item = MenuItem::with_id(app, "show", "顯示搜尋", true, None::<&str>)?;
    let quit_item = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&show_item, &quit_item])?;

    // 從嵌入的 bytes 載入 icon
    let icon = Image::from_bytes(TRAY_ICON).expect("Failed to load tray icon");

    TrayIconBuilder::new()
        .icon(icon)
        .icon_as_template(true) // macOS: 自動適應深色/淺色模式
        .menu(&menu)
        .tooltip("Mizly")
        .on_menu_event(move |app, event| handle_tray_menu_event(app, event))
        .on_tray_icon_event(handle_tray_icon_event)
        .build(app)?;

    Ok(())
}

/// 處理托盤選單事件
fn handle_tray_menu_event(app: &AppHandle, event: MenuEvent) {
    match event.id.as_ref() {
        "quit" => {
            app.exit(0);
        }
        "show" => {
            if let Some(window) = app.get_webview_window("main") {
                let _ = toggle_window_visibility(&window, true);
            }
        }
        _ => {}
    }
}

/// 處理托盤圖標事件
fn handle_tray_icon_event(tray: &TrayIcon, event: TrayIconEvent) {
    match event {
        TrayIconEvent::Click {
            button: MouseButton::Left,
            button_state: MouseButtonState::Up,
            ..
        } => {
            if let Some(window) = tray.app_handle().get_webview_window("main") {
                let _ = toggle_window(&window);
            }
        }
        _ => {}
    }
}