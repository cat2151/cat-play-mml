# cat-play-mml

🎵 Music Macro Language (MML) Parser and Player

<p align="left">
  <a href="README.ja.md"><img src="https://img.shields.io/badge/🇯🇵-Japanese-red.svg" alt="Japanese"></a>
  <a href="README.md"><img src="https://img.shields.io/badge/🇺🇸-English-blue.svg" alt="English"></a>
</p>

## Quick Links
| Item | Link |
|------|--------|
| 📊 Development Status | [generated-docs/development-status](generated-docs/development-status.md) |

## Overview

`cat-play-mml` is a CLI tool that plays music using Music Macro Language (MML). If you input the string `cde`, it will play the music `do-re-mi`. It is designed for Windows.

## Quick Start Guide

### Environment Setup
- Install `Rust` and `Zig` on Windows.

### Installation
```
cargo install --git https://github.com/cat2151/cat-play-mml.git --branch main --bin cat-play-mml
```

That's it! It will be installed on your Windows from GitHub.

### Play Music
```
cat-play-mml cde
```

It will play do-re-mi.

## Future Plans (Candidates to be written in related project issues)

- [mmlabc-to-smf-rust](https://github.com/cat2151/mmlabc-to-smf-rust/issues):
  - Implement MML `;`. `c;e;g` will form a C major chord. To allow assigning different timbres in the future, it should not be ch1 do-mi-sol, but rather ch1 do, ch2 mi, ch3 sol (1-based indexing).
  - Output MIDI Program Change 0 (0-based indexing) at the beginning of each channel in the SMF. Do not output for channels without note output.

- [smf-to-ym2151log-rust](https://github.com/cat2151/smf-to-ym2151log-rust/issues)
  - Support multiple channels with the implementation of MML `;`. Chords within a channel will be considered separately; for now, undefined behavior is acceptable.
  - MIDI Program Change 0 (0-based indexing) should be an acoustic grand piano-like timbre. More specifically, it just needs to not be a sine wave.

- [ym2151-log-play-server](https://github.com/cat2151/ym2151-log-play-server/issues)

### Key Features

- **Simple, instant playback**: Just pass "cde" as an argument to play do-re-mi.
- **Low latency**: Real-time music playback.
- **Background playback**: Play music in server mode while performing other operations.

### Usage

#### Basic Usage (Automatic Server Startup)

```
cat-play-mml cde
```

On the first run, the server automatically starts, and playback begins in the background. The command finishes immediately, allowing you to enter the next command.

For subsequent runs, the playback is sent to the already running server:

```
cat-play-mml efg
```

#### Server Control

Stop playback:

```
cat-play-mml --stop
```

Shut down the server:

```
cat-play-mml --shutdown
```

#### Manual Server Startup (For Advanced Users)

Start the server by specifying a JSON file:

```
cat-play-mml --server output.json
```

## What is MML (Music Macro Language)?

MML is a language for describing music using text. It uses notations such as:

- `c`, `d`, `e`, `f`, `g`, `a`, `b`: Scale notes (do, re, mi, fa, sol, la, si)
- `o4`: Octave setting (4th octave)
- `l4`: Note length setting (quarter note)
- `t120`: Tempo setting (BPM 120)
- `<`, `>`: Octave up/down
- `+`, `-`: Semitone up/down
- `r`: Rest

## Technical Details

### Architecture

1. **Parser**: Uses tree-sitter to convert MML text into an AST.
2. **Intermediate Representation**: Converts the AST into a music data structure.
3. **Audio Generation**: Generates audio waveforms from the intermediate representation.
4. **Playback**: Outputs audio using an audio library.

### Development Environment

- Windows
- Rust
- Zig cc (mingw and msys2 are prohibited)
- Linux runner for agent TDD (as long as the agent can do TDD) + ALSA SDK and configuration (to enable TDD in a headless environment)

### Libraries Used

- Nuked-OPM
- **Rust**: cpal

## Project Goals

### Small Goal
- [x] As a Windows Rust executable, when `cde` is specified as a command-line argument, it should play do-re-mi in real-time.

### Next Goals
- mmlabc grammar
  - Priority
    - `;`
- File output of intermediate representation (including Standard MIDI files)

## Out of Scope

- Complex MML
- Real-time MIDI message output
- Effects (e.g., LPF, overdrive/distortion, delay)
- GUI editor

## Related Projects

### cat-play-chord (Under Consideration)

[cat-play-chord](https://github.com/cat2151/cat-play-chord) is a project that generates and plays MML from chord notation using chord2mml (under consideration).

### Future Outlook for MML to SMF
- Status
  - Only `cde` has been implemented.
    - Goal: By initially focusing on minimal implementation, smooth problem-solving until do-re-mi plays in real-time.
  - SMF is being used.
    - Goal: Using SMF makes verification and development easier, reducing the risk of development stalling.
- MML dialect is assumed to be mmlabc, with existing know-how and a clear format.
- Development is planned with a TDD agent; hallucinations will be considered if they occur.

### SMF to Nuked-OPM friendly JSON (Under Consideration)
- Example
  - Delay Vibrato
    - Based on the `tone modify delay vibrato` value in `tone settings toml`,
      - In terms of the OPM sound driver,
        - Generate soft LFO register events for each tick.
    - Separating SMF and toml is for simplifying the MIDI implementation on the SMF side.
      - This makes it easier to make breaking changes to timbres and OPM sound driver-like processing on the toml side, following the ETC principle.
- The following passes are envisioned:
  - SMF to Intermediate Representation * (SMF expressed as text JSON is assumed)
  - Intermediate Representation to Intermediate Representation * (n times) * (Delay vibrato is envisioned here)
  - Intermediate Representation to Nuked-OPM friendly JSON
- Development is planned with a Linux Python TDD agent; hallucinations will be considered if they occur.

### Nuked-OPM friendly JSON player
- Implemented (log player in a separate repository)
- Purpose: To facilitate development.
  - Easier debugging, reduces the risk of development stalling.

### Real-time FM Tone Editor (Provisional, Under Consideration)
- Purpose of writing here
  - Rubber ducking
- Purpose
  - For verification
- Priority
  - Prioritize ease of development and the ability to sketch out timbres with minimal operations.
- Operations
  - Right hand: Increase/decrease numerical values assigned to mouse x,y respectively.
  - Left hand: WASD+QE for cursor movement, SPACE to confirm, ESC to cancel, Z for UNDO (this part is vague).
  - x,y switches between DecayRate, ModulatorTL, FB, MUL with cursor movement.
  - WASD+QE, UNDO are vague.
  - Instead of cursor movement and confirmation, pressing a specific key to instantly apply effects for faster editing might be better, so we'll try that too (e.g., with a settings toml file).
- Sound
  - Timbres switch between pluck and long tone with cursor movement.
  - OP connection algorithm is 2op parallel, detune has 4 different values.
- Display
  - Windows TUI 80x24
  - Timbre parameter display in mdx note.x format.
  - During startup, timbre is saved to clipboard in mdx note.x format 1 second after the last numerical change; output is tentatively limited to this.
- All specifications are provisional; breaking changes will be frequent, prioritizing ease of development.
  - Even this is still too many specifications (too much for starting small), so we will start small with a provisional implementation based on more focused specifications.

### cat-edit-ym2151-tone
- Output: Plays every time a key is pressed.
  - Internally, it plays MML.
  - First, verify with an extremely simple implementation.
- Purpose: For verifying the provision of a timbre creation experience, which is one of the true pleasures of synthesizers.
  - Various specifications are needed to provide a better success experience, but these will be excluded from the initial implementation (otherwise, it becomes endless and gets lost).
    - Start small. Take one step at a time. That is global optimization.
- Make it a separate repository as an independent TUI application. Start small.

### cat-edit-mmlabc
- Output: Plays every time a key is pressed.
- Purpose: To provide the experience of playing 'do' when 'c' is pressed.
- Make it a separate repository as an independent TUI application. Start small.

## Appendix

### How to Build, Install, and Run for Developers

```powershell
# Build & Run * In the cloned directory
cargo run --release cegb

# Install * In the cloned directory
cargo install --path .

# Run * Once installed, you can run it from any directory like this
cat-play-mml cegb
```

## License

This project is released under the [MIT License](LICENSE).

* The English README.md is automatically generated by GitHub Actions using Gemini's translation based on README.ja.md.