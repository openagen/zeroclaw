# FTMS Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add file upload, storage, text extraction, AI description, and full-text search to ZeroClaw as a new Rust module.

**Architecture:** New `src/ftms/` module with its own SQLite database (`ftms.db`), integrated into the existing Axum gateway router. Files stored on disk under `~/.zeroclaw/files/YYYY/MM/DD/`, metadata and extracted text indexed in FTS5 for search. Non-text files get AI-generated descriptions via the existing provider system.

**Tech Stack:** Rust, rusqlite (bundled SQLite + FTS5), axum (multipart uploads), tokio (async fs), existing ZeroClaw config/provider systems.

---

### Task 1: Config — Add `[ftms]` Section

**Files:**
- Modify: `src/config/schema.rs` (add FtmsConfig struct + field on Config)
- Modify: `src/config/mod.rs` (re-export FtmsConfig)

**Step 1: Add FtmsConfig struct to schema.rs**

In `src/config/schema.rs`, add after `MultimodalConfig`:

```rust
fn default_ftms_max_upload_size_mb() -> usize { 50 }
fn default_ftms_storage_dir() -> String { "~/.zeroclaw/files".to_string() }

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FtmsConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default = "default_ftms_max_upload_size_mb")]
    pub max_upload_size_mb: usize,
    #[serde(default = "default_ftms_storage_dir")]
    pub storage_dir: String,
    #[serde(default = "default_true")]
    pub auto_describe: bool,
}

impl Default for FtmsConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            max_upload_size_mb: 50,
            storage_dir: default_ftms_storage_dir(),
            auto_describe: true,
        }
    }
}
```

Note: If `default_true` doesn't already exist, add: `fn default_true() -> bool { true }`

**Step 2: Add ftms field to Config struct**

In the `Config` struct (same file), add:

```rust
#[serde(default)]
pub ftms: FtmsConfig,
```

**Step 3: Re-export in mod.rs**

In `src/config/mod.rs`, add `FtmsConfig` to the use/re-export list.

**Step 4: Verify it compiles**

Run: `cargo check 2>&1 | tail -5`
Expected: no errors

**Step 5: Commit**

```bash
git add src/config/schema.rs src/config/mod.rs
git commit -m "feat(ftms): add [ftms] config section"
```

---

### Task 2: Schema — Define FTMS Data Types

**Files:**
- Create: `src/ftms/schema.rs`
- Create: `src/ftms/mod.rs`
- Modify: `src/lib.rs` (declare module)

**Step 1: Create src/ftms/mod.rs**

```rust
//! FTMS — File/Text Management System
//!
//! Handles file upload, storage, text extraction, AI description,
//! and full-text search indexing.

pub mod schema;

pub use schema::{FileRecord, FileMetadata};
```

**Step 2: Create src/ftms/schema.rs**

```rust
use serde::{Deserialize, Serialize};

/// A stored file record with metadata and extracted content.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileRecord {
    pub id: String,
    pub filename: String,
    pub mime_type: String,
    pub file_path: String,
    pub file_size: u64,
    pub extracted_text: Option<String>,
    pub ai_description: Option<String>,
    pub session_id: Option<String>,
    pub channel: Option<String>,
    pub uploaded_at: String,
    pub tags: Option<String>,
}

/// Metadata sent with an upload request (not the file bytes themselves).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetadata {
    pub session_id: Option<String>,
    pub channel: Option<String>,
    pub tags: Option<String>,
}

/// Search result with relevance score.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSearchResult {
    pub file: FileRecord,
    pub rank: f64,
}

/// Paginated list response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileListResponse {
    pub files: Vec<FileRecord>,
    pub total: usize,
    pub offset: usize,
    pub limit: usize,
}
```

**Step 3: Declare module in lib.rs**

In `src/lib.rs`, add alphabetically:

```rust
pub(crate) mod ftms;
```

**Step 4: Verify it compiles**

Run: `cargo check 2>&1 | tail -5`
Expected: no errors (warnings about unused are OK)

**Step 5: Commit**

```bash
git add src/ftms/ src/lib.rs
git commit -m "feat(ftms): add schema types and module skeleton"
```

---

### Task 3: Storage — File System Operations

**Files:**
- Create: `src/ftms/storage.rs`
- Modify: `src/ftms/mod.rs` (add pub mod)

**Step 1: Create src/ftms/storage.rs**

