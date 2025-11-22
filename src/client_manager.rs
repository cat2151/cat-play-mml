use anyhow::{Context, Result};
use ym2151_log_play_server::client;

use crate::app::VerbosityConfig;

/// Handles client communication with the server
pub struct ClientManager;

impl ClientManager {
    pub fn new() -> Self {
        Self
    }

    /// Sends JSON content directly to the server
    pub fn send_json(&self, json_content: &str, verbosity: &VerbosityConfig) -> Result<()> {
        verbosity.println("Sending JSON directly to server...");
        match client::send_json(json_content) {
            Ok(_) => Ok(()),
            Err(e) => {
                // エラー時はVerbosityConfigに関係なく必ずprint
                println!("Failed to send JSON to server: {}\nDebug: {:?}", e, e);
                Err(e).context("Failed to send JSON to server")
            }
        }
    }

    /// Stops playback on the server
    pub fn stop_playback(&self, verbosity: &VerbosityConfig) -> Result<()> {
        verbosity.println("Sending stop command to server...");
        client::stop_playback().context("Failed to stop playback")
    }

    /// Shuts down the server
    pub fn shutdown_server(&self, verbosity: &VerbosityConfig) -> Result<()> {
        verbosity.println("Sending shutdown command to server...");
        client::shutdown_server().context("Failed to shutdown server")
    }

    /// Checks if the error indicates the server is not running
    pub fn is_server_not_running_error(&self, error: &anyhow::Error) -> bool {
        let error_msg = format!("{:?}", error);
        error_msg.contains("Failed to connect to server")
            || error_msg.contains("パイプを開くことができません")
            || error_msg.contains("ファイルが見つかりません")
            || error_msg.contains("0x80070002")
    }
}
