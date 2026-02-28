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
    #[cfg(feature = "rag-pdf")]
    {
        match pdf_extract::extract_text_from_mem(data) {
            Ok(text) => Ok(truncate_text(text)),
            _ => Ok(None),
        }
    }
    #[cfg(not(feature = "rag-pdf"))]
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
