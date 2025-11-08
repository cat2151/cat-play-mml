use clap::Parser;

/// Music Macro Language (MML) Parser and Player
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// MML text, MML file (.mml), MIDI file (.mid), or YM2151 log (.json) to play
    #[arg(value_name = "INPUT")]
    pub input: Option<String>,

    /// Run as server with the specified JSON file
    #[arg(long)]
    pub server: Option<String>,

    /// Stop playback on running server
    #[arg(long)]
    pub stop: bool,

    /// Shutdown the running server
    #[arg(long)]
    pub shutdown: bool,
}
