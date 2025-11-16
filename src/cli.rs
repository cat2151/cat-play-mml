use clap::Parser;

/// Music Macro Language (MML) Parser and Player
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// MML text, MML file (.mml), MIDI file (.mid), or YM2151 log (.json) to play
    #[arg(value_name = "INPUT")]
    pub input: Option<String>,

    /// Run as server in idle state (no initial playback)
    #[arg(long)]
    pub server: bool,

    /// Stop playback on running server
    #[arg(long)]
    pub stop: bool,

    /// Shutdown the running server
    #[arg(long)]
    pub shutdown: bool,

    /// Enable verbose output (useful with --server for debugging)
    #[arg(long)]
    pub verbose: bool,

    /// Output WAV file path (instead of playing audio)
    #[arg(long, value_name = "FILE")]
    pub output: Option<String>,
}
