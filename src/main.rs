use anyhow::{Context, Result};
use clap::Parser;
use mmlabc_to_smf::{pass1_parser, pass2_ast, pass3_events, pass4_midi};
use smf_to_ym2151log::convert_smf_to_ym2151_log;
use std::fs;
use std::path::Path;
use std::process::Command;
use ym2151_log_play_server::client;
use ym2151_log_play_server::server::Server;

/// Music Macro Language (MML) Parser and Player
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// MML text, MML file (.mml), MIDI file (.mid), or YM2151 log (.json) to play
    #[arg(value_name = "INPUT")]
    input: Option<String>,

    /// Run as server with the specified JSON file
    #[arg(long)]
    server: Option<String>,

    /// Stop playback on running server
    #[arg(long)]
    stop: bool,

    /// Shutdown the running server
    #[arg(long)]
    shutdown: bool,
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

fn is_server_running() -> bool {
    // Try to connect to the server's named pipe
    // If successful, server is running
    match ym2151_log_play_server::ipc::pipe_windows::NamedPipe::connect_default() {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn spawn_server_process(json_path: &str) -> Result<()> {
    let exe_path = std::env::current_exe().context("Failed to get current executable path")?;

    println!("Starting server process with JSON: {}", json_path);

    // Spawn the server as a detached child process
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NEW_PROCESS_GROUP: u32 = 0x00000200;
        const DETACHED_PROCESS: u32 = 0x00000008;

        Command::new(exe_path)
            .arg("--server")
            .arg(json_path)
            .creation_flags(CREATE_NEW_PROCESS_GROUP | DETACHED_PROCESS)
            .spawn()
            .context("Failed to spawn server process")?;
    }

    #[cfg(not(target_os = "windows"))]
    {
        Command::new(exe_path)
            .arg("--server")
            .arg(json_path)
            .spawn()
            .context("Failed to spawn server process")?;
    }

    println!("Server process spawned successfully");
    Ok(())
}

fn generate_json_from_input(input: &str) -> Result<String> {
    let input_type = detect_input_type(input)?;

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
            println!("Using YM2151 JSON file input...");
            json
        }
    };

    Ok(ym2151_json)
}

fn main() -> Result<()> {
    // コマンドライン引数を解析
    let args = Args::parse();

    // Handle --server mode
    if let Some(json_path) = args.server {
        println!("Running in server mode with: {}", json_path);
        let server = Server::new();
        return server.run(&json_path);
    }

    // Handle --stop command
    if args.stop {
        println!("Sending stop command to server...");
        return client::stop_playback().context("Failed to stop playback");
    }

    // Handle --shutdown command
    if args.shutdown {
        println!("Sending shutdown command to server...");
        return client::shutdown_server().context("Failed to shutdown server");
    }

    // Normal playback mode
    let input = args
        .input
        .context("INPUT is required unless using --server, --stop, or --shutdown")?;

    // Generate JSON from input
    let json_content = generate_json_from_input(&input)?;

    // Save JSON to a temporary file
    let temp_json_path = std::env::temp_dir().join("cat_play_mml_temp.json");
    fs::write(&temp_json_path, &json_content)
        .context("Failed to write temporary JSON file")?;

    let temp_json_str = temp_json_path
        .to_str()
        .context("Failed to convert temp path to string")?;

    // Check if server is running
    if is_server_running() {
        println!("Server is running. Sending JSON to server as client...");
        client::play_file(temp_json_str)
            .context("Failed to send JSON to server")?;
    } else {
        println!("Server is not running. Starting server...");
        spawn_server_process(temp_json_str)?;
        // Give server a moment to start
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    println!("Operation completed.");
    Ok(())
}
