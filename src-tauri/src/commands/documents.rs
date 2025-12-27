//! Document-related Tauri commands

use std::sync::Arc;
use tauri::State;
use tauri_plugin_opener::OpenerExt;
use uuid::Uuid;

use crate::db::{Database, Document};
use crate::sync::SyncState;

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileInfo {
    pub name: String,
    pub path: String,
    pub content_type: Option<String>,
    pub size: Option<i64>,
    pub hash: Option<String>,
}

#[tauri::command]
pub async fn get_documents(
    db: State<'_, Arc<Database>>,
    store_id: String,
) -> Result<Vec<Document>, String> {
    db.get_documents_by_store(&store_id)
}

#[tauri::command]
pub async fn upload_documents(
    db: State<'_, Arc<Database>>,
    sync_state: State<'_, Arc<SyncState>>,
    store_id: String,
    files: Vec<FileInfo>,
) -> Result<Vec<Document>, String> {
    let mut documents = Vec::new();

    for file in files {
        let doc_id = Uuid::new_v4().to_string();

        // Create document in local database immediately (optimistic)
        // sync_status defaults to 'pending'
        let doc = db.create_document(
            &doc_id,
            &store_id,
            &file.name,
            &file.path,
            file.content_type.as_deref(),
            file.size,
            file.hash.as_deref(),
        )?;

        documents.push(doc);
    }

    // Notify background sync to pick up the new documents
    sync_state.notify();

    Ok(documents)
}

#[tauri::command]
pub async fn delete_document(
    db: State<'_, Arc<Database>>,
    sync_state: State<'_, Arc<SyncState>>,
    id: String,
) -> Result<(), String> {
    // Soft delete the document (sets deleted_at timestamp)
    // Background sync will handle the API deletion
    db.soft_delete_document(&id)?;

    // Notify background sync to process soft deleted documents
    sync_state.notify();

    Ok(())
}

#[tauri::command]
pub async fn get_documents_by_uids(
    db: State<'_, Arc<Database>>,
    document_uids: Vec<String>,
) -> Result<Vec<Document>, String> {
    // API returns file names (display names) in groundingMetadata, not gemini_names
    // Search by document name field instead
    db.get_documents_by_display_names(&document_uids)
}

#[tauri::command]
pub async fn open_document_file(
    app_handle: tauri::AppHandle,
    db: State<'_, Arc<Database>>,
    document_uid: String,
) -> Result<(), String> {
    // API returns file names (display names), search by name field
    let document = db
        .get_document_by_display_name(&document_uid)?
        .ok_or_else(|| format!("Document not found: {}", document_uid))?;

    app_handle
        .opener()
        .open_path(&document.path, None::<&str>)
        .map_err(|e| format!("Failed to open file: {}", e))
}
