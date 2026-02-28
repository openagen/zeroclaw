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