```rust
use anyhow::{Context, Result};
use chrono::Local;
use std::path::{Path, PathBuf};
use tokio::fs;
use uuid::Uuid;

/// Manages file storage on disk, organized by date.
pub struct FileStorage {
    base_dir: PathBuf,
}

impl FileStorage {
    pub fn new(base_dir: &str) -> Result<Self> {
        let expanded = shellexpand::tilde(base_dir).to_string();
        let base = PathBuf::from(expanded);
        Ok(Self { base_dir: base })
    }

    /// Store file bytes, returns (relative_path, absolute_path).
    pub async fn store(
        &self,
        original_filename: &str,
        data: &[u8],
    ) -> Result<(String, PathBuf)> {
        let now = Local::now();
        let date_dir = now.format("%Y/%m/%d").to_string();
        let abs_dir = self.base_dir.join(&date_dir);
        fs::create_dir_all(&abs_dir)
            .await
            .context("Failed to create date directory")?;

        let ext = Path::new(original_filename)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("bin");
        let file_id = Uuid::new_v4().to_string();
        let stored_name = format!("{}.{}", file_id, ext);

        let abs_path = abs_dir.join(&stored_name);
        fs::write(&abs_path, data)
            .await
            .context("Failed to write file")?;

        let rel_path = format!("{}/{}", date_dir, stored_name);
        Ok((rel_path, abs_path))
    }

    /// Read file bytes by relative path.
    pub async fn read(&self, rel_path: &str) -> Result<Vec<u8>> {
        let abs = self.base_dir.join(rel_path);
        fs::read(&abs).await.context("Failed to read file")
    }

    /// Delete a file by relative path.
    pub async fn delete(&self, rel_path: &str) -> Result<()> {
        let abs = self.base_dir.join(rel_path);
        if abs.exists() {
            fs::remove_file(&abs).await.context("Failed to delete file")?;
        }
        Ok(())
    }

    /// Get absolute path for a relative path.
    pub fn absolute_path(&self, rel_path: &str) -> PathBuf {
        self.base_dir.join(rel_path)
    }
}
```

**Step 2: Add to mod.rs**

In `src/ftms/mod.rs`, add:
```rust
pub mod storage;
```

**Step 3: Verify it compiles**

Run: `cargo check 2>&1 | tail -5`

**Step 4: Commit**

```bash
git add src/ftms/storage.rs src/ftms/mod.rs
git commit -m "feat(ftms): add file storage with date-organized directories"
```

---

### Task 4: Index — SQLite FTS5 Search Database

**Files:**
- Create: `src/ftms/index.rs`
- Modify: `src/ftms/mod.rs`

**Step 1: Create src/ftms/index.rs**

