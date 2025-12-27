use tauri::{command, AppHandle, Manager};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

use crate::window::toggle_window;

/// 設置全域快捷鍵
pub fn setup_global_shortcut(app: &AppHandle) -> tauri::Result<()> {
    register_shortcut(app)
}

/// 註冊快捷鍵
fn register_shortcut(app: &AppHandle) -> tauri::Result<()> {
    let shortcut = create_platform_shortcut();

    app.global_shortcut()
        .on_shortcut(shortcut, move |app, _shortcut_id, event| {
            // 只在按鍵釋放時觸發，避免持續按住的問題
            if event.state == ShortcutState::Released {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = toggle_window(&window);
                }
            }
        })
        .map_err(|e| tauri::Error::Anyhow(anyhow::anyhow!(e)))?;

    Ok(())
}

/// 根據平台創建快捷鍵
fn create_platform_shortcut() -> Shortcut {
    if cfg!(target_os = "macos") {
        Shortcut::new(Some(Modifiers::SUPER), Code::KeyL)
    } else {
        Shortcut::new(Some(Modifiers::CONTROL), Code::KeyL)
    }
}

/// 啟用或停用全域快捷鍵
#[command]
pub fn set_global_shortcut_enabled(app: AppHandle, enabled: bool) -> Result<(), String> {
    let shortcut = create_platform_shortcut();

    if enabled {
        // 先嘗試取消註冊（避免重複註冊）
        let _ = app.global_shortcut().unregister(shortcut);
        // 重新註冊
        register_shortcut(&app).map_err(|e| e.to_string())?;
    } else {
        // 取消註冊
        app.global_shortcut()
            .unregister(shortcut)
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}
