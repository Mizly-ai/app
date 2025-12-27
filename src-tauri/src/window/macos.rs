//! macOS-specific window utilities
//!
//! This module contains platform-specific implementations using the Objective-C runtime.

use objc2::rc::Retained;
use objc2::runtime::AnyObject;
use objc2::{class, msg_send, msg_send_id};
use tauri::{AppHandle, Manager, WebviewWindow};

use crate::window::center_window;

/// Hides the application on macOS, returning focus to the previous application.
///
/// Uses NSApplication's hide: method to properly hide the app
/// rather than just hiding the window.
pub fn hide_application() {
    unsafe {
        let app: Retained<AnyObject> = msg_send_id![class!(NSApplication), sharedApplication];
        let _: () = msg_send![&*app, hide: Option::<&AnyObject>::None];
    }
}

/// Sets up an observer for screen parameter changes (resolution, external monitors, etc.)
///
/// When screen parameters change, the main window will be automatically re-centered.
/// This handles scenarios like:
/// - Connecting/disconnecting external monitors
/// - Changing display resolution
/// - Rearranging displays
pub fn setup_screen_change_observer(window: &WebviewWindow) {
    let app_handle = window.app_handle().clone();

    unsafe {
        let notification_center: Retained<AnyObject> =
            msg_send_id![class!(NSNotificationCenter), defaultCenter];

        let notification_name: Retained<AnyObject> = msg_send_id![
            class!(NSString),
            stringWithUTF8String: c"NSApplicationDidChangeScreenParametersNotification".as_ptr()
        ];

        let block = block2::StackBlock::new(move |_notification: *const AnyObject| {
            handle_screen_change(&app_handle);
        });
        let block = block.copy();

        // Note: The observer is retained for the application's lifetime.
        // This is intentional as we want to monitor screen changes throughout.
        let _observer: Retained<AnyObject> = msg_send_id![
            &*notification_center,
            addObserverForName: &*notification_name
            object: Option::<&AnyObject>::None
            queue: Option::<&AnyObject>::None
            usingBlock: &*block
        ];
    }
}

/// Handles screen parameter changes by re-centering the main window.
fn handle_screen_change(app_handle: &AppHandle) {
    if let Some(window) = app_handle.get_webview_window("main") {
        if let Err(e) = center_window(&window) {
            eprintln!("Failed to center window on screen change: {}", e);
        }
    }
}