```rust
use super::schema::{FileRecord, FileSearchResult, FileListResponse};
use anyhow::{Context, Result};
use parking_lot::Mutex;
use rusqlite::{params, Connection};
use std::path::Path;
use std::sync::Arc;

/// SQLite-backed file index with FTS5 full-text search.
pub struct FileIndex {
    conn: Arc<Mutex<Connection>>,
}

impl FileIndex {
    pub fn new(workspace_dir: &Path) -> Result<Self> {
        let db_dir = workspace_dir.join("ftms");
        std::fs::create_dir_all(&db_dir)?;
        let db_path = db_dir.join("ftms.db");
        let conn = Connection::open(&db_path)
            .context("Failed to open ftms.db")?;

        conn.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA synchronous  = NORMAL;
             PRAGMA cache_size   = -2000;
             PRAGMA temp_store   = MEMORY;",
        )?;

        Self::init_schema(&conn)?;
        Ok(Self { conn: Arc::new(Mutex::new(conn)) })
    }

    fn init_schema(conn: &Connection) -> Result<()> {
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS ftms_files (
                id              TEXT PRIMARY KEY,
                filename        TEXT NOT NULL,
                mime_type       TEXT NOT NULL,
                file_path       TEXT NOT NULL,
                file_size       INTEGER NOT NULL,
                extracted_text  TEXT,
                ai_description  TEXT,
                session_id      TEXT,
                channel         TEXT,
                uploaded_at     TEXT NOT NULL,
                tags            TEXT
            );

            CREATE INDEX IF NOT EXISTS idx_ftms_session ON ftms_files(session_id);
            CREATE INDEX IF NOT EXISTS idx_ftms_uploaded ON ftms_files(uploaded_at);
            CREATE INDEX IF NOT EXISTS idx_ftms_mime ON ftms_files(mime_type);

            CREATE VIRTUAL TABLE IF NOT EXISTS ftms_fts USING fts5(
                filename, extracted_text, ai_description, tags,
                content='ftms_files', content_rowid='rowid'
            );

            CREATE TRIGGER IF NOT EXISTS ftms_ai AFTER INSERT ON ftms_files BEGIN
                INSERT INTO ftms_fts(rowid, filename, extracted_text, ai_description, tags)
                VALUES (new.rowid, new.filename, new.extracted_text, new.ai_description, new.tags);
            END;

            CREATE TRIGGER IF NOT EXISTS ftms_ad AFTER DELETE ON ftms_files BEGIN
                INSERT INTO ftms_fts(ftms_fts, rowid, filename, extracted_text, ai_description, tags)
                VALUES ('delete', old.rowid, old.filename, old.extracted_text, old.ai_description, old.tags);
            END;

            CREATE TRIGGER IF NOT EXISTS ftms_au AFTER UPDATE ON ftms_files BEGIN
                INSERT INTO ftms_fts(ftms_fts, rowid, filename, extracted_text, ai_description, tags)
                VALUES ('delete', old.rowid, old.filename, old.extracted_text, old.ai_description, old.tags);
                INSERT INTO ftms_fts(rowid, filename, extracted_text, ai_description, tags)
                VALUES (new.rowid, new.filename, new.extracted_text, new.ai_description, new.tags);
            END;",
        ).context("Failed to init FTMS schema")?;
        Ok(())
    }

    /// Insert a new file record.
    pub fn insert(&self, record: &FileRecord) -> Result<()> {
        let conn = self.conn.lock();
        conn.execute(
            "INSERT INTO ftms_files (id, filename, mime_type, file_path, file_size,
             extracted_text, ai_description, session_id, channel, uploaded_at, tags)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            params![
                record.id, record.filename, record.mime_type, record.file_path,
                record.file_size, record.extracted_text, record.ai_description,
                record.session_id, record.channel, record.uploaded_at, record.tags,
            ],
        ).context("Failed to insert file record")?;
        Ok(())
    }

    /// Update extracted text and AI description (for async processing).
    pub fn update_content(&self, id: &str, text: Option<&str>, description: Option<&str>) -> Result<()> {
        let conn = self.conn.lock();
        conn.execute(
            "UPDATE ftms_files SET extracted_text = ?1, ai_description = ?2 WHERE id = ?3",
            params![text, description, id],
        ).context("Failed to update file content")?;
        Ok(())
    }

    /// Get a file record by ID.
    pub fn get(&self, id: &str) -> Result<Option<FileRecord>> {
        let conn = self.conn.lock();
        let mut stmt = conn.prepare(
            "SELECT id, filename, mime_type, file_path, file_size, extracted_text,
             ai_description, session_id, channel, uploaded_at, tags
             FROM ftms_files WHERE id = ?1",
        )?;
        let result = stmt.query_row(params![id], |row| {
            Ok(FileRecord {
                id: row.get(0)?,
                filename: row.get(1)?,
                mime_type: row.get(2)?,
                file_path: row.get(3)?,
                file_size: row.get::<_, i64>(4)? as u64,
                extracted_text: row.get(5)?,
                ai_description: row.get(6)?,
                session_id: row.get(7)?,
                channel: row.get(8)?,
                uploaded_at: row.get(9)?,
                tags: row.get(10)?,
            })
        });
        match result {
            Ok(r) => Ok(Some(r)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// List files with pagination, optionally filtered by session_id or mime_type.
    pub fn list(
        &self,
        offset: usize,
        limit: usize,
        session_id: Option<&str>,
        mime_prefix: Option<&str>,
    ) -> Result<FileListResponse> {
        let conn = self.conn.lock();

        // Build dynamic query
        let (where_sql, count_params, query_params) = Self::build_filter(
            session_id, mime_prefix, offset, limit,
        );

        let count: usize = conn.query_row(
            &format!("SELECT COUNT(*) FROM ftms_files {}", where_sql),
            rusqlite::params_from_iter(&count_params),
            |row| row.get(0),
        )?;

        let sql = format!(
            "SELECT id, filename, mime_type, file_path, file_size, extracted_text,
             ai_description, session_id, channel, uploaded_at, tags
             FROM ftms_files {} ORDER BY uploaded_at DESC LIMIT ? OFFSET ?",
            where_sql,
        );

        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt.query_map(
            rusqlite::params_from_iter(&query_params),
            Self::row_to_record,
        )?;

        let files: Vec<FileRecord> = rows.filter_map(|r| r.ok()).collect();
        Ok(FileListResponse { files, total: count, offset, limit })
    }

    /// Full-text search using FTS5.
    pub fn search(&self, query: &str, limit: usize) -> Result<Vec<FileSearchResult>> {
        let conn = self.conn.lock();
        let mut stmt = conn.prepare(
            "SELECT f.id, f.filename, f.mime_type, f.file_path, f.file_size,
             f.extracted_text, f.ai_description, f.session_id, f.channel,
             f.uploaded_at, f.tags, ftms_fts.rank
             FROM ftms_fts
             JOIN ftms_files f ON f.rowid = ftms_fts.rowid
             WHERE ftms_fts MATCH ?1
             ORDER BY rank
             LIMIT ?2",
        )?;
        let rows = stmt.query_map(params![query, limit as i64], |row| {
            Ok(FileSearchResult {
                file: FileRecord {
                    id: row.get(0)?,
                    filename: row.get(1)?,
                    mime_type: row.get(2)?,
                    file_path: row.get(3)?,
                    file_size: row.get::<_, i64>(4)? as u64,
                    extracted_text: row.get(5)?,
                    ai_description: row.get(6)?,
                    session_id: row.get(7)?,
                    channel: row.get(8)?,
                    uploaded_at: row.get(9)?,
                    tags: row.get(10)?,
                },
                rank: row.get(11)?,
            })
        })?;
        Ok(rows.filter_map(|r| r.ok()).collect())
    }

    // Helper: build WHERE clause and params for list()
    fn build_filter(
        session_id: Option<&str>,
        mime_prefix: Option<&str>,
        offset: usize,
        limit: usize,
    ) -> (String, Vec<String>, Vec<String>) {
        let mut clauses = Vec::new();
        let mut count_params = Vec::new();
        let mut query_params = Vec::new();

        if let Some(sid) = session_id {
            clauses.push("session_id = ?".to_string());
            count_params.push(sid.to_string());
            query_params.push(sid.to_string());
        }
        if let Some(prefix) = mime_prefix {
            clauses.push("mime_type LIKE ?".to_string());
            let like = format!("{}%", prefix);
            count_params.push(like.clone());
            query_params.push(like);
        }

        let where_sql = if clauses.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", clauses.join(" AND "))
        };

        query_params.push(limit.to_string());
        query_params.push(offset.to_string());

        (where_sql, count_params, query_params)
    }

    fn row_to_record(row: &rusqlite::Row) -> rusqlite::Result<FileRecord> {
        Ok(FileRecord {
            id: row.get(0)?,
            filename: row.get(1)?,
            mime_type: row.get(2)?,
            file_path: row.get(3)?,
            file_size: row.get::<_, i64>(4)? as u64,
            extracted_text: row.get(5)?,
            ai_description: row.get(6)?,
            session_id: row.get(7)?,
            channel: row.get(8)?,
            uploaded_at: row.get(9)?,
            tags: row.get(10)?,
        })
    }
}
```

