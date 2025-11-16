use anyhow::{Context, Result};
use clap::Parser;
use ym2151_log_play_server::server::Server;

use crate::cli::Args;
use crate::client_manager::ClientManager;
use crate::converter::generate_json_from_input;
use crate::process_manager::spawn_server_process;
use crate::temp_file::TempFileManager;

/// Main application controller that orchestrates the entire workflow
pub struct App {
    client: ClientManager,
}

impl App {
    pub fn new() -> Self {
        Self {
            client: ClientManager::new(),
        }
    }

    /// Runs the application with given command-line arguments
    pub fn run(&self) -> Result<()> {
        let args = Args::parse();

        match self.determine_mode(&args) {
            AppMode::Server => self.run_server_mode(),
            AppMode::StopPlayback => self.handle_stop_command(),
            AppMode::Shutdown => self.handle_shutdown_command(),
            AppMode::PlayInput(input) => self.handle_play_input(&input),
        }
    }

    /// Determines the application mode based on command-line arguments
    fn determine_mode(&self, args: &Args) -> AppMode {
        if args.server {
            return AppMode::Server;
        }

        if args.stop {
            return AppMode::StopPlayback;
        }

        if args.shutdown {
            return AppMode::Shutdown;
        }

        if let Some(ref input) = args.input {
            return AppMode::PlayInput(input.clone());
        }

        // This should be caught by clap validation, but handle gracefully
        AppMode::PlayInput("".to_string())
    }

    /// Runs the application in server mode
    fn run_server_mode(&self) -> Result<()> {
        println!("Running in server mode (idle state)");
        let server = Server::new();
        server.run()
    }

    /// Handles stop playback command
    fn handle_stop_command(&self) -> Result<()> {
        self.client.stop_playback()
    }

    /// Handles shutdown command
    fn handle_shutdown_command(&self) -> Result<()> {
        self.client.shutdown_server()
    }

    /// Handles input playback
    fn handle_play_input(&self, input: &str) -> Result<()> {
        if input.is_empty() {
            return Err(anyhow::anyhow!(
                "INPUT is required unless using --server, --stop, or --shutdown"
            ));
        }

        // Generate JSON from input
        let json_content = generate_json_from_input(input)?;

        // Create temporary file
        let mut temp_manager = TempFileManager::new();
        let temp_json_path = temp_manager.create_temp_json(&json_content)?;

        // Try to send to server, start server if needed
        let result = self.try_play_or_start_server(&temp_json_path);

        // Cleanup is handled by TempFileManager's Drop implementation
        result?;
        println!("Operation completed.");
        Ok(())
    }

    /// Attempts to play on existing server, starts server if not running
    fn try_play_or_start_server(&self, json_path: &str) -> Result<()> {
        match self.client.play_file(json_path) {
            Ok(_) => {
                println!("Successfully sent to existing server.");
                Ok(())
            }
            Err(e) => {
                if self.client.is_server_not_running_error(&e) {
                    println!("Server is not running. Starting server...");
                    spawn_server_process()?;
                    // Give server a moment to start
                    std::thread::sleep(std::time::Duration::from_millis(500));
                    // Now try to send the file to the server
                    self.client.play_file(json_path)?;
                    println!("Successfully sent to newly started server.");
                    Ok(())
                } else {
                    Err(e).context("Failed to send JSON to server")
                }
            }
        }
    }
}

/// Represents different application execution modes
enum AppMode {
    Server,
    StopPlayback,
    Shutdown,
    PlayInput(String),
}
