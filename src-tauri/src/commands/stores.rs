use std::sync::Arc;
use tauri::State;
use uuid::Uuid;

use crate::db::{Store, StoreWithStatus, Database};
use crate::sync::SyncState;

#[tauri::command]
pub async fn get_stores(
    db: State<'_, Arc<Database>>,
) -> Result<Vec<StoreWithStatus>, String> {
    db.get_stores()
}

#[tauri::command]
pub async fn get_store(
    db: State<'_, Arc<Database>>,
    id: String,
) -> Result<Store, String> {
    db.get_store(&id)
}

#[tauri::command]
pub async fn create_store(
    db: State<'_, Arc<Database>>,
    sync_state: State<'_, Arc<SyncState>>,
    title: String,
    directory_path: Option<String>,
) -> Result<Store, String> {
    let id = Uuid::new_v4().to_string();

    // Create store in local database immediately (optimistic)
    // sync_status defaults to 'pending'
    let store = db.create_store(&id, &title, directory_path.as_deref())?;

    // Notify background sync to pick up the new store
    sync_state.notify();

    Ok(store)
}

#[tauri::command]
pub async fn delete_store(
    db: State<'_, Arc<Database>>,
    sync_state: State<'_, Arc<SyncState>>,
    id: String,
) -> Result<(), String> {
    // Soft delete all documents in this store first
    db.soft_delete_documents_by_store(&id)?;

    // Soft delete the store (sets deleted_at timestamp)
    // Background sync will handle the API deletion
    db.soft_delete_store(&id)?;

    // Notify background sync to process soft deleted items
    sync_state.notify();

    Ok(())
}
