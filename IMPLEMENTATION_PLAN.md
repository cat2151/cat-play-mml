# Implementation Plan: Windows Executable for cat-play-mml

## Overview

This document outlines the implementation plan for creating a single Windows executable that integrates three existing Rust crates to provide a complete MML (Music Macro Language) to audio playback pipeline.

## Architecture

### Processing Pipeline

The application follows a sequential processing flow:

```
MML Text Input (e.g., "cde")
    ↓
[mmlabc-to-smf] → Standard MIDI File (in memory)
    ↓
[smf-to-ym2151log] → YM2151 Register Log (JSON)
    ↓
[ym2151-log-player] → Audio Playback
```

### Component Repositories

1. **mmlabc-to-smf-rust**
   - Repository: https://github.com/cat2151/mmlabc-to-smf-rust
   - Purpose: Parses MML text using tree-sitter and converts it to Standard MIDI File format
   - Current interface: Binary executable
   
2. **smf-to-ym2151log-rust**
   - Repository: https://github.com/cat2151/smf-to-ym2151log-rust
   - Purpose: Converts SMF to YM2151 register write log in JSON format
   - Current interface: Binary executable + Library (`smf_to_ym2151log`)
   
3. **ym2151-log-player-rust**
   - Repository: https://github.com/cat2151/ym2151-log-player-rust
   - Purpose: Plays YM2151 register logs using the Nuked-OPM emulator
   - Current interface: Binary executable

## Implementation Strategy

### 1. Project Structure

Create a new Rust project in this repository:

```
cat-play-mml/
├── Cargo.toml          # Main workspace configuration
├── src/
│   └── main.rs        # Windows executable entry point
├── README.md
├── README.ja.md
└── IMPLEMENTATION_PLAN.md (this file)
```

### 2. Dependency Configuration

The main `Cargo.toml` will reference the three crates as git dependencies:

```toml
[package]
name = "cat-play-mml"
version = "0.1.0"
edition = "2021"
authors = ["cat2151"]
license = "MIT"
description = "Music Macro Language (MML) Parser and Player"

[[bin]]
name = "cat-play-mml"
path = "src/main.rs"

[dependencies]
# Reference the three crates from their GitHub repositories
mmlabc-to-smf = { git = "https://github.com/cat2151/mmlabc-to-smf-rust" }
smf-to-ym2151log = { git = "https://github.com/cat2151/smf-to-ym2151log-rust" }
ym2151-log-player-rust = { git = "https://github.com/cat2151/ym2151-log-player-rust" }

# Additional dependencies
anyhow = "1.0"
clap = { version = "4.5", features = ["derive"] }
```

### 3. Library Exposure Requirements

For the integration to work, each crate needs to expose its functionality as a library:

#### mmlabc-to-smf-rust
**Current state**: Only has a binary (`[[bin]]`)
**Required changes**: 
- Add `[lib]` section to `Cargo.toml`
- Create `src/lib.rs` exposing the MML parsing and SMF conversion functions
- Example API:
  ```rust
  pub fn convert_mml_to_smf(mml_text: &str) -> Result<Vec<u8>, Error>
  ```

#### smf-to-ym2151log-rust
**Current state**: Already has both binary and library
**Required changes**: None (already has `[lib]` with `src/lib.rs`)
**Expected API**:
  ```rust
  pub fn convert_smf_to_ym2151log(smf_data: &[u8]) -> Result<String, Error>
  ```

#### ym2151-log-player-rust
**Current state**: Only has a binary
**Required changes**:
- Add `[lib]` section to `Cargo.toml`
- Create `src/lib.rs` exposing the playback functions
- Example API:
  ```rust
  pub fn play_ym2151_log(json_log: &str) -> Result<(), Error>
  ```

### 4. Main Application Implementation

The main application in `src/main.rs` will:

1. **Parse command-line arguments** using `clap`
   - Accept MML text as argument (e.g., "cde")
   - Support optional flags for debugging/file output

2. **Call the pipeline sequentially**:
   ```rust
   fn main() -> anyhow::Result<()> {
       // Parse command line
       let mml_input = parse_args();
       
       // Step 1: MML → SMF
       let smf_data = mmlabc_to_smf::convert_mml_to_smf(&mml_input)?;
       
       // Step 2: SMF → YM2151 Log
       let ym2151_log = smf_to_ym2151log::convert_smf_to_ym2151log(&smf_data)?;
       
       // Step 3: Play YM2151 Log
       ym2151_log_player_rust::play_ym2151_log(&ym2151_log)?;
       
       Ok(())
   }
   ```

