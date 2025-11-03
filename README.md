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

The basic implementation for the Windows Rust executable is complete. You can play music by specifying MML via command-line arguments.

## Quick Start Guide

```powershell
cargo run --release cegb
```

## Overview

`cat-play-mml` is a multi-language project designed to parse Music Macro Language (MML) and play music. It uses tree-sitter to parse MML into an AST, converts it into an intermediate representation, and then plays the music.

### Key Features

- **MML Parser**: tree-sitter based MML grammar definition
- **Low Latency**: Real-time music playback
- **Simple Command-line Arguments**: Play C-D-E by simply passing "cde" as an argument

## What is MML (Music Macro Language)

MML is a language for describing music using text. It uses notation like the following:

- `c`, `d`, `e`, `f`, `g`, `a`, `b`: Musical notes (C, D, E, F, G, A, B)

### Planned Features
- `o4`: Octave setting (4th octave)
- `l4`: Note length setting (quarter note)
- `t120`: Tempo setting (BPM 120)
- `<`, `>`: Raising/lowering octave
- `+`, `-`: Raising/lowering semitone
- `r`: Rest

## Technical Details

### Architecture

1. **Parser**: Converts MML text to AST using tree-sitter
2. **Intermediate Representation**: Converts AST to music data structure
3. **Audio Generation**: Generates audio waveform from intermediate representation
4. **Playback**: Outputs audio using an audio library

### Development Environment

- Windows
- Rust
- Zig cc (mingw and msys2 are prohibited)
- Linux runner for TDD agent (as long as the agent can perform TDD) + ALSA SDK and configuration (to enable TDD even in a headless environment)

### Audio Libraries Used

- **Rust**: cpal

## Project Goals

### Short-term Goal
- [x] For the Windows Rust executable, when "cde" is specified as a command-line argument, C-D-E should play in real-time.

### Next Goal
- mmlabc grammar
  - Priority
    - `;`
- File output of intermediate representation (including Standard MIDI Files)

## Out of Scope

- Complex MML
- Real-time MIDI message output
- Effects (e.g., LPF, overdrive/distortion, delay)
- GUI editor

## Related Projects

### cat-play-chord Under Consideration

[cat-play-chord](https://github.com/cat2151/cat-play-chord) is a project (under consideration) that generates and plays MML from chord notation using chord2mml.

### Future Outlook for mml to smf
- Status
  - Only 'cde' has been implemented.
    - Aim: By focusing on minimal implementation initially, we can smoothly resolve issues until real-time C-D-E playback is achieved.
  - Using SMF (Standard MIDI File)
    - Aim: Using SMF makes verification and development easier, reducing the risk of development stalling.
  - Expected to use mmlabc as the MML dialect, as there is existing know-how and the format is well-defined.
  - Planned to proceed with TDD agent; will consider alternatives if hallucinations occur.

### smf to Nuked-OPM friendly JSON Under Consideration
- Example
  - Delay vibrato
    - Based on the 'tone modify delay vibrato' value in tone settings toml,
      - From an OPM sound driver perspective:
        - Generate a soft LFO register event for each tick.
  - Separating SMF and toml is intended to simplify the MIDI implementation on the SMF side.
    - This makes destructive changes to timbre and OPM sound driver-like processing easier on the toml side (ETC principle).
- The following passes are envisioned:
  - SMF to Intermediate Representation (assuming JSON that represents SMF as text)
  - Intermediate Representation to Intermediate Representation (n times, delay vibrato is envisioned here)
  - Intermediate Representation to Nuked-OPM friendly JSON
- Planned to proceed with a Linux Python TDD agent; will consider alternatives if hallucinations occur.

### Nuked-OPM friendly JSON player
- Already implemented (log player in a separate repository)
- Purpose: To facilitate development
  - Easier to debug, reducing the risk of development stalling

### Real-time FM Tone Editor (Provisional, Under Consideration)
- Purpose of this section
  - Rubber ducking
- Usage
  - For verification
- Priority
  - Prioritize ease of development and the ability to roughly sketch timbres with minimal operations.
- Operations
  - Right hand: Increase/decrease values assigned to mouse x,y respectively.
  - Left hand: WASD+QE for cursor movement, SPACE for confirmation, ESC for cancel, Z for UNDO (details are fuzzy).
  - x,y switch between DecayRate, ModulatorTL, FB, MUL via cursor movement.
  - WASD+QE, UNDO are fuzzy.
  - Instead of cursor movement and confirmation, pressing a specific key to instantly apply effects for fast editing would increase editing speed, so that will also be tried (e.g., via a settings toml file).
- Sound
  - Timbre switches between pluck and long tone via cursor movement.
  - OP connection algorithm is 2-op parallel, detune has 4 different values.
- Display
  - Windows TUI 80x24
  - Timbre parameter display is in mdx note.x format.
  - While running, timbre is saved to clipboard in mdx note.x format 1 second after the last numerical change; output is limited to this for now (provisional).
- All specifications are provisional, destructive changes will be frequent, prioritizing ease of development.
- Even this is still too many specifications (too many to start small), so we will start small with a provisional implementation using more focused specifications.

## License

This project is released under the [MIT License](LICENSE).

*The English README.md is automatically generated from README.ja.md using Gemini's translation via GitHub Actions.