//! Database operations for stores

use rusqlite::{params, Row};
use serde::{Deserialize, Serialize};

use super::Database;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Store {
    pub id: String,
    pub gemini_name: Option<String>,
    pub title: String,
    pub directory_path: Option<String>,
    pub sync_status: String,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
    pub active_documents_count: i64,
    pub pending_documents_count: i64,
    pub failed_documents_count: i64,
    pub size_bytes: i64,
    pub deleted_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StoreWithStatus {
    #[serde(flatten)]
    pub store: Store,
    pub status: String,
    pub document_count: i32,
    pub local_pending_count: i32,
    pub local_failed_count: i32,
}

/// Maps a database row to Store struct
/// Expected columns: id, gemini_name, title, directory_path, sync_status, create_time, update_time,
///                   active_documents_count, pending_documents_count, failed_documents_count, size_bytes,
///                   deleted_at, created_at, updated_at
fn map_row_to_store(row: &Row) -> rusqlite::Result<Store> {
    Ok(Store {
        id: row.get(0)?,
        gemini_name: row.get(1)?,
        title: row.get(2)?,
        directory_path: row.get(3)?,
        sync_status: row.get::<_, Option<String>>(4)?.unwrap_or_else(|| "pending".to_string()),
        create_time: row.get(5)?,
        update_time: row.get(6)?,
        active_documents_count: row.get::<_, Option<i64>>(7)?.unwrap_or(0),
        pending_documents_count: row.get::<_, Option<i64>>(8)?.unwrap_or(0),
        failed_documents_count: row.get::<_, Option<i64>>(9)?.unwrap_or(0),
        size_bytes: row.get::<_, Option<i64>>(10)?.unwrap_or(0),
        deleted_at: row.get(11)?,
        created_at: row.get(12)?,
        updated_at: row.get(13)?,
    })
}

impl Database {
    pub fn create_store(
        &self,
        id: &str,
        title: &str,
        directory_path: Option<&str>,
    ) -> Result<Store, String> {
        {
            let conn = self.conn.lock().map_err(|e| e.to_string())?;

            conn.execute(
                r#"
                INSERT INTO stores (id, title, directory_path)
                VALUES (?1, ?2, ?3)
                "#,
                params![id, title, directory_path],
            )
            .map_err(|e| format!("Failed to create store: {}", e))?;
        } // Release lock here

        self.get_store(id)
    }

    pub fn get_store(&self, id: &str) -> Result<Store, String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;

        conn.query_row(
            r#"
            SELECT id, gemini_name, title, directory_path, sync_status, create_time, update_time,
                   active_documents_count, pending_documents_count, failed_documents_count, size_bytes,
                   deleted_at, created_at, updated_at
            FROM stores
            WHERE id = ?1 AND deleted_at IS NULL
            "#,
            params![id],
            map_row_to_store,
        )
        .map_err(|e| format!("Failed to get store: {}", e))
    }

    pub fn get_stores(&self) -> Result<Vec<StoreWithStatus>, String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;

        let mut stmt = conn
            .prepare(
                r#"
                SELECT
                    s.id, s.gemini_name, s.title, s.directory_path, s.sync_status, s.create_time, s.update_time,
                    s.active_documents_count, s.pending_documents_count, s.failed_documents_count, s.size_bytes,
                    s.deleted_at, s.created_at, s.updated_at,
                    COUNT(d.id) as document_count,
                    SUM(CASE WHEN d.status NOT IN ('completed', 'failed') THEN 1 ELSE 0 END) as local_pending_count,
                    SUM(CASE WHEN d.status = 'failed' THEN 1 ELSE 0 END) as local_failed_count
                FROM stores s
                LEFT JOIN documents d ON d.store_id = s.id AND d.deleted_at IS NULL
                WHERE s.deleted_at IS NULL
                GROUP BY s.id
                ORDER BY s.created_at DESC
                "#,
            )
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let stores = stmt
            .query_map([], |row| {
                let local_pending_count: i32 = row.get(15)?;
                let status = if local_pending_count > 0 { "processing" } else { "completed" };

                Ok(StoreWithStatus {
                    store: map_row_to_store(row)?,
                    status: status.to_string(),
                    document_count: row.get(14)?,
                    local_pending_count,
                    local_failed_count: row.get(16)?,
                })
            })
            .map_err(|e| format!("Failed to query stores: {}", e))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("Failed to collect stores: {}", e))?;

        Ok(stores)
    }

    /// Soft delete a store by setting deleted_at timestamp
    pub fn soft_delete_store(&self, id: &str) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;

        conn.execute(
            r#"
            UPDATE stores
            SET deleted_at = CURRENT_TIMESTAMP, updated_at = CURRENT_TIMESTAMP
            WHERE id = ?1 AND deleted_at IS NULL
            "#,
            params![id],
        )
        .map_err(|e| format!("Failed to soft delete store: {}", e))?;

        Ok(())
    }

    /// Get all soft-deleted stores for background sync processing
    pub fn get_soft_deleted_stores(&self) -> Result<Vec<Store>, String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;

        let mut stmt = conn
            .prepare(
                r#"
                SELECT id, gemini_name, title, directory_path, sync_status, create_time, update_time,
                       active_documents_count, pending_documents_count, failed_documents_count, size_bytes,
                       deleted_at, created_at, updated_at
                FROM stores
                WHERE deleted_at IS NOT NULL
                ORDER BY deleted_at ASC
                "#,
            )
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let stores = stmt
            .query_map([], map_row_to_store)
            .map_err(|e| format!("Failed to query soft deleted stores: {}", e))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("Failed to collect soft deleted stores: {}", e))?;

        Ok(stores)
    }

    /// Hard delete a store (permanently remove from database)
    pub fn hard_delete_store(&self, id: &str) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;

        conn.execute("DELETE FROM stores WHERE id = ?1", params![id])
            .map_err(|e| format!("Failed to hard delete store: {}", e))?;

        Ok(())
    }

    pub fn update_store_sync_status(
        &self,
        id: &str,
        sync_status: &str,
        gemini_name: Option<&str>,
    ) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;

        conn.execute(
            r#"
            UPDATE stores
            SET sync_status = ?1,
                gemini_name = COALESCE(?2, gemini_name),
                updated_at = CURRENT_TIMESTAMP
            WHERE id = ?3
            "#,
            params![sync_status, gemini_name, id],
        )
        .map_err(|e| format!("Failed to update store sync status: {}", e))?;

        Ok(())
    }

    /// Update store with Gemini API response data
    pub fn update_store_from_gemini(
        &self,
        id: &str,
        gemini_name: &str,
        create_time: Option<&str>,
        update_time: Option<&str>,
        active_documents_count: i64,
        pending_documents_count: i64,
        failed_documents_count: i64,
        size_bytes: i64,
    ) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;

        conn.execute(
            r#"
            UPDATE stores
            SET gemini_name = ?1,
                create_time = ?2,
                update_time = ?3,
                active_documents_count = ?4,
                pending_documents_count = ?5,
                failed_documents_count = ?6,
                size_bytes = ?7,
                sync_status = 'synced',
                updated_at = CURRENT_TIMESTAMP
            WHERE id = ?8
            "#,
            params![
                gemini_name,
                create_time,
                update_time,
                active_documents_count,
                pending_documents_count,
                failed_documents_count,
                size_bytes,
                id
            ],
        )
        .map_err(|e| format!("Failed to update store from gemini: {}", e))?;

        Ok(())
    }

    pub fn get_pending_sync_stores(&self) -> Result<Vec<Store>, String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;

        let mut stmt = conn
            .prepare(
                r#"
                SELECT id, gemini_name, title, directory_path, sync_status, create_time, update_time,
                       active_documents_count, pending_documents_count, failed_documents_count, size_bytes,
                       deleted_at, created_at, updated_at
                FROM stores
                WHERE (sync_status = 'pending' OR sync_status IS NULL) AND deleted_at IS NULL
                "#,
            )
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let stores = stmt
            .query_map([], map_row_to_store)
            .map_err(|e| format!("Failed to query pending stores: {}", e))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("Failed to collect pending stores: {}", e))?;

        Ok(stores)
    }
}
