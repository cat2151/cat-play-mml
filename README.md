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

## Usage

```bash
# Play C-D-E
cat-play-mml.exe cde

# More complex MML
cat-play-mml.exe "t120 o4 l4 cdefgab>c"

# Show help
cat-play-mml.exe --help
```

## Overview

`cat-play-mml` is a multi-language project that parses Music Macro Language (MML) and plays music. It uses tree-sitter to parse MML into an AST, converts it into an intermediate representation, and then plays the music.

### Key Features

- **MML Parser**: tree-sitter based MML grammar definition
- **Low Latency**: Real-time music playback
- **Simple Command-Line Arguments**: Play C-D-E by simply passing "cde" as an argument

## What is MML (Music Macro Language)?

MML is a language for describing music using text. It uses notations such as:

- `c`, `d`, `e`, `f`, `g`, `a`, `b`: Notes (Do, Re, Mi, Fa, Sol, La, Si)
- `o4`: Octave setting (4th octave)
- `l4`: Note length setting (quarter note)
- `t120`: Tempo setting (BPM 120)
- `<`, `>`: Octave up/down
- `+`, `-`: Semitone up/down
- `r`: Rest

## Technical Details

### Architecture

1.  **Parser**: Converts MML text to an AST using tree-sitter
2.  **Intermediate Representation**: Converts the AST into a music data structure
3.  **Audio Generation**: Generates audio waveforms from the intermediate representation
4.  **Playback**: Outputs audio using an audio library

### Development Environment

- Windows
- Rust
- Zig cc (mingw and msys2 are prohibited)
- Linux runner for agent TDD (as long as the agent can do TDD) + ALSA SDK and settings (to enable TDD in a headless environment)

### Audio Libraries Used

- **Rust**: cpal

## Project Goals

- As a Windows Rust executable, when `cde` is specified as a command-line argument, real-time playback of C-D-E should occur.

## Expected Implementations

- Output of intermediate representation to file (including Standard MIDI File)

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
  - Only C-D-E has been implemented.
    - Aim: Initially focus on minimal implementation to smoothly resolve issues until real-time playback of C-D-E is achieved.
  - Using SMF (Standard MIDI File).
    - Aim: Using SMF makes verification and development easier, reducing the risk of development stalling.
- MML dialect is assumed to be mmlabc, due to existing know-how and clear format.
- Intended to proceed with TDD agent; if hallucinations occur, they will be investigated.

### SMF to Nuked-OPM friendly JSON (Under Consideration)
- Example
  - Delay Vibrato
    - Based on the `tone modify delay vibrato` value in the tone settings toml,
      - From an OPM sound driver perspective,
        - Generate a soft LFO register event for each tick.
    - Separating SMF and toml is for simplifying the MIDI implementation on the SMF side.
      - The toml side makes it easier to make destructive changes to tone colors and OPM sound driver-like processing, following the ETC principle.
- The following passes are envisioned:
  - SMF to Intermediate Representation (json representing SMF as text is assumed)
  - Intermediate Representation to Intermediate Representation (n times) (delay vibrato is assumed here)
  - Intermediate Representation to Nuked-OPM friendly JSON
- Intended to proceed with Linux Python TDD agent; if hallucinations occur, they will be investigated.

### Nuked-OPM friendly JSON player
- Already implemented (log player in a separate repository)
- Purpose is to facilitate development
  - Easier debugging, reduces the risk of development stalling.

### Real-time FM tone editor (Tentative) - Under Consideration
- Purpose of writing this here:
  - Rubber ducking
- Use case:
  - Verification
- Priority:
  - Prioritize ease of development and quick sketching of tone colors with minimal operations.
- Operations:
  - Right hand: Mouse x,y for increasing/decreasing assigned values respectively.
  - Left hand: WASD+QE for cursor movement, SPACE for confirmation, ESC for cancel, Z for UNDO (this is vague).
  - x,y switches between DecayRate, ModulatorTL, FB, MUL with cursor movement.
  - WASD+QE, UNDO are vague.
  - Instead of cursor movement and confirmation, trying a single key press for immediate effect and fast editing might be faster; this will also be explored, possibly with a settings toml file.
- Sound:
  - Tone colors switch between pluck and long tone with cursor movement.
  - OP connection algorithm is 2-op parallel, detune has 4 different values.
- Display:
  - Windows TUI 80x24
  - Tone parameter display in mdx note.x format.
  - While running, save tone color to clipboard in mdx note.x format 1 second after the last numerical change; output only this (tentative).
- All specifications are tentative, frequent destructive changes will occur, prioritizing ease of development.
- Even this still has too many specifications (too many to start small), so begin with a smaller, more focused tentative implementation.

## License

This project is released under the [MIT License](LICENSE).

*The English README.md is automatically generated from README.ja.md by GitHub Actions using Gemini's translation.*