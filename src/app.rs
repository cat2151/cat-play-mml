use anyhow::Result;
use clap::Parser;
use ym2151_log_play_server::server::Server;

use crate::cli::Args;
use crate::client_manager::ClientManager;
use crate::converter::generate_json_from_input;

#[derive(Clone, Copy)]
pub struct VerbosityConfig {
    pub suppress_output: bool,
}

impl VerbosityConfig {
    pub fn from_args(args: &Args) -> Self {
        Self {
            suppress_output: args.server && !args.verbose,
        }
    }

    pub fn print_verbose(&self, msg: &str) {
        if !self.suppress_output {
            println!("{}", msg);
        }
    }
}

pub struct App {
    client: ClientManager,
}

impl App {
    pub fn new() -> Self {
        Self {
            client: ClientManager::new(),
        }
    }

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

    fn run_server_mode(&self, verbosity: &VerbosityConfig) -> Result<()> {
        verbosity.print_verbose("Running in server mode (idle state)");
        let server = Server::new();
        server.run()
    }

    fn handle_stop_command(&self, verbosity: &VerbosityConfig) -> Result<()> {
        self.client.stop_playback(verbosity)
    }

    fn handle_shutdown_command(&self, verbosity: &VerbosityConfig) -> Result<()> {
        self.client.shutdown_server(verbosity)
    }

    fn handle_play_input(&self, input: &str, verbosity: &VerbosityConfig) -> Result<()> {
        if input.is_empty() {
            // 注意、コマンドライン引数チェック側に統合したほうがよいかも。今後検討するつもり
            return Err(anyhow::anyhow!(
                "INPUT is required unless using --server, --stop, or --shutdown"
            ));
        }

        let json_content = generate_json_from_input(input, verbosity)?;

        ym2151_log_play_server::client::ensure_server_ready("cat-play-mml")?;

        self.client.send_json(&json_content, verbosity)?;

        verbosity.print_verbose("Operation completed.");
        Ok(())
    }
}

/// Represents different application execution modes
enum AppMode {
    Server,
    StopPlayback,
    Shutdown,
    PlayInput(String),
}