**Step 2: Add to mod.rs**

```rust
pub mod index;
pub use index::FileIndex;
```

**Step 3: Verify it compiles**

Run: `cargo check 2>&1 | tail -5`

**Step 4: Commit**

```bash
git add src/ftms/index.rs src/ftms/mod.rs
git commit -m "feat(ftms): add SQLite FTS5 file index"
```

---

### Task 5: Extract — Text Extraction from Files

**Files:**
- Create: `src/ftms/extract.rs`
- Modify: `src/ftms/mod.rs`

**Step 1: Create src/ftms/extract.rs**

```rust
use anyhow::Result;

/// Maximum text to extract (100KB) to avoid bloating the index.
const MAX_TEXT_LEN: usize = 102_400;

/// Extract text content from a file based on its MIME type.
/// Returns None for binary/media files that need AI description instead.
pub fn extract_text(data: &[u8], mime_type: &str, _filename: &str) -> Result<Option<String>> {
    match mime_type {
        // Plain text types — direct UTF-8 decode
        "text/plain" | "text/markdown" | "text/csv" | "text/html" | "text/xml"
        | "application/json" | "application/xml" => {
            let text = String::from_utf8_lossy(data).to_string();
            Ok(truncate_text(text))
        }

        // PDF — use pdf-extract if available
        "application/pdf" => extract_pdf(data),

        // Images, audio, video — no text extraction, needs AI description
        t if t.starts_with("image/") || t.starts_with("audio/") || t.starts_with("video/") => {
            Ok(None)
        }

        // Unknown — try as UTF-8, fall back to None
        _ => {
            match std::str::from_utf8(data) {
                Ok(text) if !text.trim().is_empty() => Ok(truncate_text(text.to_string())),
                _ => Ok(None),
            }
        }
    }
}

fn truncate_text(text: String) -> Option<String> {
    if text.trim().is_empty() {
        return None;
    }
    if text.len() > MAX_TEXT_LEN {
        Some(text[..MAX_TEXT_LEN].to_string())
    } else {
        Some(text)
    }
}

fn extract_pdf(data: &[u8]) -> Result<Option<String>> {
    #[cfg(feature = "pdf")]
    {
        match pdf_extract::extract_text_from_mem(data) {
            Ok(text) => Ok(truncate_text(text)),
            _ => Ok(None),
        }
    }
    #[cfg(not(feature = "pdf"))]
    {
        let _ = data;
        Ok(Some("[PDF document — enable pdf feature for text extraction]".to_string()))
    }
}

/// Guess MIME type from filename extension.
pub fn guess_mime_type(filename: &str) -> String {
    let ext = filename.rsplit('.').next().unwrap_or("").to_lowercase();
    match ext.as_str() {
        "txt" => "text/plain",
        "md" | "markdown" => "text/markdown",
        "csv" => "text/csv",
        "json" => "application/json",
        "xml" => "application/xml",
        "html" | "htm" => "text/html",
        "pdf" => "application/pdf",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "bmp" => "image/bmp",
        "svg" => "image/svg+xml",
        "mp3" => "audio/mpeg",
        "wav" => "audio/wav",
        "ogg" => "audio/ogg",
        "mp4" => "video/mp4",
        "webm" => "video/webm",
        "mov" => "video/quicktime",
        "zip" => "application/zip",
        "tar" => "application/x-tar",
        "gz" => "application/gzip",
        _ => "application/octet-stream",
    }
    .to_string()
}
```