3. **Error handling**
   - Use `anyhow` for error propagation
   - Provide user-friendly error messages

### 5. Windows-Specific Considerations

- **Console Window**: Default console subsystem is appropriate for command-line usage (no special attributes needed)
- **Audio Backend**: The `ym2151-log-player-rust` uses `cpal`, which supports Windows via WASAPI
- **Build Target**: `x86_64-pc-windows-msvc` or `x86_64-pc-windows-gnu`
- **Dependencies**: Ensure C compiler is available for building Nuked-OPM C code (MSVC or MinGW)

## Development Phases

### Phase 1: Repository Setup (This Issue)
- [x] Create implementation plan document
- [ ] Review plan with stakeholders

### Phase 2: Upstream Library Preparation
- [ ] Add library interface to `mmlabc-to-smf-rust`
- [ ] Verify library interface in `smf-to-ym2151log-rust`
- [ ] Add library interface to `ym2151-log-player-rust`

### Phase 3: Integration Implementation
- [ ] Create `Cargo.toml` with git dependencies
- [ ] Implement `src/main.rs` with sequential pipeline
- [ ] Add command-line argument parsing
- [ ] Handle errors and edge cases

### Phase 4: Testing
- [ ] Test with "cde" input (Do-Re-Mi)
- [ ] Test with various MML inputs
- [ ] Test on Windows environment
- [ ] Verify real-time playback works correctly

### Phase 5: Documentation
- [ ] Update README.md with usage instructions
- [ ] Update README.ja.md with usage instructions (Japanese)
- [ ] Document build process for Windows

## Expected Usage

```bash
# Play Do-Re-Mi
cat-play-mml.exe cde

# More complex MML
cat-play-mml.exe "t120 o4 l4 cdefgab>c"
```

## Technical Considerations

### In-Memory Processing
- All data should be passed in memory (no intermediate files)
- SMF data as `Vec<u8>`
- YM2151 log as JSON `String`

### Performance
- First playback might have latency due to compilation/initialization
- Subsequent notes should play in real-time with minimal latency

### Error Scenarios
- Invalid MML syntax
- SMF conversion errors
- Audio backend initialization failures
- Audio device not available

## Alternative Approaches Considered

### Approach A: Process-Based Integration (Rejected)
- Spawn each tool as a separate process
- Use temporary files or pipes for data transfer
- **Rejected**: Higher overhead, more complex error handling, slower

### Approach B: Monolithic Rewrite (Rejected)
- Copy all code into a single repository
- **Rejected**: Violates DRY principle, harder to maintain, loses git history

### Approach C: Git Submodules (Rejected)
- Use git submodules instead of Cargo git dependencies
- **Rejected**: More complex for users, Cargo handles this better

## Success Criteria

1. ✅ Single Windows executable (`cat-play-mml.exe`)
2. ✅ Accepts MML text as command-line argument
3. ✅ Plays "cde" as Do-Re-Mi in real-time
4. ✅ No intermediate files created
5. ✅ Error messages are clear and actionable
6. ✅ Audio playback uses YM2151 emulation via Nuked-OPM

## Future Enhancements (Out of Scope)

- File input support for longer MML pieces
- SMF file output option
- Real-time MIDI output
- GUI interface
- Effects processing

## References

- [mmlabc-to-smf-rust Repository](https://github.com/cat2151/mmlabc-to-smf-rust)
- [smf-to-ym2151log-rust Repository](https://github.com/cat2151/smf-to-ym2151log-rust)
- [ym2151-log-player-rust Repository](https://github.com/cat2151/ym2151-log-player-rust)
- [Cargo Book: Specifying Dependencies from Git](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#specifying-dependencies-from-git-repositories)
- [cpal Audio Library](https://github.com/RustAudio/cpal)

## Notes

- This is a "fuzzy" test issue as noted in the agent instructions
- The actual implementation may require adjustments based on the current state of the three repositories
- Library interfaces need to be added to the upstream repositories before integration can proceed
