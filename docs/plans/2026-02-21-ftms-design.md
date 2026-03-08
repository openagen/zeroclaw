# FTMS — File/Text Management System

**Date:** 2026-02-21
**Status:** Approved
**Author:** markus (modpunk)

## Purpose

Add the ability to upload files (documents, images, audio, video) through the ZeroClaw web chat UI, store them on the Pi's SD card organized by date, ingest their text content into a searchable SQLite FTS5 index, and use Claude to generate descriptions of non-text media (images, audio, video). Users can later search for uploaded files by content or find the chat session where they uploaded a file.

## Architecture

FTMS is implemented as a new Rust module at `src/ftms/` that integrates with the existing gateway (Axum router) and memory (SQLite) systems.

### Module Structure

```
src/ftms/
├── mod.rs        # Public API, route registration, FTMS init
├── storage.rs    # File system storage (date-organized directories)
├── index.rs      # SQLite FTS5 full-text search index
├── extract.rs    # Text extraction from various file types
├── describe.rs   # AI-powered description of non-text media
└── schema.rs     # Data types: FileRecord, FileMetadata, UploadRequest
```

### Data Flow

```
Upload request (multipart/form-data)
  → gateway /upload route (auth check)
  → storage.rs: save file to ~/.zeroclaw/files/YYYY/MM/DD/{uuid}.{ext}
  → extract.rs: extract text (PDF→text, DOCX→text, plain text passthrough)
  → describe.rs: if image/audio/video, call Claude to describe content
  → index.rs: insert into SQLite FTS5 table (filename, extracted text, AI description, metadata)
  → Return FileRecord JSON to client
```

### Storage Layout

Files stored under `~/.zeroclaw/files/` organized by upload date:

```
~/.zeroclaw/files/
├── 2026/
│   └── 02/
│       └── 21/
│           ├── a1b2c3d4.pdf
│           └── e5f6g7h8.png
```

### Database Schema

New table in the existing ZeroClaw SQLite database:

```sql
CREATE TABLE IF NOT EXISTS ftms_files (
    id TEXT PRIMARY KEY,           -- UUID
    filename TEXT NOT NULL,         -- Original filename
    mime_type TEXT NOT NULL,        -- Detected MIME type
    file_path TEXT NOT NULL,        -- Relative path under ~/.zeroclaw/files/
    file_size INTEGER NOT NULL,     -- Size in bytes
    extracted_text TEXT,            -- Extracted text content (nullable)
    ai_description TEXT,            -- AI-generated description (nullable)
    session_id TEXT,                -- Chat session ID for context tracking
    channel TEXT,                   -- Which channel (web, telegram, etc.)
    uploaded_at TEXT NOT NULL,       -- ISO 8601 timestamp
    tags TEXT                       -- Optional comma-separated tags
);

CREATE VIRTUAL TABLE IF NOT EXISTS ftms_files_fts USING fts5(
    filename, extracted_text, ai_description, tags,
    content='ftms_files',
    content_rowid='rowid'
);
```

### Gateway Routes

Added to the existing Axum router in `src/gateway/mod.rs`:

| Method | Path | Auth | Body | Description |
|--------|------|------|------|-------------|
| POST | /upload | Bearer token | multipart/form-data | Upload a file |
| GET | /files | Bearer token | — | List files (paginated, filterable) |
| GET | /files/:id | Bearer token | — | Get file metadata |
| GET | /files/:id/download | Bearer token | — | Download file content |
| GET | /files/search | Bearer token | ?q=query | Full-text search |

### Body Size Limit

The existing gateway enforces a 64KB body limit. FTMS needs a separate limit for the upload route:
- **Upload route**: 50MB max (configurable via config.toml)
- **All other routes**: keep existing 64KB limit

### Text Extraction Strategy

| File Type | Method |
|-----------|--------|
| .txt, .md, .csv, .json, .xml | Direct read (UTF-8) |
| .pdf | `pdf-extract` crate or shell out to `pdftotext` |
| .docx | `docx-rs` crate (XML-based, pure Rust) |
| .png, .jpg, .gif, .webp | AI description via Claude vision |
| .mp3, .wav, .ogg | AI description (metadata extraction + optional transcription) |
| .mp4, .webm | AI description (extract keyframe + describe) |

For the initial implementation, focus on: plain text files, images (Claude vision), and PDF. Other formats can be added incrementally.

### AI Description

For non-text files (images, audio, video), FTMS calls the existing provider system to generate a description:

1. Image: Send to Claude with "Describe this image in detail" prompt
2. Audio/Video: Extract metadata (duration, codec), note as "audio/video file" with metadata

This uses the existing `providers::Provider` trait already in ZeroClaw.

### Session Context Tracking

Each upload records:
- `session_id`: The chat session UUID (from webhook request context)
- `channel`: Which interface ("web", "telegram", etc.)
- `uploaded_at`: Precise timestamp

This allows users to find uploads by chat context: "I uploaded a file during that conversation about X" → search files → find session_id → retrieve chat history.

### Web UI Changes

Add to `index.html`:
- Paperclip/attachment icon next to the message input
- File picker dialog (accept all file types)
- Upload progress indicator
- Thumbnail preview for images
- File message bubble showing filename, size, and AI description

### Proxy Changes

Add to `server.py`:
- Pass-through for `/upload` (multipart, increased body limit)
- Pass-through for `/files`, `/files/:id`, `/files/:id/download`, `/files/search`

### Config

New section in `~/.zeroclaw/config.toml`:

```toml
[ftms]
enabled = true
max_upload_size_mb = 50
storage_dir = ~/.zeroclaw/files
auto_describe = true   # Use AI to describe non-text files
```

## Trade-offs

- **SQLite FTS5 over external search engine**: Keeps it lightweight and zero-dependency, matching ZeroClaw's philosophy. FTS5 is built into rusqlite.
- **Date-organized storage over content-addressed**: Simpler to browse manually, easier to backup/prune by date.
- **AI description async**: Description can happen after upload returns, so the user isn't blocked waiting for Claude to describe their image.

## Success Criteria

1. User can upload a file from web UI and it appears in `~/.zeroclaw/files/`
2. Text files are searchable by content via `/files/search?q=...`
3. Images get an AI-generated description stored in the index
4. User can find which chat session a file was uploaded in
5. Files persist across reboots
