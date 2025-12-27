//! Background synchronization task for uploading stores and documents to Gemini

use serde::Serialize;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tauri::AppHandle;
use tauri::Emitter;
use tokio::sync::Notify;
use tokio::time::sleep;

use crate::db::Database;
use crate::gemini::GeminiClient;
use crate::polling::PollingState;

/// Interval between sync cycles when there's work to do
const SYNC_INTERVAL: Duration = Duration::from_secs(2);

/// Interval between sync cycles when idle
const IDLE_INTERVAL: Duration = Duration::from_secs(30);

// ============================================================================
// Payload Types
// ============================================================================

/// Payload for store sync events
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StoreSyncPayload {
    pub store_id: String,
    pub sync_status: String,
    pub gemini_name: Option<String>,
}

/// Payload for document sync events
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentSyncPayload {
    pub document_id: String,
    pub store_id: String,
    pub sync_status: String,
    pub gemini_name: Option<String>,
    pub operation_name: Option<String>,
    pub status: String,
}

// ============================================================================
// Sync State
// ============================================================================

/// State for background sync task
pub struct SyncState {
    notify: Notify,
    running: AtomicBool,
}

impl SyncState {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            notify: Notify::new(),
            running: AtomicBool::new(false),
        })
    }

    /// Notify the sync task to wake up
    pub fn notify(&self) {
        self.notify.notify_one();
    }

    /// Check if sync task is running
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    /// Set running state
    pub fn set_running(&self, running: bool) {
        self.running.store(running, Ordering::SeqCst);
    }
}

// ============================================================================
// Sync Operations
// ============================================================================

/// Sync pending stores (create stores in Gemini)
async fn sync_pending_stores(
    app_handle: &AppHandle,
    db: &Database,
    gemini_client: &Arc<GeminiClient>,
) -> bool {
    let stores = match db.get_pending_sync_stores() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to get pending stores: {}", e);
            return false;
        }
    };

    if stores.is_empty() {
        return false;
    }

    for store in stores {
        match gemini_client.create_store(&store.title).await {
            Ok(response) => {
                // Update store with gemini_name and mark as synced
                if let Err(e) = db.update_store_from_gemini(
                    &store.id,
                    &response.name,
                    response.create_time.as_deref(),
                    response.update_time.as_deref(),
                    response.active_documents_count,
                    response.pending_documents_count,
                    response.failed_documents_count,
                    response.size_bytes,
                ) {
                    eprintln!("Failed to update store from gemini: {}", e);
                    continue;
                }

                // Emit success event to frontend
                let payload = StoreSyncPayload {
                    store_id: store.id.clone(),
                    sync_status: "synced".to_string(),
                    gemini_name: Some(response.name),
                };

                if let Err(e) = app_handle.emit("store-sync-updated", &payload) {
                    eprintln!("Failed to emit store sync event: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Failed to create store for store {}: {}", store.id, e);

                // Mark as failed
                let _ = db.update_store_sync_status(&store.id, "failed", None);

                // Emit failure event
                let payload = StoreSyncPayload {
                    store_id: store.id.clone(),
                    sync_status: "failed".to_string(),
                    gemini_name: None,
                };
                let _ = app_handle.emit("store-sync-updated", &payload);
            }
        }
    }

    true
}

/// Upload pending documents to Gemini
async fn sync_pending_documents(
    app_handle: &AppHandle,
    db: &Database,
    gemini_client: &Arc<GeminiClient>,
    polling_state: &Arc<PollingState>,
) -> bool {
    let documents = match db.get_pending_upload_documents() {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Failed to get pending upload documents: {}", e);
            return false;
        }
    };

    if documents.is_empty() {
        return false;
    }

    for doc in documents {
        // Get store to get gemini_name
        let store = match db.get_store(&doc.store_id) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to get store for document: {}", e);
                continue;
            }
        };

        let store_gemini_name = match store.gemini_name {
            Some(name) => name,
            None => continue, // Store not synced yet
        };

        // Upload document using resumable upload
        match gemini_client
            .upload_document(&store_gemini_name, &doc.path, Some(&doc.name))
            .await
        {
            Ok(operation) => {
                // Update document with operation info
                if let Err(e) = db.update_document_operation(&doc.id, &operation.name) {
                    eprintln!("Failed to update document operation: {}", e);
                    continue;
                }

                // Emit success event
                let payload = DocumentSyncPayload {
                    document_id: doc.id.clone(),
                    store_id: doc.store_id.clone(),
                    sync_status: "synced".to_string(),
                    gemini_name: None, // Will be set after operation completes
                    operation_name: Some(operation.name.clone()),
                    status: "processing".to_string(),
                };

                if let Err(e) = app_handle.emit("document-sync-updated", &payload) {
                    eprintln!("Failed to emit document sync event: {}", e);
                }

                // Notify polling task to check document status
                polling_state.notify();
            }
            Err(e) => {
                eprintln!("Failed to upload document {}: {}", doc.id, e);

                // Mark as failed
                let _ = db.update_document_sync_status(&doc.id, "failed");
                let _ = db.update_document_error(&doc.id, "failed", &e);

                // Emit failure event
                let payload = DocumentSyncPayload {
                    document_id: doc.id.clone(),
                    store_id: doc.store_id.clone(),
                    sync_status: "failed".to_string(),
                    gemini_name: None,
                    operation_name: None,
                    status: "failed".to_string(),
                };
                let _ = app_handle.emit("document-sync-updated", &payload);
            }
        }
    }

    true
}

