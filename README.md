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

### Installation
```
cargo install --git https://github.com/cat2151/cat-play-mml.git --branch main --bin cat-play-mml
```

That's it! It will be installed on your Windows from GitHub.

### Playback
```
cat-play-mml cde
```

This plays CDE (Do-Re-Mi).

## Overview

`cat-play-mml` is a CLI tool that plays music using Music Macro Language (MML). If you input the string `cde`, it will play the notes `Do-Re-Mi`.

### Key Features

- **Simple, instant playback**: Just pass "cde" as an argument to play CDE.
- **Low latency**: Real-time music playback.

## What is MML (Music Macro Language)?

MML is a language for describing music using text. It uses notations like the following:

- `c`, `d`, `e`, `f`, `g`, `a`, `b`: Notes (C, D, E, F, G, A, B)

### Planned for Future Implementation
- `o4`: Octave setting (4th octave)
- `l4`: Note length setting (quarter note)
- `t120`: Tempo setting (BPM 120)
- `<`, `>`: Octave up/down
- `+`, `-`: Semitone up/down
- `r`: Rest

## Technical Details

### Architecture

1.  **Parser**: Converts MML text to an AST using tree-sitter.
2.  **Intermediate Representation**: Converts the AST into a music data structure.
3.  **Audio Generation**: Generates audio waveforms from the intermediate representation.
4.  **Playback**: Outputs audio using an audio library.

### Development Environment

- Windows
- Rust
- Zig cc (mingw and msys2 are prohibited)
- Linux runner for agent TDD (as long as the agent can do TDD) + ALSA SDK and configuration (to enable TDD in headless environments)

### Libraries Used

- Nuked-OPM
- **Rust**: cpal

## Project Goals

### Minor Goal
- [x] As a Windows Rust executable, when `cde` is specified as a command-line argument, it should play CDE (Do-Re-Mi) in real-time.

### Next Goal
- mmlabc syntax
  - Priority
    - `;`
- Intermediate representation file output (including Standard MIDI File)

## Out of Scope

- Complex MML
- Real-time MIDI message output
- Effects (LPF, overdrive/distortion, delay, etc.)
- GUI editor

## Related Projects

### cat-play-chord (Under Consideration)

[cat-play-chord](https://github.com/cat2151/cat-play-chord) is a project (under consideration) that uses chord2mml to generate and play MML from chord notation.

### Future Outlook for MML to SMF
- Status
  - Only `cde` is implemented.
    - Goal: Initially focusing on minimal implementation to smooth problem-solving until real-time CDE playback is achieved.
  - SMF is used.
    - Goal: Using SMF makes verification and development easier, reducing the risk of development stalling.
- MML dialect is assumed to be mmlabc, as there is expertise and a clear format.
- Planned to proceed with a TDD agent; if hallucinations occur, they will be addressed.

### SMF to Nuked-OPM friendly JSON (Under Consideration)
- Example
  - Delay Vibrato
    - Based on the `tone modify delay vibrato` value in `tone settings toml`,
      - From an OPM sound driver perspective,
        - Generate soft LFO register events per tick.
    - Separating SMF and toml is for simplifying the MIDI implementation on the SMF side.
      - This makes destructive changes to timbre and OPM sound driver-like processing easier on the toml side, following the ETC principle.
- The following passes are envisioned:
  - SMF to Intermediate Representation (json expressing SMF as text is assumed).
  - Intermediate Representation to Intermediate Representation (n times) (Delay vibrato is envisioned here).
  - Intermediate Representation to Nuked-OPM friendly JSON.
- Planned to proceed with a Linux Python TDD agent; if hallucinations occur, they will be addressed.

### Nuked-OPM friendly JSON player
- Already implemented (log player in a separate repository).
- Purpose is to facilitate development.
  - Easier debugging, reducing the risk of development stalling.

### Real-time FM Tone Editor (Provisional) (Under Consideration)
- Purpose of writing this here
  - Rubber ducking
- Use case
  - Verification
- Priority
  - Prioritize ease of development and quick sketching of tones with minimal operations.
- Operations
  - Right hand: Mouse x,y for increasing/decreasing assigned values.
  - Left hand: WASD+QE for cursor movement, SPACE for confirmation, ESC for cancel, Z for UNDO (this is vague).
  - x,y switches between DecayRate, ModulatorTL, FB, MUL with cursor movement.
  - WASD+QE, UNDO are vague.
  - Instead of cursor movement and confirmation, trying a mode where pressing a key immediately applies an effect for faster editing is also an option, perhaps via a settings toml file.
- Sound
  - Toggle between pluck and long tone timbres with cursor movement.
  - OP connection algorithm is 2-op parallel, detune values are 4 distinct values.
- Display
  - Windows TUI 80x24.
  - Tone parameter display in mdx note.x format.
  - While running, save tone to clipboard in mdx note.x format 1 second after the last numerical change; output is limited to this only, provisionally.
- All specifications are provisional, frequent destructive changes will occur, prioritizing ease of development.
- Even this is still too many specifications (too much to start small), so begin with a more constrained provisional implementation.

## Additional Information

### Build, Install, and Run Instructions for Developers

```powershell
# Build & Run (in the cloned directory)
cargo run --release cegb

# Install (in the cloned directory)
cargo install --path .

# Run (once installed, you can run it from any directory like this)
cat-play-mml cegb
```

## License

This project is released under the [MIT License](LICENSE).

*The English README.md is automatically generated from README.ja.md by GitHub Actions using Gemini's translation.*