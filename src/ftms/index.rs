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
