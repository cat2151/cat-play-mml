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

The basic implementation of the Windows Rust executable is complete. You can now play music by specifying MML via command-line arguments.

## Quick Start Guide

### Environment Setup
- Install `Rust` and `Zig` on Windows.

### Build
```powershell
cargo run --release cegb
```

### Install (Allows execution from anywhere)
```
cargo install --path .
```

### Run (After installation)
```
cat-play-mml cegb
```

## Overview

`cat-play-mml` is a multi-language project that parses Music Macro Language (MML) and plays music. It uses tree-sitter to parse MML into an AST, converts it to an intermediate representation, and then plays the music.

### Key Features

- **MML Parser**: tree-sitter based MML grammar definition
- **Low Latency**: Real-time music playback
- **Simple Command-line Arguments**: Play "Do-Re-Mi" by simply passing "cde" as an argument

## What is MML (Music Macro Language)?

MML is a language for describing music using text. It uses notation such as:

- `c`, `d`, `e`, `f`, `g`, `a`, `b`: Musical notes (C, D, E, F, G, A, B)

### Planned future implementations
- `o4`: Octave setting (4th octave)
- `l4`: Note length setting (quarter note)
- `t120`: Tempo setting (BPM 120)
- `<`, `>`: Octave up/down
- `+`, `-`: Semitone up/down
- `r`: Rest

## Technical Details

### Architecture

1. **Parser**: Uses tree-sitter to convert MML text into an AST
2. **Intermediate Representation**: Converts the AST into a music data structure
3. **Audio Generation**: Generates audio waveforms from the intermediate representation
4. **Playback**: Outputs audio using an audio library

### Development Environment

- Windows
- Rust
- Zig cc (no mingw or msys2)
- Linux runner for agent TDD (as long as the agent can do TDD) + ALSA SDK and configuration (to enable TDD even in headless environments)

### Audio Libraries Used

- **Rust**: cpal

## Project Goals

### Minor Goal
- [x] As a Windows Rust executable, when "cde" is specified as a command-line argument, "Do-Re-Mi" plays in real-time.

### Next Goal
- MMLABC grammar
  - Priority
    - `;`
- Intermediate representation file output (including Standard MIDI File)

## Out of Scope

- Complex MML
- Real-time MIDI message output
- Effects (e.g., LPF, overdrive/distortion, delay)
- GUI editor

## Related Projects

### cat-play-chord (Under consideration)

[cat-play-chord](https://github.com/cat2151/cat-play-chord) is a project (under consideration) that generates and plays MML from chord notation using chord2mml.

### mml to smf Future Outlook
- Status
  - Only `cde` is implemented.
    - Aim: Initially focus on minimal implementation to smooth problem-solving until real-time "Do-Re-Mi" playback.
  - Using SMF.
    - Aim: Using SMF makes verification and development easier, reducing the risk of development stalling.
- MML dialect is planned to be `mmlabc`, as there is expertise and the format is clear.
- Planned to proceed with a TDD agent; will consider if hallucinations occur.

### smf to Nuked-OPM friendly JSON (Under consideration)
- Example
  - Delay Vibrato
    - Based on the `tone modify delay vibrato` values in the tone settings toml,
      - From an OPM sound driver perspective,
        - Generate soft LFO register events per tick.
    - Separating SMF and toml is for keeping the MIDI implementation on the SMF side simple.
      - This allows destructive changes to timbre and OPM sound driver-like processing to be made easily on the toml side (ETC principle).
- The following passes are envisioned:
  - SMF to Intermediate Representation (assuming JSON expressing SMF as text)
  - Intermediate Representation to Intermediate Representation (N times, where delay vibrato is expected here)
  - Intermediate Representation to Nuked-OPM friendly JSON
- Planned to proceed with a Linux Python TDD agent; will consider if hallucinations occur.

### Nuked-OPM friendly JSON player
- Already implemented (log player in a separate repository)
- Purpose: To facilitate development.
  - Easier to debug, reduces the risk of development stalling.

### Real-time FM tone editor (tentative) under consideration
- Purpose of writing this here:
  - Rubber duck debugging
- Use case:
  - Verification
- Priority:
  - Prioritize ease of development and sketching timbres with minimal operations.
- Operations:
  - Right hand: Mouse x,y for increasing/decreasing assigned numerical values.
  - Left hand: WASD+QE for cursor movement, SPACE for confirmation, ESC for cancel, Z for UNDO (still fuzzy).
  - x,y switches between DecayRate, ModulatorTL, FB, MUL with cursor movement.
  - WASD+QE, UNDO are fuzzy.
  - Instead of cursor movement and confirmation, try if pressing a specific key to instantly apply an effect allows for faster editing; also consider a settings toml file.
- Sound:
  - Timbre switches between pluck and long tone with cursor movement.
  - OPM connection algorithm is 2-op parallel, detune has 4 different values.
- Display:
  - Windows TUI 80x24
  - Timbre parameter display in mdx note.x format.
  - While running, save timbre to clipboard in mdx note.x format 1 second after the last numerical change; restrict output to this only (tentative).
- All specifications are tentative and subject to frequent destructive changes; ease of development is prioritized.
- Even this is still too many specifications (too many to start small), so begin with a smaller, more focused tentative implementation.

## License

This project is licensed under the [MIT License](LICENSE).

※The English README.md is automatically generated by GitHub Actions using Gemini's translation based on README.ja.md.