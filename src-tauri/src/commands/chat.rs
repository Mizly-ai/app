//! Chat-related Tauri commands

use serde::Serialize;
use tauri::AppHandle;

use crate::gemini::GeminiClient;

/// Chat query result for frontend
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatResult {
    pub content: String,
    pub sources: Vec<String>,
}

/// Query stores with a question
#[tauri::command]
pub async fn query_stores(
    app_handle: AppHandle,
    store_names: Vec<String>,
    query: String,
) -> Result<ChatResult, String> {
    if store_names.is_empty() {
        return Err("No stores provided".to_string());
    }

    if query.trim().is_empty() {
        return Err("Query cannot be empty".to_string());
    }

    let client = GeminiClient::new(app_handle);
    let result = client.query_stores(&store_names, &query).await?;

    Ok(ChatResult {
        content: result.content,
        sources: result.sources,
    })
}

/// Get suggested questions based on store content
#[tauri::command]
pub async fn suggest_questions(
    app_handle: AppHandle,
    store_names: Vec<String>,
    locale: String,
) -> Result<Vec<String>, String> {
    if store_names.is_empty() {
        return Ok(vec![]);
    }

    let client = GeminiClient::new(app_handle);
    client.suggest_questions(&store_names, &locale).await
}
