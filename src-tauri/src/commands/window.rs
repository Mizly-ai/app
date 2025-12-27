use tauri::{Result, State, WebviewWindow};

use crate::state::AppState;
use crate::window::toggle_window_visibility;

#[tauri::command]
pub fn open_directory(path: String) -> Result<()> {
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&path)
            .spawn()
            .map_err(|e| anyhow::anyhow!("Failed to open directory: {}", e))?;
    }
    #[cfg(not(target_os = "macos"))]
    {
        return Err(anyhow::anyhow!("Not implemented for this platform"));
    }
    Ok(())
}

#[tauri::command]
pub fn open_file(path: String) -> Result<()> {
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&path)
            .spawn()
            .map_err(|e| anyhow::anyhow!("Failed to open file: {}", e))?;
    }
    #[cfg(not(target_os = "macos"))]
    {
        return Err(anyhow::anyhow!("Not implemented for this platform"));
    }
    Ok(())
}

#[tauri::command]
pub fn show_window(window: WebviewWindow) -> Result<()> {
    toggle_window_visibility(&window, true)
}

#[tauri::command]
pub fn hide_window(window: WebviewWindow) -> Result<()> {
    toggle_window_visibility(&window, false)
}

#[tauri::command]
pub fn set_prevent_auto_hide(state: State<AppState>, prevent: bool) -> Result<()> {
    if let Ok(mut prevent_auto_hide) = state.prevent_auto_hide.lock() {
        *prevent_auto_hide = prevent;
    }
    Ok(())
}

#[tauri::command]
pub fn set_window_size(window: WebviewWindow, width: f64, height: f64) -> Result<()> {
    use tauri::{LogicalSize, PhysicalPosition};

    // Set size first
    window.set_size(LogicalSize::new(width, height))?;

    // Manually calculate center position for more reliable centering
    if let Ok(Some(monitor)) = window.current_monitor() {
        let monitor_size = monitor.size();
        let monitor_position = monitor.position();
        let scale_factor = monitor.scale_factor();

        // Convert logical size to physical
        let window_width = (width * scale_factor) as i32;
        let window_height = (height * scale_factor) as i32;

        let x = monitor_position.x + (monitor_size.width as i32 - window_width) / 2;
        let y = monitor_position.y + (monitor_size.height as i32 - window_height) / 2;

        window.set_position(PhysicalPosition::new(x, y))?;
    } else {
        // Fallback to built-in center
        window.center()?;
    }

    Ok(())
}

#[tauri::command]
pub fn set_window_movable(window: WebviewWindow, movable: bool) -> Result<()> {
    #[cfg(target_os = "macos")]
    {
        use objc2::runtime::AnyObject;
        use objc2::msg_send;

        if let Ok(ns_window) = window.ns_window() {
            unsafe {
                let ns_window = ns_window as *mut AnyObject;
                let _: () = msg_send![ns_window, setMovable: movable];
            }
        }
    }
    Ok(())
}

#[tauri::command]
pub fn set_always_on_top(window: WebviewWindow, always_on_top: bool) -> Result<()> {
    window.set_always_on_top(always_on_top)?;
    Ok(())
}
