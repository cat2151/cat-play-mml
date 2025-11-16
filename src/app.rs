use anyhow::{Context, Result};
use clap::Parser;
use ym2151_log_play_server::server::Server;

use crate::cli::Args;
use crate::client_manager::ClientManager;
use crate::converter::generate_json_from_input;
use crate::process_manager::spawn_server_process;

/// Configuration for controlling output verbosity
#[derive(Clone, Copy)]
pub struct VerbosityConfig {
    /// Whether to suppress output (true when in server mode without verbose flag)
    pub suppress_output: bool,
}

impl VerbosityConfig {
    /// Create a new VerbosityConfig based on command-line arguments
    pub fn from_args(args: &Args) -> Self {
        Self {
            // Suppress output when in server mode unless verbose flag is set
            suppress_output: args.server && !args.verbose,
        }
    }

    /// Print a message if output is not suppressed
    pub fn println(&self, msg: &str) {
        if !self.suppress_output {
            println!("{}", msg);
        }
    }
}

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
        let verbosity = VerbosityConfig::from_args(&args);

        match self.determine_mode(&args) {
            AppMode::Server => self.run_server_mode(&verbosity),
            AppMode::StopPlayback => self.handle_stop_command(&verbosity),
            AppMode::Shutdown => self.handle_shutdown_command(&verbosity),
            AppMode::PlayInput(input) => self.handle_play_input(&input, &verbosity),
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
    fn run_server_mode(&self, verbosity: &VerbosityConfig) -> Result<()> {
        verbosity.println("Running in server mode (idle state)");
        let server = Server::new();
        server.run()
    }

    /// Handles stop playback command
    fn handle_stop_command(&self, verbosity: &VerbosityConfig) -> Result<()> {
        self.client.stop_playback(verbosity)
    }

    /// Handles shutdown command
    fn handle_shutdown_command(&self, verbosity: &VerbosityConfig) -> Result<()> {
        self.client.shutdown_server(verbosity)
    }

    /// Handles input playback
    fn handle_play_input(&self, input: &str, verbosity: &VerbosityConfig) -> Result<()> {
        if input.is_empty() {
            return Err(anyhow::anyhow!(
                "INPUT is required unless using --server, --stop, or --shutdown"
            ));
        }

        // Generate JSON from input
        let json_content = generate_json_from_input(input, verbosity)?;

        // Try to send JSON directly to server, start server if needed
        let result = self.try_play_or_start_server(&json_content, verbosity);

        result?;
        verbosity.println("Operation completed.");
        Ok(())
    }

    /// Attempts to play on existing server, starts server if not running
    fn try_play_or_start_server(
        &self,
        json_content: &str,
        verbosity: &VerbosityConfig,
    ) -> Result<()> {
        match self.client.send_json(json_content, verbosity) {
            Ok(_) => {
                verbosity.println("Successfully sent JSON to existing server.");
                Ok(())
            }
            Err(e) => {
                if self.client.is_server_not_running_error(&e) {
                    verbosity.println("Server is not running. Starting server...");
                    spawn_server_process(verbosity)?;
                    // Give server a moment to start
                    std::thread::sleep(std::time::Duration::from_millis(500));
                    // Now try to send the JSON directly to the server
                    self.client.send_json(json_content, verbosity)?;
                    verbosity.println("Successfully sent JSON to newly started server.");
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
