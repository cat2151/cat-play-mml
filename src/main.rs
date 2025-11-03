use anyhow::Result;
use clap::Parser;
use mmlabc_to_smf::{pass1_parser, pass2_ast, pass3_events, pass4_midi};
use smf_to_ym2151log::convert_smf_to_ym2151_log;
use ym2151_log_player_rust::{audio::AudioPlayer, events::EventLog, player::Player};

/// Music Macro Language (MML) Parser and Player
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// MML text to play (e.g., "cde" for Do-Re-Mi)
    #[arg(value_name = "MML")]
    mml: String,
}

fn main() -> Result<()> {
    // コマンドライン引数を解析
    let args = Args::parse();

    println!("Playing MML: {}", args.mml);

    // ステップ1: MML → SMF (4パスの統合)
    println!("Step 1: Converting MML to SMF...");
    let tokens = pass1_parser::parse_mml(&args.mml);
    let ast = pass2_ast::tokens_to_ast(&tokens);
    let events = pass3_events::ast_to_events(&ast);
    let smf_data = pass4_midi::events_to_midi(&events)?;
    println!("  SMF data generated: {} bytes", smf_data.len());

    // ステップ2: SMF → YM2151ログ (1関数で完結)
    println!("Step 2: Converting SMF to YM2151 log...");
    let ym2151_json = convert_smf_to_ym2151_log(&smf_data)?;
    println!("  YM2151 log generated: {} bytes", ym2151_json.len());

    // ステップ3: YM2151ログを再生
    println!("Step 3: Playing YM2151 log...");
    let event_log: EventLog = serde_json::from_str(&ym2151_json)?;
    let player = Player::new(event_log);
    let mut audio_player = AudioPlayer::new(player)?;
    println!("  Audio playback started. Press Ctrl+C to stop.");
    // Block until playback completes
    audio_player.wait();

    println!("Playback completed.");
    Ok(())
}
