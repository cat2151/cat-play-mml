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
- Install `Rust` and `Zig` on Windows

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

`cat-play-mml` is a CLI tool that plays music using Music Macro Language (MML). If you input the string `cde`, it will play the music "Do-Re-Mi".

### Key Features

- **Simple, instant playback**: Just pass "cde" as an argument to play Do-Re-Mi.
- **Low Latency**: Real-time music playback.

## What is MML (Music Macro Language)?

MML is a language for describing music using text. It uses notations like the following:

- `c`, `d`, `e`, `f`, `g`, `a`, `b`: Scale (Do, Re, Mi, Fa, So, La, Ti)

### Planned for Future Implementation
- `o4`: Octave setting (4th octave)
- `l4`: Note length setting (quarter note)
- `t120`: Tempo setting (BPM 120)
- `<`, `>`: Octave up/down
- `+`, `-`: Semitone up/down
- `r`: Rest

## Technical Details

### Architecture

1.  **Parser**: Converts MML text to AST using tree-sitter.
2.  **Intermediate Representation**: Converts AST to music data structures.
3.  **Audio Generation**: Generates audio waveforms from the intermediate representation.
4.  **Playback**: Outputs audio using an audio library.

### Development Environment

- Windows
- Rust
- Zig cc (mingw and msys2 are prohibited)
- Linux runner for agent's TDD (as long as the agent can do TDD) + ALSA SDK and configuration (to enable TDD in headless environments)

### Libraries Used

- Nuked-OPM
- **Rust**: cpal

## Project Goals

### Minor Goals
- [x] For the Windows Rust executable, when `cde` is specified as a command-line argument, Do-Re-Mi should play in real-time.

### Next Goals
- mmlabc syntax
  - Priority
    - `;`
- Intermediate representation file output (including Standard MIDI files)

## Out of Scope

- Complex MML
- Real-time MIDI message output
- Effects (e.g., LPF, Overdrive/Distortion, Delay)
- GUI editor

## Related Projects

### cat-play-chord (Under consideration)

[cat-play-chord](https://github.com/cat2151/cat-play-chord) is a project (under consideration) that generates and plays MML from chord notation using chord2mml.

### mml to smf Future Outlook
- Status
  - Only `cde` has been implemented.
    - Goal: By focusing on a minimal implementation initially, we aim to smoothly resolve issues leading to real-time playback of Do-Re-Mi.
  - Using SMF
    - Goal: Using SMF makes verification and development easier, reducing the risk of development stalling.
- The MML dialect is assumed to be mmlabc, as there is expertise and a clear format.
- It is assumed that development will proceed with a TDD agent; hallucinations will be considered if they occur.

### smf to Nuked-OPM friendly JSON (Under consideration)
- Example
  - Delay Vibrato
    - Based on the `tone modify delay vibrato` value in `tone settings toml`,
      - From an OPM sound driver perspective,
        - Generate a soft LFO register event for each tick.
    - Separating SMF and toml is for simplifying the MIDI implementation on the SMF side.
      - This makes it easier to make breaking changes to timbre and OPM sound driver-like processing on the toml side (ETC principle).
- The following passes are envisioned:
  - SMF to intermediate representation * (SMF expressed as text in JSON format is assumed)
  - Intermediate representation to intermediate representation * (n times) * (Delay vibrato is envisioned here)
  - Intermediate representation to Nuked-OPM friendly JSON
- It is assumed that development will proceed with a Linux Python TDD agent; hallucinations will be considered if they occur.

### Nuked-OPM friendly JSON player
- Already implemented (log player in a separate repository)
- Purpose: To facilitate development.
  - Easier to debug, reducing the risk of development stalling.

### Real-time FM Tone Editor (Provisional, Under consideration)
- Purpose of writing here
  - Rubber ducking
- Usage
  - For verification
- Priority
  - Prioritize ease of development and the ability to sketch out tone colors with minimal operations.
- Operation
  - Right hand: Mouse x, y for increasing/decreasing values assigned to them.
  - Left hand: WASD+QE for cursor movement, SPACE for confirmation, ESC for cancel, Z for UNDO (this part is vague).
  - x,y switch between DecayRate, ModulatorTL, FB, MUL with cursor movement.
  - WASD+QE, UNDO are vague.
  - Instead of cursor movement and confirmation, pressing a specific key to instantly apply an effect and enable fast editing might increase editing speed, so that will also be tried, possibly with a settings toml file.
- Sound
  - Switch between pluck and long tone for timbre with cursor movement.
  - OP connection algorithm is 2-op parallel, detune has 4 different values.
- Display
  - Windows TUI 80x24
  - Tone parameter display in mdx note.x format.
  - While running, save the tone in mdx note.x format to the clipboard 1 second after the last value change; output is limited to this for now (provisional).
- All specifications are provisional, frequent breaking changes are expected, prioritizing ease of development.
- Even this is too many specifications (too many to start small), so we will start small with a provisional implementation using even more constrained specifications.

## License

This project is released under the [MIT License](LICENSE).

*The English README.md is automatically generated from README.ja.md using Gemini's translation via GitHub Actions.