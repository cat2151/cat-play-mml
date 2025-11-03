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

### Environment Setup
- Install `Rust` and `Zig` on Windows.

### Installation *Install from GitHub to your Windows machine. This will enable you to play "Do Re Mi" using "cat-play-mml cde".
```
cargo install --git https://github.com/cat2151/cat-play-mml.git --branch main --bin cat-play-mml
```

### Supplementary Note *For those familiar with git clone
```powershell
# Build & Run *In the cloned directory
cargo run --release cegb

# Install *In the cloned directory
cargo install --path .

# Run *Once installed, you can run it from any directory like this
cat-play-mml cegb
```

## Overview

`cat-play-mml` is a CLI tool that plays music using Music Macro Language (MML). Inputting the string `cde` will play the "Do Re Mi" musical notes.

### Key Features

- **Simple, instant playback**: Just pass "cde" as an argument to play "Do Re Mi".
- **Low Latency**: Real-time music playback.

## What is MML (Music Macro Language)?

MML is a language for describing music in text. It uses notation like the following:

- `c`, `d`, `e`, `f`, `g`, `a`, `b`: Musical notes (Do, Re, Mi, Fa, Sol, La, Si).

### Planned Features
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
- Linux runner for agent's TDD (as long as the agent can do TDD) + ALSA SDK and configuration (to enable TDD in headless environments)

### Libraries Used

- Nuked-OPM
- **Rust**: cpal

## Project Goals

### Short-term Goals
- [x] For the Windows Rust executable, play "Do Re Mi" in real-time when `cde` is specified as a command-line argument.

### Next Goals
- mmlabc grammar
  - Priority
    - `;`
- Intermediate representation file output (including Standard MIDI files)

## Out of Scope

- Complex MML
- Real-time MIDI message output
- Effects (e.g., LPF, overdrive/distortion, delay)
- GUI editor

## Related Projects

### cat-play-chord (Under Consideration)

[cat-play-chord](https://github.com/cat2151/cat-play-chord) is a project (under consideration) that generates and plays MML from chord notation using chord2mml.

### Future Outlook for mml to smf
- Status
  - Only `cde` has been implemented.
    - Goal: Initially, focusing on a minimal implementation to smoothly resolve issues until real-time "Do Re Mi" playback is achieved.
  - Using SMF (Standard MIDI File)
    - Goal: Using SMF makes verification and development easier, reducing the risk of development stagnation.
- MML dialect is expected to be mmlabc, with existing know-how and a clear format.
- Intended to proceed with a TDD agent; will consider if hallucinations occur.

### smf to Nuked-OPM friendly JSON (Under Consideration)
- Example
  - Delay Vibrato
    - Based on the `tone modify delay vibrato` value in `tone settings toml`,
      - From an OPM sound driver perspective,
        - Generate a soft LFO register event for each tick.
    - Separating SMF and toml is intended to simplify the MIDI implementation on the SMF side.
      - This makes it easier to make breaking changes to sound timbres and OPM sound driver-like processing on the toml side, following the ETC principle.
- The following passes are envisioned:
  - SMF to Intermediate Representation *Assumes JSON representing SMF as text.
  - Intermediate Representation to Intermediate Representation *n times *Delay vibrato is envisioned here.
  - Intermediate Representation to Nuked-OPM friendly JSON
- Intended to proceed with a Linux Python TDD agent; will consider if hallucinations occur.

### Nuked-OPM friendly JSON Player
- Implemented (log player in a separate repository)
- Purpose: To facilitate development
  - Easier debugging, reduces the risk of development stagnation.

### Real-time FM Tone Editor (Provisional, Under Consideration)
- Purpose of writing this here
  - Rubber ducking
- Use case
  - For verification
- Priority
  - Prioritize ease of development and the ability to sketch out tone roughs with minimal operations.
- Operations
  - Right hand: Mouse x,y for increasing/decreasing assigned numerical values respectively.
  - Left hand: WASD+QE for cursor movement, SPACE to confirm, ESC to cancel, Z for UNDO (this is still vague).
  - x,y switch between DecayRate, ModulatorTL, FB, MUL via cursor movement.
  - WASD+QE, UNDO are vague.
  - Instead of cursor movement and confirmation, pressing a specific key to instantly apply effects for faster editing might be better; this will also be tested, potentially with a settings toml file.
- Sound
  - Switch between pluck and long tone timbres via cursor movement.
  - OP connection algorithm is 2-op parallel, detune has 4 different values.
- Display
  - Windows TUI 80x24
  - Timbre parameter display in mdx note.x format.
  - While running, save the timbre in mdx note.x format to the clipboard 1 second after the last numerical change; output will be limited to this, provisionally.
- All specifications are provisional, destructive changes will be frequent, prioritizing ease of development.
- Even this is too many specifications (too many to start small), so we will start small with a provisional implementation based on a more focused set of specifications.

## License

This project is released under the [MIT License](LICENSE).

*The English README.md is automatically generated from README.ja.md via Gemini's translation within GitHub Actions.