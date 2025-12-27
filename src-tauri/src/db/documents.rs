//! Database operations for documents

use rusqlite::{params, OptionalExtension, Row};
use serde::{Deserialize, Serialize};

use super::Database;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    pub id: String,
    pub store_id: String,
    pub gemini_name: Option<String>,
    pub operation_name: Option<String>,
    pub name: String,
    pub path: String,
    pub content_type: Option<String>,
    pub mime_type: Option<String>,
    pub size: Option<i64>,
    pub hash: Option<String>,
    pub status: String,
    pub sync_status: String,
    pub error_message: Option<String>,
    pub deleted_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PendingDocument {
    pub id: String,
    pub store_id: String,
    pub gemini_name: String,
    pub store_gemini_name: String,
}

/// Struct for soft-deleted documents with their associated store gemini_name
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SoftDeletedDocument {
    pub id: String,
    pub gemini_name: String,
    pub store_gemini_name: String,
}

/// Maps a database row to Document struct
/// Expected columns: id, store_id, gemini_name, operation_name, name, path, content_type, mime_type,
///                   size, hash, status, sync_status, error_message, deleted_at, created_at, updated_at
fn map_row_to_document(row: &Row) -> rusqlite::Result<Document> {
    Ok(Document {
        id: row.get(0)?,
        store_id: row.get(1)?,
        gemini_name: row.get(2)?,
        operation_name: row.get(3)?,
        name: row.get(4)?,
        path: row.get(5)?,
        content_type: row.get(6)?,
        mime_type: row.get(7)?,
        size: row.get(8)?,
        hash: row.get(9)?,
        status: row.get::<_, Option<String>>(10)?.unwrap_or_else(|| "pending".to_string()),
        sync_status: row.get::<_, Option<String>>(11)?.unwrap_or_else(|| "pending".to_string()),
        error_message: row.get(12)?,
        deleted_at: row.get(13)?,
        created_at: row.get(14)?,
        updated_at: row.get(15)?,
    })
}

impl Database {
    pub fn create_document(
        &self,
        id: &str,
        store_id: &str,
        name: &str,
        path: &str,
        content_type: Option<&str>,
        size: Option<i64>,
        hash: Option<&str>,
    ) -> Result<Document, String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;

