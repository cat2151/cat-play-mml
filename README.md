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

Currently planning to implement the Windows Rust executable.

## Overview

`cat-play-mml` is a multi-language project that parses Music Macro Language (MML) and plays music. It uses tree-sitter to parse MML into an AST, converts it into an intermediate representation, and then plays the music.

### Key Features

- **MML Parser**: tree-sitter-based MML grammar definition
- **Low Latency**: Real-time music playback
- **Simple Command-line Arguments**: Just pass "cde" as an argument to play Do-Re-Mi

## What is MML (Music Macro Language)

MML is a language for describing music in text. It uses notation like the following:

- `c`, `d`, `e`, `f`, `g`, `a`, `b`: Scale notes (Do, Re, Mi, Fa, Sol, La, Si)
- `o4`: Octave setting (4th octave)
- `l4`: Note length setting (quarter note)
- `t120`: Tempo setting (BPM 120)
- `<`, `>`: Raise/lower octave
- `+`, `-`: Raise/lower by a semitone
- `r`: Rest

## Technical Details

### Architecture

1. **Parser**: Uses tree-sitter to convert MML text into an AST
2. **Intermediate Representation**: Converts the AST into a music data structure
3. **Audio Generation**: Generates audio waveforms from the intermediate representation
4. **Playback**: Outputs audio using an audio library

### Audio Libraries Used

- **Rust**: cpal

## Project Goals

As a Windows Rust executable, when "cde" is specified as a command-line argument, it should play Do-Re-Mi in real-time.

## Anticipated Implementations

- File output of intermediate representation (including Standard MIDI Files)

## Out of Scope

- Complex MML
- Real-time MIDI message output
- Effects (e.g., LPF, overdrive/distortion, delay)
- GUI editor

## Related Projects

### cat-play-chord Under Consideration

[cat-play-chord](https://github.com/cat2151/cat-play-chord) is a project that generates MML from chord notation using chord2mml and plays it (under consideration).

### Future Outlook for MML to SMF
- Status
  - Only "cde" has been implemented
    - Goal: Initially focusing on minimal implementation to streamline problem-solving until real-time Do-Re-Mi playback is achieved.
  - Using SMF (Standard MIDI File)
    - Goal: Using SMF makes verification and development easier, reducing the risk of project stagnation.
- MML dialect is expected to be mmlabc, as there is existing know-how and a clear format.
- Intended to proceed with a TDD agent; will consider alternatives if hallucinations occur.

### smf to Nuked-OPM friendly JSON Under Consideration
- Example
  - Delay Vibrato
    - Based on the 'tone modify delay vibrato' value in 'tone settings toml',
      - From an OPM sound driver perspective,
        - Generate a soft LFO register event for each tick.
    - Separating SMF and TOML is intended to simplify the MIDI implementation on the SMF side.
      - This makes destructive changes to timbre and OPM sound driver-like processing easier on the TOML side (ETC principle).
- The following passes are envisioned:
  - SMF to Intermediate Representation (assuming JSON that represents SMF as text)
  - Intermediate Representation to Intermediate Representation (n times, delay vibrato is anticipated here)
  - Intermediate Representation to Nuked-OPM friendly JSON
- Intended to proceed with a Linux Python TDD agent; will consider alternatives if hallucinations occur.

### Nuked-OPM friendly JSON player
- Implemented (log player in a separate repository)
- Purpose: To facilitate development
  - Makes debugging easier and reduces the risk of project stagnation.

### Real-time FM tone editor (Provisional) Under Consideration
- Purpose of writing this here
  - Rubber ducking
- Purpose
  - For verification
- Priority
  - Prioritize ease of development and the ability to quickly sketch timbres with minimal operations.
- Operations
  - Right hand: Increase/decrease values assigned to mouse x,y respectively.
  - Left hand: WASD+QE for cursor movement, SPACE for confirmation, ESC for cancel, Z for UNDO (details are fuzzy).
  - x,y switch between DecayRate, ModulatorTL, FB, MUL via cursor movement.
  - WASD+QE, UNDO are fuzzy.
  - Instead of cursor movement and confirmation, pressing a specific key to instantly apply effects and achieve fast editing might be better for editing speed, so that will also be tried (e.g., via a settings TOML file).
- Sound
  - Switch between pluck and long tone timbres via cursor movement.
  - OP connection algorithm is 2-op parallel, detune has 4 different values.
- Display
  - Windows TUI 80x24
  - Timbre parameter display in mdx note.x format.
  - During runtime, save timbre to clipboard in mdx note.x format 1 second after the last value change; output will be limited to this (provisional).
- All specifications are provisional, destructive changes will be frequent, prioritizing ease of development.
- Even this is still too many specifications (too many to start small), so we will start smaller with a provisional implementation based on more focused specifications.

## License

This project is released under the [MIT License](LICENSE).

*The English README.md is automatically generated by GitHub Actions using Gemini's translation based on README.ja.md