**Step 2: Add to mod.rs**

```rust
pub mod extract;
```

**Step 3: Verify it compiles**

Run: `cargo check 2>&1 | tail -5`

**Step 4: Commit**

```bash
git add src/ftms/extract.rs src/ftms/mod.rs
git commit -m "feat(ftms): add text extraction with MIME detection"
```

---

### Task 6: Describe — AI-Powered Media Description

**Files:**
- Create: `src/ftms/describe.rs`
- Modify: `src/ftms/mod.rs`

**Step 1: Create src/ftms/describe.rs**

```rust
use anyhow::Result;
use base64::Engine;

/// Generate an AI description for a media file.
/// For images: encode as base64 data URI using ZeroClaw's [IMAGE:] marker system.
/// For audio/video: return basic metadata description.
pub fn describe_media(
    data: &[u8],
    mime_type: &str,
    filename: &str,
) -> Result<Option<String>> {
    if mime_type.starts_with("image/") {
        let b64 = base64::engine::general_purpose::STANDARD.encode(data);
        let data_uri = format!("data:{};base64,{}", mime_type, b64);
        Ok(Some(format!(
            "[Uploaded image: {}]\n[IMAGE:{}]",
            filename, data_uri
        )))
    } else if mime_type.starts_with("audio/") {
        Ok(Some(format!(
            "[Uploaded audio file: {}, size: {} bytes]",
            filename,
            data.len()
        )))
    } else if mime_type.starts_with("video/") {
        Ok(Some(format!(
            "[Uploaded video file: {}, size: {} bytes]",
            filename,
            data.len()
        )))
    } else {
        Ok(None)
    }
}
```

**Step 2: Add to mod.rs**

```rust
pub mod describe;
```

**Step 3: Verify it compiles**

Run: `cargo check 2>&1 | tail -5`

**Step 4: Commit**

```bash
git add src/ftms/describe.rs src/ftms/mod.rs
git commit -m "feat(ftms): add AI media description generation"
```

---

### Task 7: FTMS Service — Orchestrator in mod.rs

**Files:**
- Modify: `src/ftms/mod.rs` (add FtmsService)

**Step 1: Update src/ftms/mod.rs with FtmsService**