        conn.execute(
            r#"
            INSERT INTO documents (id, store_id, name, path, content_type, size, hash, status)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 'pending')
            "#,
            params![id, store_id, name, path, content_type, size, hash],
        )
        .map_err(|e| format!("Failed to create document: {}", e))?;

        drop(conn);
        self.get_document(id)
    }

    pub fn get_document(&self, id: &str) -> Result<Document, String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;

        conn.query_row(
            r#"
            SELECT id, store_id, gemini_name, operation_name, name, path, content_type, mime_type,
                   size, hash, status, sync_status, error_message, deleted_at, created_at, updated_at
            FROM documents
            WHERE id = ?1 AND deleted_at IS NULL
            "#,
            params![id],
            map_row_to_document,
        )
        .map_err(|e| format!("Failed to get document: {}", e))
    }

    pub fn get_documents_by_store(&self, store_id: &str) -> Result<Vec<Document>, String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;

        let mut stmt = conn
            .prepare(
                r#"
                SELECT id, store_id, gemini_name, operation_name, name, path, content_type, mime_type,
                       size, hash, status, sync_status, error_message, deleted_at, created_at, updated_at
                FROM documents
                WHERE store_id = ?1 AND deleted_at IS NULL
                ORDER BY created_at ASC
                "#,
            )
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let documents = stmt
            .query_map(params![store_id], map_row_to_document)
            .map_err(|e| format!("Failed to query documents: {}", e))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("Failed to collect documents: {}", e))?;

        Ok(documents)
    }

    /// Update document with Gemini operation info after upload initiation
    pub fn update_document_operation(
        &self,
        id: &str,
        operation_name: &str,
    ) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;

        conn.execute(
            r#"
            UPDATE documents
            SET operation_name = ?1, status = 'processing', sync_status = 'synced', updated_at = CURRENT_TIMESTAMP
            WHERE id = ?2
            "#,
            params![operation_name, id],
        )
        .map_err(|e| format!("Failed to update document operation: {}", e))?;

        Ok(())
    }

    /// Update document with Gemini document info after upload completion
    pub fn update_document_from_gemini(
        &self,
        id: &str,
        gemini_name: &str,
        mime_type: Option<&str>,
        status: &str,
    ) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;

        conn.execute(
            r#"
            UPDATE documents
            SET gemini_name = ?1, mime_type = ?2, status = ?3, operation_name = NULL, updated_at = CURRENT_TIMESTAMP
            WHERE id = ?4
            "#,
            params![gemini_name, mime_type, status, id],
        )
        .map_err(|e| format!("Failed to update document from gemini: {}", e))?;

        Ok(())
    }

    #[allow(dead_code)]
    pub fn update_document_status(&self, id: &str, status: &str) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;

        conn.execute(
            r#"
            UPDATE documents
            SET status = ?1, updated_at = CURRENT_TIMESTAMP
            WHERE id = ?2
            "#,
            params![status, id],
        )
        .map_err(|e| format!("Failed to update document status: {}", e))?;

        Ok(())
    }

    /// Update document status with error message
    pub fn update_document_error(
        &self,
        id: &str,
        status: &str,
        error_message: &str,
    ) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;

        conn.execute(
            r#"
            UPDATE documents
            SET status = ?1, error_message = ?2, updated_at = CURRENT_TIMESTAMP
            WHERE id = ?3
            "#,
            params![status, error_message, id],
        )
        .map_err(|e| format!("Failed to update document error: {}", e))?;

        Ok(())
    }

    /// Get documents with pending operations (for polling)
    pub fn get_documents_with_pending_operations(&self) -> Result<Vec<Document>, String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;

        let mut stmt = conn
            .prepare(
                r#"
                SELECT id, store_id, gemini_name, operation_name, name, path, content_type, mime_type,
                       size, hash, status, sync_status, error_message, deleted_at, created_at, updated_at
                FROM documents
                WHERE operation_name IS NOT NULL
                AND status NOT IN ('completed', 'failed')
                AND deleted_at IS NULL
                "#,
            )
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let documents = stmt
            .query_map([], map_row_to_document)
            .map_err(|e| format!("Failed to query documents with pending operations: {}", e))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("Failed to collect documents: {}", e))?;

        Ok(documents)
    }

    /// Soft delete a document by setting deleted_at timestamp
    pub fn soft_delete_document(&self, id: &str) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;

        conn.execute(
            r#"
            UPDATE documents
            SET deleted_at = CURRENT_TIMESTAMP, updated_at = CURRENT_TIMESTAMP
            WHERE id = ?1 AND deleted_at IS NULL
            "#,
            params![id],
        )
        .map_err(|e| format!("Failed to soft delete document: {}", e))?;

        Ok(())
    }

    /// Soft delete all documents in a store
    pub fn soft_delete_documents_by_store(&self, store_id: &str) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;

        conn.execute(
            r#"
            UPDATE documents
            SET deleted_at = CURRENT_TIMESTAMP, updated_at = CURRENT_TIMESTAMP
            WHERE store_id = ?1 AND deleted_at IS NULL
            "#,
            params![store_id],
        )
        .map_err(|e| format!("Failed to soft delete documents by store: {}", e))?;

        Ok(())
    }

    /// Get all soft-deleted documents for background sync processing
    pub fn get_soft_deleted_documents(&self) -> Result<Vec<SoftDeletedDocument>, String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;

        let mut stmt = conn
            .prepare(
                r#"
                SELECT d.id, d.gemini_name, s.gemini_name
                FROM documents d
                JOIN stores s ON s.id = d.store_id
                WHERE d.deleted_at IS NOT NULL
                AND d.gemini_name IS NOT NULL
                AND s.gemini_name IS NOT NULL
                ORDER BY d.deleted_at ASC
                "#,
            )
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let documents = stmt
            .query_map([], |row| {
                Ok(SoftDeletedDocument {
                    id: row.get(0)?,
                    gemini_name: row.get(1)?,
                    store_gemini_name: row.get(2)?,
                })
            })
            .map_err(|e| format!("Failed to query soft deleted documents: {}", e))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("Failed to collect soft deleted documents: {}", e))?;

        Ok(documents)
    }

    /// Hard delete a document (permanently remove from database)
    pub fn hard_delete_document(&self, id: &str) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;

        conn.execute("DELETE FROM documents WHERE id = ?1", params![id])
            .map_err(|e| format!("Failed to hard delete document: {}", e))?;

        Ok(())
    }

    pub fn update_document_sync_status(&self, id: &str, sync_status: &str) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;

        conn.execute(
            r#"
            UPDATE documents
            SET sync_status = ?1, updated_at = CURRENT_TIMESTAMP
            WHERE id = ?2
            "#,
            params![sync_status, id],
        )
        .map_err(|e| format!("Failed to update document sync status: {}", e))?;

        Ok(())
    }

    /// Get a document by its display name (file name)
    pub fn get_document_by_display_name(&self, name: &str) -> Result<Option<Document>, String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;

        conn.query_row(
            r#"
            SELECT id, store_id, gemini_name, operation_name, name, path, content_type, mime_type,
                   size, hash, status, sync_status, error_message, deleted_at, created_at, updated_at
            FROM documents
            WHERE name = ?1 AND deleted_at IS NULL
            "#,
            params![name],
            map_row_to_document,
        )
        .optional()
        .map_err(|e| format!("Failed to get document by display name: {}", e))
    }

    /// Get documents by their display names (file names from API groundingMetadata)
    pub fn get_documents_by_display_names(&self, names: &[String]) -> Result<Vec<Document>, String> {
        if names.is_empty() {
            return Ok(Vec::new());
        }

        let conn = self.conn.lock().map_err(|e| e.to_string())?;

        let placeholders: Vec<String> = names.iter().enumerate().map(|(i, _)| format!("?{}", i + 1)).collect();
        let query = format!(
            r#"
            SELECT id, store_id, gemini_name, operation_name, name, path, content_type, mime_type,
                   size, hash, status, sync_status, error_message, deleted_at, created_at, updated_at
            FROM documents
            WHERE name IN ({}) AND deleted_at IS NULL
            "#,
            placeholders.join(", ")
        );

        let mut stmt = conn
            .prepare(&query)
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let params: Vec<&dyn rusqlite::ToSql> = names.iter().map(|s| s as &dyn rusqlite::ToSql).collect();

        let documents = stmt
            .query_map(params.as_slice(), map_row_to_document)
            .map_err(|e| format!("Failed to query documents by display names: {}", e))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("Failed to collect documents: {}", e))?;

        Ok(documents)
    }

    pub fn get_pending_upload_documents(&self) -> Result<Vec<Document>, String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;

        let mut stmt = conn
            .prepare(
                r#"
                SELECT d.id, d.store_id, d.gemini_name, d.operation_name, d.name, d.path, d.content_type, d.mime_type,
                       d.size, d.hash, d.status, d.sync_status, d.error_message, d.deleted_at, d.created_at, d.updated_at
                FROM documents d
                JOIN stores s ON s.id = d.store_id
                WHERE d.sync_status = 'pending'
                AND s.gemini_name IS NOT NULL
                AND s.sync_status = 'synced'
                AND d.deleted_at IS NULL
                AND s.deleted_at IS NULL
                "#,
            )
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let documents = stmt
            .query_map([], map_row_to_document)
            .map_err(|e| format!("Failed to query pending upload documents: {}", e))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("Failed to collect pending upload documents: {}", e))?;

        Ok(documents)
    }
}
