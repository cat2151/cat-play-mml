use anyhow::Result;
use mmlabc_to_smf::{pass1_parser, pass2_ast, pass3_events, pass4_midi};
use smf_to_ym2151log::convert_smf_to_ym2151_log;

use crate::app::VerbosityConfig;
use crate::input::InputType;

/// Converts various input types to YM2151 JSON format
pub fn generate_json_from_input(input: &str, verbosity: &VerbosityConfig) -> Result<String> {
    let input_type = crate::input::detect_input_type(input)?;

    let ym2151_json = match input_type {
        InputType::MmlString(mml) | InputType::MmlFile(mml) => {
            convert_mml_to_json(&mml, verbosity)?
        }
        InputType::MidFile(smf_data) => convert_smf_to_json(&smf_data, verbosity)?,
        InputType::JsonFile(json) => {
            verbosity.print_verbose("Using YM2151 JSON file input...");
            json
        }
    };

    Ok(ym2151_json)
}

/// Converts MML string to YM2151 JSON
fn convert_mml_to_json(mml: &str, verbosity: &VerbosityConfig) -> Result<String> {
    verbosity.print_verbose("Processing MML input...");

    // ステップ1: MML → SMF (4パスの統合)
    verbosity.print_verbose("Step 1: Converting MML to SMF...");
    let tokens = pass1_parser::parse_mml(mml);
    let ast = pass2_ast::tokens_to_ast(&tokens);
    let events = pass3_events::ast_to_events(&ast);
    let smf_data = pass4_midi::events_to_midi(&events)?;
    verbosity.print_verbose(&format!("  SMF data generated: {} bytes", smf_data.len()));

    // ステップ2: SMF → YM2151ログ (1関数で完結)
    convert_smf_to_json(&smf_data, verbosity)
}

/// Converts SMF data to YM2151 JSON
fn convert_smf_to_json(smf_data: &[u8], verbosity: &VerbosityConfig) -> Result<String> {
    verbosity.print_verbose("Step 2: Converting SMF to YM2151 log...");
    let json = convert_smf_to_ym2151_log(smf_data)?;
    verbosity.print_verbose(&format!("  YM2151 log generated: {} bytes", json.len()));
    Ok(json)
}