```rust
//! FTMS — File/Text Management System
//!
//! Handles file upload, storage, text extraction, AI description,
//! and full-text search indexing.

pub mod schema;
pub mod storage;
pub mod index;
pub mod extract;
pub mod describe;

pub use schema::{FileRecord, FileMetadata, FileSearchResult, FileListResponse};
pub use index::FileIndex;
pub use storage::FileStorage;

use anyhow::Result;
use chrono::Local;
use std::path::Path;
use std::sync::Arc;
use uuid::Uuid;

/// Main FTMS service — coordinates storage, indexing, and extraction.
pub struct FtmsService {
    pub storage: FileStorage,
    pub index: Arc<FileIndex>,
}

impl FtmsService {
    pub fn new(storage_dir: &str, workspace_dir: &Path) -> Result<Self> {
        let storage = FileStorage::new(storage_dir)?;
        let index = Arc::new(FileIndex::new(workspace_dir)?);
        Ok(Self { storage, index })
    }

    /// Upload a file: store on disk, extract text, index metadata.
    pub async fn upload(
        &self,
        filename: &str,
        data: &[u8],
        metadata: FileMetadata,
    ) -> Result<FileRecord> {
        let id = Uuid::new_v4().to_string();
        let mime_type = extract::guess_mime_type(filename);

        // Store file on disk
        let (rel_path, _abs_path) = self.storage.store(filename, data).await?;

        // Extract text content
        let extracted_text = extract::extract_text(data, &mime_type, filename)?;

        // Generate AI description for media files
        let ai_description = describe::describe_media(data, &mime_type, filename)?;

        let record = FileRecord {
            id,
            filename: filename.to_string(),
            mime_type,
            file_path: rel_path,
            file_size: data.len() as u64,
            extracted_text,
            ai_description,
            session_id: metadata.session_id,
            channel: metadata.channel,
            uploaded_at: Local::now().to_rfc3339(),
            tags: metadata.tags,
        };

        // Index in SQLite
        self.index.insert(&record)?;

        Ok(record)
    }
}
```

**Step 2: Verify it compiles**

Run: `cargo check 2>&1 | tail -5`

**Step 3: Commit**

```bash
git add src/ftms/mod.rs
git commit -m "feat(ftms): add FtmsService orchestrator"
```

---

### Task 8: Gateway Integration — Add FTMS Routes

**Files:**
- Modify: `src/gateway/mod.rs` (add AppState field, routes, handlers)

**Step 1: Add FTMS to AppState**

Find the `AppState` struct in `src/gateway/mod.rs` and add:

```rust
ftms: Option<Arc<crate::ftms::FtmsService>>,
```

**Step 2: Initialize FTMS in run_gateway()**

In the `run_gateway()` function, where AppState is constructed, add FTMS initialization:

```rust
let ftms = if config.ftms.enabled {
    let workspace_dir = crate::config::workspace_dir();
    match crate::ftms::FtmsService::new(&config.ftms.storage_dir, &workspace_dir) {
        Ok(svc) => {
            tracing::info!("FTMS enabled, storage: {}", config.ftms.storage_dir);
            Some(Arc::new(svc))
        }
        Err(e) => {
            tracing::error!("FTMS init failed: {e}");
            None
        }
    }
} else {
    None
};
```

Add `ftms` to the AppState construction.

Note: Check how `workspace_dir` is obtained in run_gateway() — it likely uses `directories::ProjectDirs` or a config path. Match the existing pattern.

**Step 3: Add routes**

In the Router::new() chain, add FTMS routes. The /upload route needs a larger body limit. Use axum's nested router approach:

```rust
// Upload route with higher body limit (50MB)
let upload_router = Router::new()
    .route("/upload", post(handle_ftms_upload))
    .layer(RequestBodyLimitLayer::new(
        config.ftms.max_upload_size_mb * 1024 * 1024,
    ))
    .with_state(state.clone());

// Main router (existing routes + FTMS query routes)
let app = Router::new()
    .route("/health",           get(handle_health))
    // ... all existing routes ...
    .route("/files",            get(handle_ftms_list))
    .route("/files/search",     get(handle_ftms_search))
    .route("/files/{id}",       get(handle_ftms_get))
    .route("/files/{id}/download", get(handle_ftms_download))
    .with_state(state)
    .layer(RequestBodyLimitLayer::new(MAX_BODY_SIZE))
    .layer(TimeoutLayer::new(Duration::from_secs(REQUEST_TIMEOUT_SECS)));

// Merge upload router (its own body limit) with main router
let app = upload_router.merge(app);
```

Note: Axum 0.8 uses `{id}` for path params (not `:id`).

**Step 4: Add handler functions**

Add these handlers to `src/gateway/mod.rs`. Each follows the same auth pattern as `handle_webhook`:

