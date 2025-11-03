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

The basic implementation of the Windows Rust executable is complete. You can play music by specifying MML as a command-line argument.

## Usage

```bash
# Play "Do-Re-Mi"
cat-play-mml.exe cde

# More complex MML
cat-play-mml.exe "t120 o4 l4 cdefgab>c"

# Show help
cat-play-mml.exe --help
```

## Overview

`cat-play-mml` is a multi-language project that parses Music Macro Language (MML) and plays music. It uses tree-sitter to parse MML into an AST, converts it to an intermediate representation, and then plays the music.

### Key Features

- **MML Parser**: tree-sitter-based MML grammar definition
- **Low Latency**: Real-time music playback
- **Simple Command-line Arguments**: Play "Do-Re-Mi" by simply passing "cde" as an argument

## What is MML (Music Macro Language)?

MML is a language for describing music using text. It uses notation such as:

- `c`, `d`, `e`, `f`, `g`, `a`, `b`: Musical notes (Do, Re, Mi, Fa, Sol, La, Si)
- `o4`: Octave setting (4th octave)
- `l4`: Note length setting (quarter note)
- `t120`: Tempo setting (BPM 120)
- `<`, `>`: Raise/lower octave
- `+`, `-`: Semitone up/down
- `r`: Rest

## Technical Details

### Architecture

1.  **Parser**: Uses tree-sitter to convert MML text into an AST
2.  **Intermediate Representation**: Converts the AST into a music data structure
3.  **Audio Generation**: Generates audio waveforms from the intermediate representation
4.  **Playback**: Outputs audio using an audio library

### Development Environment

- Windows
- Rust
- Zig cc (mingw and msys2 are prohibited)
- Linux runner for agent TDD (as long as the agent can perform TDD) + ALSA SDK and settings (to enable TDD in a headless environment)

### Audio Libraries Used

- **Rust**: cpal

## Project Goals

- As a Windows Rust executable, when "cde" is specified as a command-line argument, it should play "Do-Re-Mi" in real-time.

## Anticipated Implementations

- File output of intermediate representation (including Standard MIDI Files)

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
  - Only "cde" has been implemented.
    - Goal: Initially focusing on minimal implementation to smooth problem-solving until "Do-Re-Mi" plays in real-time.
  - Using SMF (Standard MIDI File)
    - Goal: Using SMF makes verification and development easier, reducing the risk of development stalling.
  - Assumed MML dialect is mmlabc, as there is expertise and the format is clear.
  - Planned to proceed with a TDD agent; will consider if hallucinations occur.

### SMF to Nuked-OPM friendly JSON (Under Consideration)
- Example
  - Delay Vibrato
    - Based on the 'tone modify delay vibrato' value in tone settings TOML,
      - From an OPM sound driver perspective,
        - Generate a soft LFO register event for each tick.
    - Separating SMF and TOML is intended to simplify the MIDI implementation on the SMF side.
      - This makes it easier to implement destructive changes to timbres and OPM sound driver-like processing on the TOML side (ETC principle).
- The following passes are anticipated:
  - SMF to Intermediate Representation (assuming SMF expressed as text in JSON)
  - Intermediate Representation to Intermediate Representation (n times, delay vibrato is anticipated here)
  - Intermediate Representation to Nuked-OPM friendly JSON
- Planned to proceed with a Linux Python TDD agent; will consider if hallucinations occur.

### Nuked-OPM friendly JSON Player
- Implemented (log player in a separate repository)
- Purpose: To facilitate development
  - Easier to debug, reducing the risk of development stalling.

### Real-time FM Tone Editor (Provisional, Under Consideration)
- Purpose of writing this here
  - Rubber Ducking
- Usage
  - For verification
- Priority
  - Prioritize ease of development and the ability to quickly sketch timbres with minimal operations.
- Operations
  - Right hand: Mouse x, y for increasing/decreasing assigned values respectively.
  - Left hand: WASD+QE for cursor movement, SPACE for confirmation, ESC for cancellation, Z for UNDO (vague).
  - x, y switch between DecayRate, ModulatorTL, FB, MUL with cursor movement.
  - WASD+QE, UNDO are vague.
  - Instead of cursor movement and confirmation, try a method where pressing a specific key immediately applies an effect for faster editing (e.g., via a settings TOML file), as this might increase editing speed.
- Sound
  - Switch between pluck and long tone timbres with cursor movement.
  - OP connection algorithm is 2-op parallel, detune has 4 distinct values.
- Display
  - Windows TUI 80x24
  - Timbre parameter display in mdx note.x format.
  - While running, save the timbre in mdx note.x format to the clipboard 1 second after the last numerical change; limit output to this only (provisional).
- All specifications are provisional, destructive changes will be frequent, prioritizing ease of development.
- Even this is still too many specifications (too many to start small), so start small with a more focused provisional implementation.

## License

This project is released under the [MIT License](LICENSE).

*The English README.md is automatically generated by GitHub Actions using Gemini's translation based on README.ja.md.