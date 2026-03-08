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
