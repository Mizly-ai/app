//! Window management utilities for the Search application.
//!
//! This module provides functions for creating, positioning, and managing
//! the main application window, including platform-specific behavior.

#[cfg(target_os = "macos")]
mod macos;

use tauri::{
    AppHandle, Emitter, Manager, PhysicalPosition, Result, WebviewUrl, WebviewWindow,
    WebviewWindowBuilder, WindowEvent,
};

#[cfg(target_os = "macos")]
use tauri::TitleBarStyle;

use crate::config::AppConfig;
use crate::state::AppState;

/// Delay before auto-hiding window on focus loss (in milliseconds).
///
/// This delay helps avoid IMK (Input Method Kit) mach port errors on macOS.
const AUTO_HIDE_DELAY_MS: u64 = 100;

/// Centers the window on the current monitor.
///
/// Calculates the center position based on the monitor's size and position,
/// then moves the window to that location.
///
/// # Errors
///
/// Returns an error if unable to get monitor information or set window position.
pub fn center_window(window: &WebviewWindow) -> Result<()> {
    let Some(monitor) = window.current_monitor()? else {
        return Ok(());
    };

    let Ok(outer_size) = window.outer_size() else {
        return Ok(());
    };

    let monitor_size = monitor.size();
    let monitor_position = monitor.position();

    let x = monitor_position.x + (monitor_size.width as i32 - outer_size.width as i32) / 2;
    let y = monitor_position.y + (monitor_size.height as i32 - outer_size.height as i32) / 2;

    window.set_position(PhysicalPosition::new(x, y))
}

/// Toggles the window visibility (show/hide).
///
/// When showing, the window is also focused and a `focus-search` event is emitted.
/// When hiding on macOS, the application is also hidden to return focus to the previous app.
///
/// # Arguments
///
/// * `window` - The window to toggle
/// * `show` - `true` to show the window, `false` to hide it
pub fn toggle_window_visibility(window: &WebviewWindow, show: bool) -> Result<()> {
    if show {
        window.show()?;
        window.set_focus()?;
        window.emit("focus-search", ())?;
    } else {
        window.hide()?;
        #[cfg(target_os = "macos")]
        macos::hide_application();
    }
    Ok(())
}

/// Toggles the window between visible and hidden states.
///
/// If the window is currently visible, it will be hidden. If hidden, it will be shown.
pub fn toggle_window(window: &WebviewWindow) -> Result<()> {
    let is_visible = window.is_visible()?;
    toggle_window_visibility(window, !is_visible)
}

/// Creates the main application window with the specified configuration.
///
/// The window is configured for production use:
/// - Fixed size, transparent, always on top, hidden from taskbar
///
/// # Arguments
///
/// * `app` - The Tauri application handle
/// * `config` - Application configuration containing window settings
pub fn create_main_window(app: &AppHandle, config: &AppConfig) -> Result<WebviewWindow> {
    let mut builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
        .title("Search")
        .inner_size(config.window_width, config.window_height)
        .center()
        .resizable(false)
        .always_on_top(true)
        .decorations(false)
        .transparent(true)
        .shadow(false)
        .skip_taskbar(true)
        .visible(false)
        .focused(true)
        .disable_drag_drop_handler();

    #[cfg(target_os = "macos")]
    {
        builder = builder
            .title_bar_style(TitleBarStyle::Overlay)
            .hidden_title(true);
    }

    builder.build()
}

/// Sets up screen change observer for automatic window repositioning.
///
/// On macOS, this monitors for display configuration changes (resolution changes,
/// external monitor connections, etc.) and re-centers the window accordingly.
/// On other platforms, this is a no-op.
pub fn setup_screen_change_observer(window: &WebviewWindow) {
    #[cfg(target_os = "macos")]
    macos::setup_screen_change_observer(window);

    #[cfg(not(target_os = "macos"))]
    let _ = window; // Silence unused variable warning
}

/// Sets up window event handlers.
///
/// Handles the following events:
/// - **CloseRequested**: Prevents actual close, hides window instead
/// - **Focused(false)**: Auto-hides window after a short delay (unless prevented)
/// - **Resized**: Re-centers the window
pub fn setup_window_events(window: &WebviewWindow) {
    let window_clone = window.clone();

    window.on_window_event(move |event| match event {
        WindowEvent::CloseRequested { api, .. } => {
            handle_close_requested(api, &window_clone);
        }
        WindowEvent::Focused(false) => {
            handle_focus_lost(&window_clone);
        }
        WindowEvent::Resized(_) => {
            handle_resized(&window_clone);
        }
        _ => {}
    });
}

/// Handles the close request by hiding the window instead of closing it.
fn handle_close_requested(api: &tauri::CloseRequestApi, window: &WebviewWindow) {
    api.prevent_close();

    if let Err(e) = toggle_window_visibility(window, false) {
        eprintln!("Failed to hide window: {}", e);
    }
}

/// Handles focus loss by scheduling an auto-hide after a delay.
///
/// The delay helps avoid IMK (Input Method Kit) mach port errors on macOS.
/// Auto-hide can be prevented by setting the `prevent_auto_hide` flag in AppState.
fn handle_focus_lost(window: &WebviewWindow) {
    let window = window.clone();

    tauri::async_runtime::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_millis(AUTO_HIDE_DELAY_MS)).await;

        if should_prevent_auto_hide(&window) {
            return;
        }

        if let Err(e) = toggle_window_visibility(&window, false) {
            eprintln!("Failed to hide window on focus lost: {}", e);
        }
    });
}

/// Checks if auto-hide should be prevented based on AppState.
fn should_prevent_auto_hide(window: &WebviewWindow) -> bool {
    window
        .app_handle()
        .try_state::<AppState>()
        .and_then(|state| state.prevent_auto_hide.lock().ok().map(|guard| *guard))
        .unwrap_or(false)
}

/// Handles window resize by re-centering the window.
fn handle_resized(window: &WebviewWindow) {
    if let Err(e) = center_window(window) {
        eprintln!("Failed to center window on resize: {}", e);
    }
}
