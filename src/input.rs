use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

#[derive(Debug)]
pub enum InputType {
    MmlString(String),
    MmlFile(String),
    MidFile(Vec<u8>),
    JsonFile(String),
}

/// Detects the type of input based on file extension or content
pub fn detect_input_type(input: &str) -> Result<InputType> {
    let path = Path::new(input);

    // Check if it's a file with a recognized extension
    if let Some(ext) = path.extension() {
        let ext_lower = ext.to_string_lossy().to_lowercase();
        match ext_lower.as_str() {
            "mml" => {
                let content = fs::read_to_string(path)
                    .context(format!("Failed to read MML file: {}", input))?;
                return Ok(InputType::MmlFile(content));
            }
            "mid" => {
                let content =
                    fs::read(path).context(format!("Failed to read MIDI file: {}", input))?;
                return Ok(InputType::MidFile(content));
            }
            "json" => {
                let content = fs::read_to_string(path)
                    .context(format!("Failed to read JSON file: {}", input))?;
                return Ok(InputType::JsonFile(content));
            }
            _ => {}
        }
    }

    // Otherwise, treat as MML string
    Ok(InputType::MmlString(input.to_string()))
}
