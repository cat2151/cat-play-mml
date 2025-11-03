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

## Status

The basic implementation of the Windows Rust executable is complete. You can play music by specifying MML via command-line arguments.

## Quick Start Guide

```powershell
cargo run --release cegb
```

## Overview

`cat-play-mml` is a multi-language project that parses Music Macro Language (MML) and plays music. It uses tree-sitter to parse MML into an AST, converts it to an intermediate representation, and then plays the music.

### Key Features

- **MML Parser**: tree-sitter-based MML grammar definition
- **Low Latency**: Real-time music playback
- **Simple Command-Line Arguments**: Play C-D-E by simply passing "cde" as an argument

## What is MML (Music Macro Language)?

MML is a language for describing music in text. It uses the following notation:

- `c`, `d`, `e`, `f`, `g`, `a`, `b`: Musical scale (Do, Re, Mi, Fa, Sol, La, Si)

### Planned for Future Implementation
- `o4`: Octave setting (4th octave)
- `l4`: Note length setting (quarter note)
- `t120`: Tempo setting (BPM 120)
- `<`, `>`: Octave up/down
- `+`, `-`: Semitone up/down
- `r`: Rest

## Technical Details

### Architecture

1. **Parser**: Converts MML text to AST using tree-sitter
2. **Intermediate Representation**: Converts AST to music data structure
3. **Audio Generation**: Generates audio waveforms from the intermediate representation
4. **Playback**: Outputs audio using an audio library

### Development Environment

- Windows
- Rust
- Zig cc (mingw and msys2 are prohibited)
- Linux runner for agent TDD (as long as the agent can do TDD) + ALSA SDK and configuration (to enable TDD in a headless environment)

### Audio Libraries Used

- **Rust**: cpal

## Project Goals

- As a Windows Rust executable, playing C-D-E in real-time when "cde" is specified as a command-line argument.

## Expected Implementations

- File output of intermediate representation (including Standard MIDI Files)

## Out of Scope

- Complex MML
- Real-time MIDI message output
- Effects (e.g., LPF, overdrive/distortion, delay)
- GUI editor

## Related Projects

### cat-play-chord (Under Consideration)

[cat-play-chord](https://github.com/cat2151/cat-play-chord) is a project (under consideration) that uses chord2mml to generate MML from chord notation and play it.

### Future Outlook for MML to SMF
- Status
  - Only C-D-E has been implemented
    - Goal: Initially focus on minimal implementation to smooth problem-solving until C-D-E plays in real-time.
  - Using SMF (Standard MIDI File)
    - Goal: Using SMF makes verification and development easier, reducing the risk of development stalling.
- The MML dialect is assumed to be mmlabc, as there is existing know-how and a clear format.
- It is assumed to proceed with a TDD agent; hallucinations will be reviewed if they occur.

### SMF to Nuked-OPM Friendly JSON (Under Consideration)
- Example
  - Delay Vibrato
    - Based on the `tone modify delay vibrato` value in `tone settings toml`,
      - From an OPM sound driver perspective,
        - Generate a soft LFO register event for each tick.
    - Separating SMF and TOML is intended to simplify the MIDI implementation on the SMF side.
      - The TOML side allows for easier destructive changes to timbre and OPM sound driver-like processing, following the ETC principle.
- The following passes are envisioned:
  - SMF to Intermediate Representation *Assumes JSON representation of SMF as text.
  - Intermediate Representation to Intermediate Representation *n times *Delay vibrato is envisioned here.
  - Intermediate Representation to Nuked-OPM friendly JSON
- It is assumed to proceed with a Linux Python TDD agent; hallucinations will be reviewed if they occur.

### Nuked-OPM Friendly JSON Player
- Already implemented (log player in a separate repository)
- Purpose: To facilitate development
  - Easier to debug, reduces the risk of development stalling.

### Real-time FM Tone Editor (Provisional, Under Consideration)
- Purpose of writing here
  - Rubber Ducking
- Usage
  - For verification
- Priority
  - Prioritize ease of development and the ability to create rough sketches of tones with minimal operations.
- Operation
  - Right hand: Mouse x,y for increasing/decreasing assigned values respectively.
  - Left hand: WASD+QE for cursor movement, SPACE to confirm, ESC to cancel, Z for UNDO (this part is vague).
  - x,y switches between DecayRate, ModulatorTL, FB, MUL with cursor movement.
  - WASD+QE, UNDO are vague.
  - Instead of cursor movement and confirmation, pressing a specific key to instantly apply an effect for fast editing might increase editing speed; this will also be explored (e.g., via a settings TOML file).
- Sound
  - Tone color switches between pluck and long tone with cursor movement.
  - OP connection algorithm is 2-op parallel, detune has 4 different values.
- Display
  - Windows TUI 80x24
  - Tone parameter display in MDX note.x format.
  - While running, tone saved to clipboard in MDX note.x format 1 second after the last numerical change; output limited to this (provisional).
- All specifications are provisional; destructive changes will be frequent, prioritizing ease of development.
- Even this is still too many specifications (too much for a small start), so we will start small with a more refined provisional implementation.

## License

This project is licensed under the [MIT License](LICENSE).

*The English README.md is automatically generated by GitHub Actions using Gemini's translation based on README.ja.md.