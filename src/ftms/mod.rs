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
