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

That's it! It will be installed on your Windows machine from GitHub.

### Playing Music
```
cat-play-mml cde
```

Do-re-mi will play.

## Candidate Issues to be Filed in Related Projects in the Future

- [mmlabc-to-smf-rust](https://github.com/cat2151/mmlabc-to-smf-rust/issues):
  - Implement MML `;`. `c;e;g` will represent a C major chord. To allow separate timbres to be assigned in the future, it should be ch1 C, ch2 E, ch3 G (1-based notation), not ch1 C-E-G.
  - Output MIDI Program Change 0 (0-based notation) at the beginning of each channel in the SMF. Do not output to channels without note output.

- [smf-to-ym2151log-rust](https://github.com/cat2151/smf-to-ym2151log-rust/issues)
  - With the implementation of MML `;`, support multiple channels. Chords within a channel will be considered separately and can remain as undefined behavior for now.
  - MIDI Program Change 0 (0-based notation) should produce an acoustic grand piano-like timbre. More specifically, it should be something other than a sine wave.

- [ym2151-log-player-rust](https://github.com/cat2151/ym2151-log-player-rust/issues)

### Key Features

- **Simple, instant playback**: Just pass "cde" as an argument to play do-re-mi.
- **Low latency**: Real-time music playback.

## What is MML (Music Macro Language)?

MML is a language used to describe music in text. It uses notation such as:

- `c`, `d`, `e`, `f`, `g`, `a`, `b`: Scale notes (Do, Re, Mi, Fa, Sol, La, Si)

### Planned for Future Implementation
- `o4`: Octave setting (4th octave)
- `l4`: Note length setting (quarter note)
- `t120`: Tempo setting (BPM 120)
- `<`, `>`: Raise/lower octave
- `+`, `-`: Raise/lower by a semitone
- `r`: Rest

## Technical Details

### Architecture

1. **Parser**: Converts MML text to AST using tree-sitter.
2. **Intermediate Representation**: Converts AST to music data structure.
3. **Audio Generation**: Generates audio waveforms from the intermediate representation.
4. **Playback**: Outputs audio using an audio library.

### Development Environment

- Windows
- Rust
- Zig cc (mingw and msys2 are prohibited)
- Linux runner for agent TDD (as long as the agent can perform TDD) + ALSA SDK and configuration (to enable TDD in a headless environment)

### Libraries Used

- Nuked-OPM
- **Rust**: cpal

## Project Goals

### Small Goal
- [x] As a Windows Rust .exe, when "cde" is specified as a command-line argument, do-re-mi plays in real-time.

### Next Goals
- mmlabc grammar
  - Priority:
    - `;`
- File output of intermediate representation (including Standard MIDI Files)

## Out of Scope

- Complex MML
- Real-time MIDI message output
- Effects (e.g., LPF, overdrive/distortion, delay)
- GUI editor

## Related Projects

### cat-play-chord (Under consideration)

[cat-play-chord](https://github.com/cat2151/cat-play-chord) is a project (under consideration) that generates and plays MML from chord notation using chord2mml.

### Future Outlook for mml to smf
- Status
  - Only "cde" has been implemented.
    - Goal: Initially focusing on minimal implementation to streamline problem-solving until do-re-mi plays in real-time.
  - Using SMF.
    - Goal: Using SMF makes verification and development easier, reducing the risk of development stalling.
- MML dialect is assumed to be mmlabc, with existing know-how and clear format.
- Assumed to proceed with a TDD agent; if hallucinations occur, they will be investigated.

### smf to Nuked-OPM friendly JSON (Under consideration)
- Example:
  - Delay Vibrato
    - Based on the 'tone modify delay vibrato' value in the tone settings toml,
      - from an OPM sound driver perspective,
        - generate a soft LFO register event every tick.
    - Separating SMF and toml is intended to simplify the MIDI implementation on the SMF side.
      - It makes destructive changes to timbre and OPM sound driver-like processing easier on the toml side, adhering to the ETC principle.
- The following passes are envisioned:
  - SMF to intermediate representation * (assuming SMF expressed as text JSON)
  - Intermediate representation to intermediate representation * (n times) * (delay vibrato is envisioned here)
  - Intermediate representation to Nuked-OPM friendly JSON
- Assumed to proceed with a Linux Python TDD agent; if hallucinations occur, they will be investigated.

### Nuked-OPM friendly JSON player
- Already implemented (log player in a separate repository)
- Purpose: To facilitate development.
  - Easier debugging, reducing the risk of development stalling.

### Real-time FM Tone Editor (Tentative, Under Consideration)
- Purpose of writing here:
  - Rubber duck debugging.
- Purpose:
  - For verification.
- Priority:
  - Prioritize ease of development and the ability to sketch timbres with minimal operations.
- Operation:
  - Right hand: Increase/decrease assigned values with mouse x,y.
  - Left hand: WASD+QE for cursor movement, SPACE to confirm, ESC to cancel, Z for UNDO (this is vague).
  - x,y will switch between DecayRate, ModulatorTL, FB, MUL with cursor movement.
  - WASD+QE, UNDO are vague.
  - Instead of cursor movement and confirmation, pressing a specific key to instantly apply effects for faster editing might be better, so that will also be tried (e.g., using a settings toml file).
- Sound:
  - Switch between pluck and long tone timbres with cursor movement.
  - OP connection algorithm is 2-op parallel, detune has 4 different values.
- Display:
  - Windows TUI 80x24
  - Timbre parameter display in mdx note.x format.
  - While running, save timbre in mdx note.x format to clipboard 1 second after the last value change; output is limited to this for now (tentative).
- All specifications are tentative; destructive changes will be frequent, prioritizing ease of development.
- Even this is too many specifications (too many to start small), so we will start small with a tentative implementation based on more focused specifications.

### cat-edit-ym2151-tone
- Output: Plays each time a key is pressed.
  - Internally, it will play MML.
  - First, verify with an extremely simple implementation.
- Purpose: For verifying the provision of a timbre creation experience, which is one of the true pleasures of synthesizers.
  - Although various specifications are needed to provide a better success experience, these will be excluded from the initial implementation (otherwise, it will be endless and get lost).
    - Start small. Do it step by step. That is global optimization.
- Will be separated into an independent TUI application repository. Start small.

### cat-edit-mmlabc
- Output: Plays each time a key is pressed.
- Purpose: To provide the experience of "C" playing "Do".
- Will be separated into an independent TUI application repository. Start small.

## Supplementary Information

### For Developers: How to Build, Install, and Run

```powershell
# Build & Run * In the cloned directory.
cargo run --release cegb

# Install * In the cloned directory.
cargo install --path .

# Run * Once installed, you can run it from any directory like this.
cat-play-mml cegb
```

## License

This project is released under the [MIT License](LICENSE).

* The English README.md is automatically generated by GitHub Actions using Gemini's translation of README.ja.md.