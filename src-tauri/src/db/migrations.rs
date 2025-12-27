//! Database migrations for the application
//!
//! This module handles database schema migrations.
//! For existing databases, we add missing columns before creating indexes.

use rusqlite::Connection;

/// Check if a column exists in a table
fn column_exists(conn: &Connection, table: &str, column: &str) -> bool {
    let sql = format!("PRAGMA table_info({})", table);
    let mut stmt = match conn.prepare(&sql) {
        Ok(s) => s,
        Err(_) => return false,
    };

    let result = stmt.query_map([], |row| {
        let name: String = row.get(1)?;
        Ok(name)
    });

    match result {
        Ok(rows) => rows.filter_map(|r| r.ok()).any(|name| name == column),
        Err(_) => false,
    }
}

/// Add a column if it doesn't exist
fn add_column_if_not_exists(
    conn: &Connection,
    table: &str,
    column: &str,
    definition: &str,
) -> Result<(), String> {
    if !column_exists(conn, table, column) {
        let sql = format!("ALTER TABLE {} ADD COLUMN {} {}", table, column, definition);
        conn.execute(&sql, [])
            .map_err(|e| format!("Failed to add column {} to {}: {}", column, table, e))?;
    }
    Ok(())
}

const MIGRATIONS: &[&str] = &[
    // Migration 1: Create stores table
    r#"
    CREATE TABLE IF NOT EXISTS stores (
        id TEXT PRIMARY KEY,
        gemini_name TEXT UNIQUE,
        title TEXT NOT NULL,
        directory_path TEXT,
        sync_status TEXT DEFAULT 'pending',
        create_time TEXT,
        update_time TEXT,
        active_documents_count INTEGER DEFAULT 0,
        pending_documents_count INTEGER DEFAULT 0,
        failed_documents_count INTEGER DEFAULT 0,
        size_bytes INTEGER DEFAULT 0,
        deleted_at DATETIME DEFAULT NULL,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
        updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
    );
    "#,
    // Migration 2: Create documents table
    r#"
    CREATE TABLE IF NOT EXISTS documents (
        id TEXT PRIMARY KEY,
        store_id TEXT NOT NULL,
        gemini_name TEXT UNIQUE,
        operation_name TEXT,
        name TEXT NOT NULL,
        path TEXT NOT NULL,
        content_type TEXT,
        mime_type TEXT,
        size INTEGER,
        hash TEXT,
        status TEXT DEFAULT 'pending',
        sync_status TEXT DEFAULT 'pending',
        error_message TEXT,
        deleted_at DATETIME DEFAULT NULL,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
        updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
        FOREIGN KEY (store_id) REFERENCES stores(id) ON DELETE CASCADE
    );
    "#,
    // Migration 3: Create indexes for documents
    r#"
    CREATE INDEX IF NOT EXISTS idx_documents_status ON documents(status);
    "#,
    r#"
    CREATE INDEX IF NOT EXISTS idx_documents_store ON documents(store_id);
    "#,
    // Migration 5: Create index for soft delete queries
    r#"
    CREATE INDEX IF NOT EXISTS idx_documents_deleted ON documents(deleted_at);
    "#,
    r#"
    CREATE INDEX IF NOT EXISTS idx_stores_deleted ON stores(deleted_at);
    "#,
    // Migration 7: Create index for operation polling (column added in ensure_schema)
    r#"
    CREATE INDEX IF NOT EXISTS idx_documents_operation ON documents(operation_name);
    "#,
    // Migration 8: Create index for gemini_name lookups
    r#"
    CREATE INDEX IF NOT EXISTS idx_stores_gemini_name ON stores(gemini_name);
    "#,
    r#"
    CREATE INDEX IF NOT EXISTS idx_documents_gemini_name ON documents(gemini_name);
    "#,
];

/// Ensure all required columns exist (for upgrading old databases)
/// Note: SQLite doesn't allow adding UNIQUE columns via ALTER TABLE,
/// so we add them without UNIQUE constraint for existing databases
fn ensure_schema(conn: &Connection) -> Result<(), String> {
    // Add missing columns to stores table
    // Note: gemini_name is UNIQUE in new schema, but we can't add UNIQUE via ALTER TABLE
    add_column_if_not_exists(conn, "stores", "gemini_name", "TEXT")?;
    add_column_if_not_exists(conn, "stores", "sync_status", "TEXT DEFAULT 'pending'")?;
    add_column_if_not_exists(conn, "stores", "create_time", "TEXT")?;
    add_column_if_not_exists(conn, "stores", "update_time", "TEXT")?;
    add_column_if_not_exists(conn, "stores", "active_documents_count", "INTEGER DEFAULT 0")?;
    add_column_if_not_exists(conn, "stores", "pending_documents_count", "INTEGER DEFAULT 0")?;
    add_column_if_not_exists(conn, "stores", "failed_documents_count", "INTEGER DEFAULT 0")?;
    add_column_if_not_exists(conn, "stores", "size_bytes", "INTEGER DEFAULT 0")?;
    add_column_if_not_exists(conn, "stores", "deleted_at", "DATETIME DEFAULT NULL")?;

    // Add missing columns to documents table
    add_column_if_not_exists(conn, "documents", "gemini_name", "TEXT")?;
    add_column_if_not_exists(conn, "documents", "operation_name", "TEXT")?;
    add_column_if_not_exists(conn, "documents", "mime_type", "TEXT")?;
    add_column_if_not_exists(conn, "documents", "sync_status", "TEXT DEFAULT 'pending'")?;
    add_column_if_not_exists(conn, "documents", "error_message", "TEXT")?;
    add_column_if_not_exists(conn, "documents", "deleted_at", "DATETIME DEFAULT NULL")?;

    Ok(())
}

pub fn run_migrations(conn: &Connection) -> Result<(), String> {
    // Enable foreign key support
    conn.execute("PRAGMA foreign_keys = ON;", [])
        .map_err(|e| format!("Failed to enable foreign keys: {}", e))?;

    // Create migrations table if not exists
    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS schema_migrations (
            version INTEGER PRIMARY KEY,
            applied_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );
        "#,
        [],
    )
    .map_err(|e| format!("Failed to create migrations table: {}", e))?;

    // Ensure schema has all required columns (for upgrading old databases)
    ensure_schema(conn)?;

    // Get current version
    let current_version: i32 = conn
        .query_row(
            "SELECT COALESCE(MAX(version), 0) FROM schema_migrations",
            [],
            |row| row.get(0),
        )
        .unwrap_or(0);

    // Run pending migrations
    for (index, migration) in MIGRATIONS.iter().enumerate() {
        let version = (index + 1) as i32;
        if version > current_version {
            conn.execute(migration, [])
                .map_err(|e| format!("Failed to run migration {}: {}", version, e))?;

            conn.execute(
                "INSERT INTO schema_migrations (version) VALUES (?1)",
                [version],
            )
            .map_err(|e| format!("Failed to record migration {}: {}", version, e))?;
        }
    }

    Ok(())
}
