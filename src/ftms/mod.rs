//! FTMS — File/Text Management System
//!
//! Handles file upload, storage, text extraction, AI description,
//! and full-text search indexing.

pub mod schema;
pub mod storage;
pub mod index;
pub mod extract;
pub mod describe;

pub use schema::{FileRecord, FileMetadata};
pub use index::FileIndex;
