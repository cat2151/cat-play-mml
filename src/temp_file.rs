use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

/// Manages temporary file operations
pub struct TempFileManager {
    temp_path: Option<PathBuf>,
}

impl TempFileManager {
    pub fn new() -> Self {
        Self { temp_path: None }
    }

    /// Creates a temporary JSON file with unique name
    pub fn create_temp_json(&mut self, content: &str) -> Result<String> {
        let process_id = std::process::id();
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let temp_filename = format!("cat_play_mml_{}_{}_.json", process_id, timestamp);
        let temp_json_path = std::env::temp_dir().join(temp_filename);

        fs::write(&temp_json_path, content).context("Failed to write temporary JSON file")?;

        let temp_path_str = temp_json_path
            .to_str()
            .context("Failed to convert temp path to string")?
            .to_string();

        self.temp_path = Some(temp_json_path);
        Ok(temp_path_str)
    }

    /// Cleans up the temporary file
    pub fn cleanup(&mut self) {
        if let Some(ref path) = self.temp_path {
            if let Err(e) = fs::remove_file(path) {
                eprintln!("Warning: Failed to clean up temporary file: {}", e);
            }
            self.temp_path = None;
        }
    }
}

impl Drop for TempFileManager {
    fn drop(&mut self) {
        self.cleanup();
    }
}
