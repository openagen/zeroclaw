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
