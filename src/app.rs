use anyhow::{Context, Result};
use clap::Parser;

#[cfg(windows)]
use ym2151_log_play_server::server::Server;

use crate::cli::Args;
#[cfg(windows)]
use crate::client_manager::ClientManager;
use crate::converter::generate_json_from_input;
#[cfg(windows)]
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
    #[cfg(windows)]
    client: ClientManager,
}

impl App {
    pub fn new() -> Self {
        Self {
            #[cfg(windows)]
            client: ClientManager::new(),
        }
    }

    /// Runs the application with given command-line arguments
    pub fn run(&self) -> Result<()> {
        let args = Args::parse();
        let verbosity = VerbosityConfig::from_args(&args);

        match self.determine_mode(&args) {
            AppMode::WavOutput { input, output_path } => {
                self.handle_wav_output(&input, &output_path, &verbosity)
            }
            #[cfg(windows)]
            AppMode::Server => self.run_server_mode(&verbosity),
            #[cfg(windows)]
            AppMode::StopPlayback => self.handle_stop_command(&verbosity),
            #[cfg(windows)]
            AppMode::Shutdown => self.handle_shutdown_command(&verbosity),
            #[cfg(not(windows))]
            AppMode::UnsupportedOnPlatform => Err(anyhow::anyhow!(
                "Server mode (--server, --stop, --shutdown) is only supported on Windows"
            )),
            AppMode::PlayInput(input) => self.handle_play_input(&input, &verbosity),
        }
    }

    /// Determines the application mode based on command-line arguments
    fn determine_mode(&self, args: &Args) -> AppMode {
        // WAV output mode takes precedence if --output is specified
        if let Some(ref output_path) = args.output {
            if let Some(ref input) = args.input {
                return AppMode::WavOutput {
                    input: input.clone(),
                    output_path: output_path.clone(),
                };
            } else {
                // This should be caught by validation, but handle gracefully
                return AppMode::WavOutput {
                    input: "".to_string(),
                    output_path: output_path.clone(),
                };
            }
        }

        #[cfg(windows)]
        {
            if args.server {
                return AppMode::Server;
            }

            if args.stop {
                return AppMode::StopPlayback;
            }

            if args.shutdown {
                return AppMode::Shutdown;
            }
        }

        #[cfg(not(windows))]
        {
            if args.server || args.stop || args.shutdown {
                return AppMode::UnsupportedOnPlatform;
            }
        }

        if let Some(ref input) = args.input {
            return AppMode::PlayInput(input.clone());
        }

        // This should be caught by clap validation, but handle gracefully
        AppMode::PlayInput("".to_string())
    }

    /// Runs the application in server mode
    #[cfg(windows)]
    fn run_server_mode(&self, verbosity: &VerbosityConfig) -> Result<()> {
        verbosity.println("Running in server mode (idle state)");
        let server = Server::new();
        server.run()
    }

    /// Handles stop playback command
    #[cfg(windows)]
    fn handle_stop_command(&self, verbosity: &VerbosityConfig) -> Result<()> {
        self.client.stop_playback(verbosity)
    }

    /// Handles shutdown command
    #[cfg(windows)]
    fn handle_shutdown_command(&self, verbosity: &VerbosityConfig) -> Result<()> {
        self.client.shutdown_server(verbosity)
    }

    /// Handles WAV file output - generates 3 debug files
    fn handle_wav_output(
        &self,
        input: &str,
        output_path: &str,
        verbosity: &VerbosityConfig,
    ) -> Result<()> {
        if input.is_empty() {
            return Err(anyhow::anyhow!("INPUT is required for WAV output"));
        }

        verbosity.println(&format!("Generating debug WAV files from: {}", output_path));

        // Generate JSON from input
        let json_content = generate_json_from_input(input, verbosity)?;

        // Determine base path (remove .wav extension if present)
        let base_path = if output_path.ends_with(".wav") {
            &output_path[..output_path.len() - 4]
        } else {
            output_path
        };

        // 1. Generate using AudioPlayer (captures buffer at 55930 Hz, same as server)
        verbosity.println("  [1/3] Generating foo_realtime.wav (AudioPlayer wav_buffer, 55930 Hz)...");
        self.generate_realtime_wav(&json_content, &format!("{}_realtime.wav", base_path), verbosity)?;

        // 2. Generate using wav_writer with Resampler (48000 Hz)
        verbosity.println("  [2/3] Generating foo_debug48k.wav (wav_writer + resampling, 48000 Hz)...");
        self.generate_resampled_wav(&json_content, &format!("{}_debug48k.wav", base_path), verbosity)?;

        // 3. Generate using wav_writer without resampling (55930 Hz)
        verbosity.println("  [3/3] Generating foo_debug55k.wav (wav_writer, 55930 Hz)...");
        self.generate_native_wav(&json_content, &format!("{}_debug55k.wav", base_path), verbosity)?;

        verbosity.println("✅ All debug WAV files created successfully!");
        Ok(())
    }