```rust
use axum::extract::Multipart;

// Auth helper to reduce duplication
fn check_bearer_auth(state: &AppState, headers: &HeaderMap) -> bool {
    if !state.pairing.require_pairing() {
        return true;
    }
    let auth = headers.get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok()).unwrap_or("");
    let token = auth.strip_prefix("Bearer ").unwrap_or("");
    state.pairing.is_authenticated(token)
}

async fn handle_ftms_upload(
    State(state): State<AppState>,
    headers: HeaderMap,
    mut multipart: Multipart,
) -> impl IntoResponse {
    if !check_bearer_auth(&state, &headers) {
        return (StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({"error": "Unauthorized"}))).into_response();
    }

    let ftms = match &state.ftms {
        Some(f) => f,
        None => return (StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({"error": "FTMS not enabled"}))).into_response(),
    };

    let mut file_data: Option<(String, Vec<u8>)> = None;
    let mut session_id: Option<String> = None;
    let mut channel: Option<String> = None;
    let mut tags: Option<String> = None;

    while let Ok(Some(field)) = multipart.next_field().await {
        let name = field.name().unwrap_or("").to_string();
        match name.as_str() {
            "file" => {
                let fname = field.file_name().unwrap_or("upload").to_string();
                if let Ok(bytes) = field.bytes().await {
                    file_data = Some((fname, bytes.to_vec()));
                }
            }
            "session_id" => { session_id = field.text().await.ok(); }
            "channel" => { channel = field.text().await.ok(); }
            "tags" => { tags = field.text().await.ok(); }
            _ => {}
        }
    }

    let (filename, data) = match file_data {
        Some(d) => d,
        None => return (StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "No file field in multipart"}))).into_response(),
    };

    let metadata = crate::ftms::FileMetadata { session_id, channel, tags };

    match ftms.upload(&filename, &data, metadata).await {
        Ok(record) => (StatusCode::OK, Json(serde_json::json!(record))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    }
}

async fn handle_ftms_list(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    if !check_bearer_auth(&state, &headers) {
        return (StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({"error": "Unauthorized"}))).into_response();
    }
    let ftms = match &state.ftms {
        Some(f) => f,
        None => return (StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({"error": "FTMS not enabled"}))).into_response(),
    };
    let offset = params.get("offset").and_then(|v| v.parse().ok()).unwrap_or(0usize);
    let limit = params.get("limit").and_then(|v| v.parse().ok()).unwrap_or(20usize);
    let session_id = params.get("session_id").map(|s| s.as_str());
    let mime_prefix = params.get("type").map(|s| s.as_str());

    match ftms.index.list(offset, limit, session_id, mime_prefix) {
        Ok(resp) => (StatusCode::OK, Json(serde_json::json!(resp))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    }
}

async fn handle_ftms_search(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    if !check_bearer_auth(&state, &headers) {
        return (StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({"error": "Unauthorized"}))).into_response();
    }
    let ftms = match &state.ftms {
        Some(f) => f,
        None => return (StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({"error": "FTMS not enabled"}))).into_response(),
    };
    let query = match params.get("q") {
        Some(q) if !q.is_empty() => q.as_str(),
        _ => return (StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "Missing ?q= parameter"}))).into_response(),
    };
    let limit = params.get("limit").and_then(|v| v.parse().ok()).unwrap_or(20usize);

    match ftms.index.search(query, limit) {
        Ok(results) => (StatusCode::OK, Json(serde_json::json!(results))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    }
}

async fn handle_ftms_get(
    State(state): State<AppState>,
    headers: HeaderMap,
    AxumPath(id): AxumPath<String>,
) -> impl IntoResponse {
    if !check_bearer_auth(&state, &headers) {
        return (StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({"error": "Unauthorized"}))).into_response();
    }
    let ftms = match &state.ftms {
        Some(f) => f,
        None => return (StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({"error": "FTMS not enabled"}))).into_response(),
    };
    match ftms.index.get(&id) {
        Ok(Some(record)) => (StatusCode::OK, Json(serde_json::json!(record))).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "File not found"}))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    }
}

async fn handle_ftms_download(
    State(state): State<AppState>,
    headers: HeaderMap,
    AxumPath(id): AxumPath<String>,
) -> impl IntoResponse {
    if !check_bearer_auth(&state, &headers) {
        return (StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({"error": "Unauthorized"}))).into_response();
    }
    let ftms = match &state.ftms {
        Some(f) => f,
        None => return (StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({"error": "FTMS not enabled"}))).into_response(),
    };
    let record = match ftms.index.get(&id) {
        Ok(Some(r)) => r,
        Ok(None) => return (StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "File not found"}))).into_response(),
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    };
    match ftms.storage.read(&record.file_path).await {
        Ok(data) => {
            let headers = [
                (header::CONTENT_TYPE, record.mime_type),
                (header::CONTENT_DISPOSITION, format!("attachment; filename=\"{}\"", record.filename)),
            ];
            (StatusCode::OK, headers, data).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    }
}
```

**Step 5: Check if axum multipart feature is enabled**

In `Cargo.toml`, verify axum features include `"multipart"`. If not, add it:
```toml
axum = { version = "0.8", default-features = false, features = ["http1", "json", "tokio", "query", "ws", "macros", "multipart"] }
```