/// Process soft-deleted stores (delete from Gemini then hard delete locally)
async fn process_pending_store_deletions(db: &Database, gemini_client: &Arc<GeminiClient>) -> bool {
    let stores = match db.get_soft_deleted_stores() {
        Ok(s) => s,
        Err(_) => return false,
    };

    if stores.is_empty() {
        return false;
    }

    for store in stores {
        // Only call Gemini API if store has a gemini_name
        if let Some(ref gemini_name) = store.gemini_name {
            match gemini_client.delete_store(gemini_name, true).await {
                Ok(()) => {
                    // API deletion successful, hard delete locally
                    let _ = db.hard_delete_store(&store.id);
                }
                Err(e) => {
                    // Check if it's a 404 (already deleted)
                    if e.contains("404") || e.contains("NOT_FOUND") {
                        let _ = db.hard_delete_store(&store.id);
                    }
                    // Otherwise keep the soft-deleted record for retry
                }
            }
        } else {
            // No gemini_name means it was never synced, just hard delete
            let _ = db.hard_delete_store(&store.id);
        }
    }

    true
}

/// Process soft-deleted documents (delete from Gemini then hard delete locally)
async fn process_pending_document_deletions(
    db: &Database,
    gemini_client: &Arc<GeminiClient>,
) -> bool {
    let documents = match db.get_soft_deleted_documents() {
        Ok(d) => d,
        Err(_) => return false,
    };

    if documents.is_empty() {
        return false;
    }

    for doc in documents {
        match gemini_client.delete_document(&doc.gemini_name).await {
            Ok(()) => {
                // API deletion successful, hard delete locally
                let _ = db.hard_delete_document(&doc.id);
            }
            Err(e) => {
                // Check if it's a 404 (already deleted)
                if e.contains("404") || e.contains("NOT_FOUND") {
                    let _ = db.hard_delete_document(&doc.id);
                }
                // Otherwise keep the soft-deleted record for retry
            }
        }
    }

    true
}

// ============================================================================
// Main Sync Task
// ============================================================================

/// Start the background sync task
pub fn start_sync_task(
    app_handle: AppHandle,
    db: Arc<Database>,
    gemini_client: Arc<GeminiClient>,
    sync_state: Arc<SyncState>,
    polling_state: Arc<PollingState>,
) {
    if sync_state.is_running() {
        return;
    }

    sync_state.set_running(true);

    tauri::async_runtime::spawn(async move {
        loop {
            // Run all sync operations
            let mut has_work = false;

            // Step 1: Sync pending stores
            has_work |= sync_pending_stores(&app_handle, &db, &gemini_client).await;

            // Step 2: Upload pending documents
            has_work |= sync_pending_documents(&app_handle, &db, &gemini_client, &polling_state).await;

            // Step 3: Process soft-deleted stores
            has_work |= process_pending_store_deletions(&db, &gemini_client).await;

            // Step 4: Process soft-deleted documents
            has_work |= process_pending_document_deletions(&db, &gemini_client).await;

            // Wait before next sync cycle
            let interval = if has_work {
                SYNC_INTERVAL
            } else {
                IDLE_INTERVAL
            };

            tokio::select! {
                _ = sleep(interval) => {}
                _ = sync_state.notify.notified() => {}
            }
        }
    });
}