    /// Generate WAV using AudioPlayer wav_buffer (captures at 55930 Hz, NOT resampled)
    /// This simulates what the server saves during real-time playback
    #[cfg(feature = "realtime-audio")]
    fn generate_realtime_wav(
        &self,
        json_content: &str,
        output_path: &str,
        verbosity: &VerbosityConfig,
    ) -> Result<()> {
        use ym2151_log_player_rust::audio::AudioPlayer;
        use ym2151_log_player_rust::events::EventLog;
        use ym2151_log_player_rust::player::Player;
        use ym2151_log_player_rust::wav_writer;

        let event_log: EventLog = serde_json::from_str(json_content)
            .context("Failed to parse YM2151 JSON log")?;

        let player = Player::new(event_log);
        let mut audio_player = AudioPlayer::new(player)
            .context("Failed to create AudioPlayer")?;

        // Wait for playback to complete
        audio_player.wait();

        // Get the wav_buffer (stores at 55930 Hz, NOT resampled)
        let buffer = audio_player.get_wav_buffer();
        
        // Write to WAV file at 55930 Hz (native OPM rate)
        wav_writer::write_wav(output_path, &buffer, 55930)
            .context("Failed to write realtime WAV file")?;

        verbosity.println(&format!("    ✓ Created: {} ({} samples, 55930 Hz)", output_path, buffer.len() / 2));
        Ok(())
    }

    /// Generate WAV using AudioPlayer (non-audio feature, skip)
    #[cfg(not(feature = "realtime-audio"))]
    fn generate_realtime_wav(
        &self,
        _json_content: &str,
        output_path: &str,
        verbosity: &VerbosityConfig,
    ) -> Result<()> {
        verbosity.println(&format!("    ⚠ Skipped: {} (realtime-audio feature not available)", output_path));
        Ok(())
    }

    /// Generate WAV using wav_writer with manual resampling (48000 Hz)
    fn generate_resampled_wav(
        &self,
        json_content: &str,
        output_path: &str,
        verbosity: &VerbosityConfig,
    ) -> Result<()> {
        use ym2151_log_player_rust::events::EventLog;
        use ym2151_log_player_rust::player::Player;
        use ym2151_log_player_rust::resampler::AudioResampler;
        use ym2151_log_player_rust::wav_writer;

        let event_log: EventLog = serde_json::from_str(json_content)
            .context("Failed to parse YM2151 JSON log")?;

        let mut player = Player::new(event_log);
        let mut resampler = AudioResampler::new()
            .context("Failed to create resampler")?;

        const GENERATION_BUFFER_SIZE: usize = 2048;
        let mut generation_buffer = vec![0i16; GENERATION_BUFFER_SIZE * 2];
        let mut resampled_output = Vec::new();

        // Generate and resample samples
        loop {
            player.generate_samples(&mut generation_buffer);
            
            let resampled = resampler.resample(&generation_buffer)
                .context("Failed to resample audio")?;
            
            resampled_output.extend_from_slice(&resampled);

            if !player.should_continue_tail() {
                break;
            }
        }

        // Write to WAV file at 48000 Hz
        wav_writer::write_wav(output_path, &resampled_output, 48000)
            .context("Failed to write resampled WAV file")?;

        verbosity.println(&format!("    ✓ Created: {} ({} samples, 48000 Hz)", output_path, resampled_output.len() / 2));
        Ok(())
    }

    /// Generate WAV using wav_writer without resampling (55930 Hz)
    fn generate_native_wav(
        &self,
        json_content: &str,
        output_path: &str,
        verbosity: &VerbosityConfig,
    ) -> Result<()> {
        use ym2151_log_player_rust::events::EventLog;
        use ym2151_log_player_rust::player::Player;
        use ym2151_log_player_rust::wav_writer;

        let event_log: EventLog = serde_json::from_str(json_content)
            .context("Failed to parse YM2151 JSON log")?;

        let player = Player::new(event_log);

        // Use existing generate_wav function
        wav_writer::generate_wav(player, output_path)
            .context("Failed to generate native WAV file")?;

        verbosity.println(&format!("    ✓ Created: {} (55930 Hz)", output_path));
        Ok(())
    }

    /// Handles input playback
    #[cfg(windows)]
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

    /// Handles input playback (non-Windows)
    #[cfg(not(windows))]
    fn handle_play_input(&self, _input: &str, _verbosity: &VerbosityConfig) -> Result<()> {
        Err(anyhow::anyhow!(
            "Real-time playback is only supported on Windows. Use --output to generate a WAV file instead."
        ))
    }

    /// Attempts to play on existing server, starts server if not running
    #[cfg(windows)]
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
    WavOutput { input: String, output_path: String },
    #[cfg(windows)]
    Server,
    #[cfg(windows)]
    StopPlayback,
    #[cfg(windows)]
    Shutdown,
    #[cfg(not(windows))]
    UnsupportedOnPlatform,
    PlayInput(String),
}
