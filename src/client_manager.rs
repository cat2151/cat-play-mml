use anyhow::{Context, Result};
use ym2151_log_play_server::client;

/// Handles client communication with the server
pub struct ClientManager;

impl ClientManager {
    pub fn new() -> Self {
        Self
    }

    /// Attempts to play a file on the server
    pub fn play_file(&self, json_path: &str) -> Result<()> {
        println!("Attempting to send to server...");
        client::play_file(json_path).context("Failed to send JSON to server")
    }

    /// Stops playback on the server
    pub fn stop_playback(&self) -> Result<()> {
        println!("Sending stop command to server...");
        client::stop_playback().context("Failed to stop playback")
    }

    /// Shuts down the server
    pub fn shutdown_server(&self) -> Result<()> {
        println!("Sending shutdown command to server...");
        client::shutdown_server().context("Failed to shutdown server")
    }

    /// Checks if the error indicates the server is not running
    pub fn is_server_not_running_error(&self, error: &anyhow::Error) -> bool {
        let error_msg = format!("{:?}", error);
        error_msg.contains("パイプを開くことができません")
            || error_msg.contains("ファイルが見つかりません")
            || error_msg.contains("0x80070002")
    }
}
