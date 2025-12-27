//! Document status polling for Gemini operations

use serde::Serialize;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tauri::{AppHandle, Emitter};
use tokio::sync::Notify;
use tokio::time::sleep;

use crate::db::Database;
use crate::gemini::GeminiClient;

const POLLING_INTERVAL: Duration = Duration::from_secs(5);
const IDLE_INTERVAL: Duration = Duration::from_secs(30);

/// Map Gemini document state to internal status
fn map_gemini_state_to_status(state: Option<&str>) -> String {
    match state {
        Some("STATE_ACTIVE") | Some("ACTIVE") => "completed".to_string(),
        Some("STATE_PENDING") | Some("PENDING") => "processing".to_string(),
        Some("STATE_FAILED") | Some("FAILED") => "failed".to_string(),
        Some(s) if s.to_lowercase().contains("active") => "completed".to_string(),
        Some(s) if s.to_lowercase().contains("pending") => "processing".to_string(),
        Some(s) if s.to_lowercase().contains("fail") => "failed".to_string(),
        _ => "completed".to_string(),
    }
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentStatusPayload {
    pub document_id: String,
    pub gemini_name: Option<String>,
    pub store_id: String,
    pub status: String,
}

pub struct PollingState {
    notify: Notify,
    running: AtomicBool,
}

impl PollingState {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            notify: Notify::new(),
            running: AtomicBool::new(false),
        })
    }

    /// Notify the polling task to wake up immediately
    pub fn notify(&self) {
        self.notify.notify_one();
    }

    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    pub fn set_running(&self, running: bool) {
        self.running.store(running, Ordering::SeqCst);
    }
}

pub fn start_polling_task(
    app_handle: AppHandle,
    db: Arc<Database>,
    gemini_client: Arc<GeminiClient>,
    polling_state: Arc<PollingState>,
) {
    if polling_state.is_running() {
        return;
    }

    polling_state.set_running(true);

    tauri::async_runtime::spawn(async move {
        loop {
            // Get documents with pending operations
            let pending_docs = match db.get_documents_with_pending_operations() {
                Ok(docs) => docs,
                Err(e) => {
                    eprintln!("Failed to get documents with pending operations: {}", e);
                    sleep(POLLING_INTERVAL).await;
                    continue;
                }
            };

            if pending_docs.is_empty() {
                // No pending documents, wait longer or until notified
                tokio::select! {
                    _ = sleep(IDLE_INTERVAL) => {}
                    _ = polling_state.notify.notified() => {}
                }
                continue;
            }

            // Check operation status for each pending document
            for doc in &pending_docs {
                let operation_name = match &doc.operation_name {
                    Some(name) => name.clone(),
                    None => continue,
                };

                match gemini_client.get_operation(&operation_name).await {
                    Ok(operation) => {
                        if operation.done {
                            // Operation completed
                            if let Some(error) = operation.error {
                                // Operation failed
                                let _ = db.update_document_error(
                                    &doc.id,
                                    "failed",
                                    &error.message,
                                );

                                let payload = DocumentStatusPayload {
                                    document_id: doc.id.clone(),
                                    gemini_name: None,
                                    store_id: doc.store_id.clone(),
                                    status: "failed".to_string(),
                                };
                                let _ = app_handle.emit("document-status-updated", &payload);
                            } else if let Some(response) = operation.response {
                                // Operation succeeded - get document info
                                if let Some(document_name) = response.document_name {
                                    // Get the document details from Gemini
                                    match gemini_client.get_document(&document_name).await {
                                        Ok(gemini_doc) => {
                                            let status = map_gemini_state_to_status(gemini_doc.state.as_deref());

                                            let _ = db.update_document_from_gemini(
                                                &doc.id,
                                                &gemini_doc.name,
                                                gemini_doc.mime_type.as_deref(),
                                                &status,
                                            );

                                            let payload = DocumentStatusPayload {
                                                document_id: doc.id.clone(),
                                                gemini_name: Some(gemini_doc.name.clone()),
                                                store_id: doc.store_id.clone(),
                                                status: status.clone(),
                                            };
                                            let _ =
                                                app_handle.emit("document-status-updated", &payload);
                                        }
                                        Err(e) => {
                                            eprintln!(
                                                "Failed to get document info for {}: {}",
                                                document_name, e
                                            );
                                        }
                                    }
                                }
                            }
                        }
                        // If not done, continue polling
                    }
                    Err(e) => {
                        eprintln!(
                            "Failed to get operation status for {}: {}",
                            operation_name, e
                        );
                    }
                }
            }

            // Wait before next poll cycle
            tokio::select! {
                _ = sleep(POLLING_INTERVAL) => {}
                _ = polling_state.notify.notified() => {}
            }
        }
    });
}
