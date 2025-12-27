//! Settings management module
//!
//! Handles application settings including API key storage using tauri-plugin-store.

use std::fs;
use std::path::PathBuf;
use tauri::AppHandle;
use tauri::Manager;

const SETTINGS_FILE: &str = "settings.json";
const API_KEY_FIELD: &str = "gemini_api_key";

/// Get the settings file path
fn get_settings_path(app_handle: &AppHandle) -> Result<PathBuf, String> {
    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data dir: {}", e))?;

    Ok(app_data_dir.join(SETTINGS_FILE))
}

/// Read settings from file
fn read_settings(app_handle: &AppHandle) -> Result<serde_json::Value, String> {
    let path = get_settings_path(app_handle)?;

    if !path.exists() {
        return Ok(serde_json::json!({}));
    }

    let content =
        fs::read_to_string(&path).map_err(|e| format!("Failed to read settings: {}", e))?;

    serde_json::from_str(&content).map_err(|e| format!("Failed to parse settings: {}", e))
}

/// Write settings to file
fn write_settings(app_handle: &AppHandle, settings: &serde_json::Value) -> Result<(), String> {
    let path = get_settings_path(app_handle)?;

    // Ensure parent directory exists
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Failed to create settings dir: {}", e))?;
    }

    let content = serde_json::to_string_pretty(settings)
        .map_err(|e| format!("Failed to serialize settings: {}", e))?;

    fs::write(&path, content).map_err(|e| format!("Failed to write settings: {}", e))?;

    Ok(())
}

/// Get API key synchronously (for use in non-async contexts)
pub fn get_api_key_sync(app_handle: &AppHandle) -> Option<String> {
    let settings = read_settings(app_handle).ok()?;
    settings
        .get(API_KEY_FIELD)
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
}

/// Get the Gemini API key
#[tauri::command]
pub fn get_api_key(app_handle: AppHandle) -> Result<Option<String>, String> {
    let settings = read_settings(&app_handle)?;
    Ok(settings
        .get(API_KEY_FIELD)
        .and_then(|v| v.as_str())
        .map(|s| s.to_string()))
}

/// Set the Gemini API key
#[tauri::command]
pub fn set_api_key(app_handle: AppHandle, api_key: String) -> Result<(), String> {
    // Validate API key format (should start with "AIza")
    if !api_key.starts_with("AIza") {
        return Err("Invalid API key format. Gemini API keys should start with 'AIza'.".to_string());
    }

    let mut settings = read_settings(&app_handle)?;

    if let Some(obj) = settings.as_object_mut() {
        obj.insert(API_KEY_FIELD.to_string(), serde_json::json!(api_key));
    }

    write_settings(&app_handle, &settings)?;

    Ok(())
}

/// Check if API key is configured
#[tauri::command]
pub fn has_api_key(app_handle: AppHandle) -> Result<bool, String> {
    let settings = read_settings(&app_handle)?;
    Ok(settings
        .get(API_KEY_FIELD)
        .and_then(|v| v.as_str())
        .map(|s| !s.is_empty())
        .unwrap_or(false))
}

/// Clear the API key
#[tauri::command]
pub fn clear_api_key(app_handle: AppHandle) -> Result<(), String> {
    let mut settings = read_settings(&app_handle)?;

    if let Some(obj) = settings.as_object_mut() {
        obj.remove(API_KEY_FIELD);
    }

    write_settings(&app_handle, &settings)?;

    Ok(())
}
