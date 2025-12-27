//! Search Desktop Application
//!
//! A Tauri-based desktop application for document management and search.

// Module declarations
mod commands;
mod config;
mod db;
mod gemini;
mod polling;
mod settings;
mod shortcuts;
mod state;
mod sync;
mod tray;
mod window;

// Use modules
use std::sync::Arc;

use tauri::{Manager, RunEvent};

use commands::{
    create_store, delete_document, delete_store, get_documents, get_documents_by_uids, get_store,
    get_stores, hide_window, open_directory, open_document_file, open_file, query_stores,
    set_always_on_top, set_prevent_auto_hide, set_window_movable, set_window_size, show_window,
    suggest_questions, upload_documents,
};
use config::AppConfig;
use db::Database;
use gemini::GeminiClient;
use polling::{start_polling_task, PollingState};
use settings::{get_api_key, set_api_key, has_api_key, clear_api_key};
use shortcuts::{set_global_shortcut_enabled, setup_global_shortcut};
use state::AppState;
use sync::{start_sync_task, SyncState};
use tray::setup_system_tray;
use window::{create_main_window, setup_screen_change_observer, setup_window_events};

/// Application main entry point
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_process::init())
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            // Window commands
            show_window,
            hide_window,
            set_prevent_auto_hide,
            set_window_size,
            set_window_movable,
            set_always_on_top,
            open_directory,
            open_file,
            // Store commands
            get_stores,
            get_store,
            create_store,
            delete_store,
            // Document commands
            get_documents,
            get_documents_by_uids,
            upload_documents,
            delete_document,
            open_document_file,
            // Settings commands
            get_api_key,
            set_api_key,
            has_api_key,
            clear_api_key,
            // Chat commands
            query_stores,
            suggest_questions,
            // Shortcut commands
            set_global_shortcut_enabled,
        ])
        .setup(|app| {
            // Load configuration
            let config = AppConfig::default();

            // Initialize database
            let db = Arc::new(
                Database::new(app.handle()).expect("Failed to initialize database"),
            );
            app.manage(db.clone());

            // Initialize Gemini API client
            let gemini_client = Arc::new(GeminiClient::new(app.handle().clone()));
            app.manage(gemini_client.clone());

            // Initialize polling state (for document status polling)
            let polling_state = PollingState::new();
            app.manage(polling_state.clone());

            // Initialize sync state (for background sync)
            let sync_state = SyncState::new();
            app.manage(sync_state.clone());

            // Start background polling task (for checking document processing status)
            start_polling_task(
                app.handle().clone(),
                db.clone(),
                gemini_client.clone(),
                polling_state.clone(),
            );

            // Start background sync task (for uploading stores/documents)
            start_sync_task(app.handle().clone(), db, gemini_client, sync_state, polling_state);

            // Create main window
            let window = create_main_window(app.handle(), &config)?;

            // Setup system tray
            setup_system_tray(app.handle())?;

            // Setup global shortcuts and window events
            setup_global_shortcut(app.handle())?;
            setup_window_events(&window);

            // Setup screen change observer
            setup_screen_change_observer(&window);

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|app, event| {
            if let RunEvent::Reopen { .. } = event {
                // Handle clicking the Dock icon on macOS
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window::toggle_window(&window);
                }
            }
        });
}
