use anyhow::{bail, Result};
use clap::{Parser, Subcommand};

/// Music Macro Language (MML) Parser and Player
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Command>,

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
}

#[derive(Subcommand, Debug, Clone, PartialEq, Eq)]
pub enum Command {
    /// Check whether a newer build is available on GitHub
    Check,
    /// Start the self-update flow in the background
    Update,
}

impl Args {
    pub fn validate(&self) -> Result<()> {
        let control_flags = [self.server, self.stop, self.shutdown]
            .into_iter()
            .filter(|enabled| *enabled)
            .count();

        if control_flags > 1 {
            bail!("--server, --stop, and --shutdown cannot be used together");
        }

        if self.command.is_some() && (self.input.is_some() || control_flags > 0) {
            bail!(
                "subcommands cannot be used together with INPUT, --server, --stop, or --shutdown"
            );
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_check_subcommand() {
        let args = Args::try_parse_from(["cat-play-mml", "check"]).unwrap();

        assert_eq!(args.command, Some(Command::Check));
        assert!(args.input.is_none());
        assert!(!args.server);
        assert!(!args.stop);
        assert!(!args.shutdown);
    }

    #[test]
    fn parses_update_subcommand() {
        let args = Args::try_parse_from(["cat-play-mml", "update"]).unwrap();

        assert_eq!(args.command, Some(Command::Update));
        assert!(args.input.is_none());
    }

    #[test]
    fn parses_reserved_word_as_input_after_double_dash() {
        let args = Args::try_parse_from(["cat-play-mml", "--", "check"]).unwrap();

        assert_eq!(args.command, None);
        assert_eq!(args.input.as_deref(), Some("check"));
    }

    #[test]
    fn validate_rejects_subcommand_with_server_flag() {
        let args = Args {
            command: Some(Command::Check),
            input: None,
            server: true,
            stop: false,
            shutdown: false,
            verbose: false,
        };

        let err = args.validate().unwrap_err();
        assert!(err
            .to_string()
            .contains("subcommands cannot be used together"));
    }
}
