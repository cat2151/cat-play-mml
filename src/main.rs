use anyhow::{Context, Result};
use clap::Parser;
use mmlabc_to_smf::{pass1_parser, pass2_ast, pass3_events, pass4_midi};
use smf_to_ym2151log::convert_smf_to_ym2151_log;
use std::fs;
use std::path::Path;
use ym2151_log_player_rust::{audio::AudioPlayer, events::EventLog, player::Player};

/// Music Macro Language (MML) Parser and Player
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// MML text, MML file (.mml), MIDI file (.mid), or YM2151 log (.json) to play
    #[arg(value_name = "INPUT")]
    input: String,
}

enum InputType {
    MmlString(String),
    MmlFile(String),
    MidFile(Vec<u8>),
    JsonFile(String),
}

fn detect_input_type(input: &str) -> Result<InputType> {
    let path = Path::new(input);

    // Check if it's a file with a recognized extension
    if let Some(ext) = path.extension() {
        let ext_lower = ext.to_string_lossy().to_lowercase();
        match ext_lower.as_str() {
            "mml" => {
                let content = fs::read_to_string(path)
                    .context(format!("Failed to read MML file: {}", input))?;
                return Ok(InputType::MmlFile(content));
            }
            "mid" => {
                let content =
                    fs::read(path).context(format!("Failed to read MIDI file: {}", input))?;
                return Ok(InputType::MidFile(content));
            }
            "json" => {
                let content = fs::read_to_string(path)
                    .context(format!("Failed to read JSON file: {}", input))?;
                return Ok(InputType::JsonFile(content));
            }
            _ => {}
        }
    }

    // Otherwise, treat as MML string
    Ok(InputType::MmlString(input.to_string()))
}

fn main() -> Result<()> {
    // コマンドライン引数を解析
    let args = Args::parse();

    // 入力タイプを検出
    let input_type = detect_input_type(&args.input)?;

    // 入力タイプに応じて処理を分岐
    let ym2151_json = match input_type {
        InputType::MmlString(mml) | InputType::MmlFile(mml) => {
            println!("Processing MML input...");

            // ステップ1: MML → SMF (4パスの統合)
            println!("Step 1: Converting MML to SMF...");
            let tokens = pass1_parser::parse_mml(&mml);
            let ast = pass2_ast::tokens_to_ast(&tokens);
            let events = pass3_events::ast_to_events(&ast);
            let smf_data = pass4_midi::events_to_midi(&events)?;
            println!("  SMF data generated: {} bytes", smf_data.len());

            // ステップ2: SMF → YM2151ログ (1関数で完結)
            println!("Step 2: Converting SMF to YM2151 log...");
            let json = convert_smf_to_ym2151_log(&smf_data)?;
            println!("  YM2151 log generated: {} bytes", json.len());
            json
        }
        InputType::MidFile(smf_data) => {
            println!("Processing MIDI file input...");

            // ステップ2: SMF → YM2151ログ (1関数で完結)
            println!("Step 2: Converting SMF to YM2151 log...");
            let json = convert_smf_to_ym2151_log(&smf_data)?;
            println!("  YM2151 log generated: {} bytes", json.len());
            json
        }
        InputType::JsonFile(json) => {
            println!("Processing YM2151 JSON file input...");
            json
        }
    };

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