**Step 6: Verify it compiles**

Run: `cargo check 2>&1 | tail -20`
Fix compilation errors iteratively.

**Step 7: Commit**

```bash
git add src/gateway/mod.rs src/ftms/ Cargo.toml
git commit -m "feat(ftms): integrate FTMS routes into gateway"
```

---

### Task 9: Web UI — File Upload Button

**Files:**
- Modify: `~/zeroclaw-web/index.html`

**Step 1: Add hidden file input and upload button**

Add to the message input area (next to send button):

```html
<input type="file" id="fileInput" style="display:none" accept="*/*">
<button id="attachBtn" title="Upload file" style="...">📎</button>
```

**Step 2: Add upload JavaScript**

```javascript
document.getElementById('attachBtn').onclick = () => {
    document.getElementById('fileInput').click();
};

document.getElementById('fileInput').onchange = async (e) => {
    const file = e.target.files[0];
    if (!file) return;
    const formData = new FormData();
    formData.append('file', file);
    formData.append('channel', 'web');

    const resp = await fetch('/upload', {
        method: 'POST',
        headers: { 'Authorization': 'Bearer ' + token },
        body: formData,
    });
    const result = await resp.json();
    // Display file message in chat
    addFileMessage(result);
    e.target.value = '';
};
```

**Step 3: Add file message bubble rendering**

Add a `addFileMessage()` function that creates a chat bubble showing:
- Filename and size
- Thumbnail for images (using `/files/{id}/download` as src)
- AI description if available

**Step 4: Commit**

```bash
git add ~/zeroclaw-web/index.html
git commit -m "feat(ftms): add file upload UI to web chat"
```

---

### Task 10: Proxy — Pass-Through for FTMS Routes

**Files:**
- Modify: `~/zeroclaw-web/server.py`

**Step 1: Add /upload handling to do_POST**

In `do_POST`, add:
```python
elif self.path == "/upload":
    self._handle_upload()
```

Add `_handle_upload()` method that reads the raw body and forwards it to `GATEWAY + "/upload"` with the same Content-Type header (multipart boundary must be preserved).

**Step 2: Add /files routes to do_GET**

In `do_GET`, add:
```python
elif self.path.startswith("/files"):
    self._proxy_get()
```

The existing `_proxy_get` already forwards to `GATEWAY + self.path`, so this should work as-is.

**Step 3: Commit**

```bash
git add ~/zeroclaw-web/server.py
git commit -m "feat(ftms): add proxy pass-through for FTMS routes"
```

---

### Task 11: Enable, Build, Deploy, Test

**Step 1: Enable FTMS in config**

```bash
# SSH to Pi and add to ~/.zeroclaw/config.toml:
echo -e '\n[ftms]\nenabled = true' >> ~/.zeroclaw/config.toml
```

**Step 2: Build on Pi**

```bash
cd ~/zeroclaw && cargo build --release 2>&1 | tail -10
```

Note: Building on RPi4 8GB will take 10-30 minutes for a full build. Incremental builds are faster.

**Step 3: Restart services**

```bash
systemctl --user restart zeroclaw
systemctl --user restart zeroclaw-web
```

**Step 4: Test upload via curl**

```bash
echo "Hello FTMS" > /tmp/test.txt
curl -X POST http://localhost:42617/upload \
  -F "file=@/tmp/test.txt" \
  -F "channel=cli" \
  -F "session_id=test-session"
```

Expected: JSON response with FileRecord including `extracted_text: "Hello FTMS"`

**Step 5: Test search**

```bash
curl "http://localhost:42617/files/search?q=Hello"
```

Expected: JSON array with the test file

**Step 6: Test list**

```bash
curl "http://localhost:42617/files"
```

**Step 7: Test download**

```bash
curl "http://localhost:42617/files/{id-from-upload}/download" -o /tmp/downloaded.txt
diff /tmp/test.txt /tmp/downloaded.txt
```

**Step 8: Test web UI upload**

Open `http://192.168.0.14:8081` in browser, pair, click paperclip, upload a file.

**Step 9: Push to fork**

```bash
git push origin main
```

---

## Dependency Notes

No new Cargo dependencies needed except potentially enabling the `multipart` feature on axum. All required crates already in Cargo.toml:
- `rusqlite` (bundled) — SQLite + FTS5
- `axum` — HTTP routes + multipart
- `tokio` — async file I/O
- `uuid` — file IDs
- `chrono` — timestamps
- `base64` — image encoding
- `serde`/`serde_json` — serialization
- `shellexpand` — tilde expansion
- `parking_lot` — fast mutexes
