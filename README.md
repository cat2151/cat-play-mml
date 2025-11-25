# cat-play-mml

<p align="left">
  <a href="README.ja.md"><img src="https://img.shields.io/badge/🇯🇵-Japanese-red.svg" alt="Japanese"></a>
  <a href="README.md"><img src="https://img.shields.io/badge/🇺🇸-English-blue.svg" alt="English"></a>
</p>

🎵 Music Macro Language (MML) Parser and Player

## Quick Links
| Item | Link |
|------|--------|
| 📊 Development Status | [generated-docs/development-status](generated-docs/development-status.md) |

## Overview

`cat-play-mml` is a CLI tool that plays music using Music Macro Language (MML). If you input the string `cde`, it will play the notes `CDE` (Do-Re-Mi). It is for Windows and is written in Rust.

## Quick Start Guide

### Setup
- Install `Rust` on Windows.

### Installation
```
cargo install --git https://github.com/cat2151/cat-play-mml.git --branch main --bin cat-play-mml
```

That's it! It will be installed on your Windows machine directly from GitHub.

### Playback
```
cat-play-mml cde
```

CDE (Do-Re-Mi) will play.

## Future Plans for Related Project Issues

- [mmlabc-to-smf-rust](https://github.com/cat2151/mmlabc-to-smf-rust/issues):
  - Implement MML `;`. `c;e;g` will represent a C major chord. To allow different timbres to be assigned in the future, it should be ch1 C, ch2 E, ch3 G (1-based notation) rather than ch1 C E G.
  - Output MIDI Program Change 0 (0-based notation) at the beginning of each SMF channel. Do not output to channels without note output.

- [smf-to-ym2151log-rust](https://github.com/cat2151/smf-to-ym2151log-rust/issues)
  - Support multiple channels following the implementation of MML `;`. Chords within a channel will be considered separately; for now, undefined behavior is acceptable.
  - MIDI Program Change 0 (0-based notation) should use an acoustic grand piano-like timbre. More specifically, it should simply not be a sine wave.

- [ym2151-log-play-server](https://github.com/cat2151/ym2151-log-play-server/issues)

### Key Features

- **Simple, instant playback**: Just pass "cde" as an argument to play CDE (Do-Re-Mi).
- **Low latency**: Real-time music playback.
- **Background playback**: Perform other operations while music plays in server mode.

### Usage

#### Basic Usage (Automatic Server Start)

```
cat-play-mml cde
```

On the first run, the server automatically starts, and playback begins in the background. The command exits immediately, allowing you to enter the next command.

For subsequent runs, playback is sent to the already running server:

```
cat-play-mml efg
```

#### Server Control

Stop playback:

```
cat-play-mml --stop
```

Shut down the server:

```
cat-play-mml --shutdown
```

#### Manual Server Start (Advanced Users)

```
cat-play-mml --server
```

## What is MML (Music Macro Language)?

MML is a language for describing music using text. It uses notations such as:

- `c`, `d`, `e`, `f`, `g`, `a`, `b`: Musical scale notes (Do, Re, Mi, Fa, Sol, La, Si).
- `o4`: Octave setting (4th octave).
- `l4`: Note length setting (quarter note).
- `t120`: Tempo setting (BPM 120).
- `<`, `>`: Octave up/down.
- `+`, `-`: Semitone up/down.
- `r`: Rest.

## Technical Details

### Architecture

1. **Parser**: Converts MML text to AST using tree-sitter.
2. **Intermediate Representation**: Converts AST to music data structure.
3. **Audio Generation**: Generates audio waveforms from the intermediate representation.
4. **Playback**: Outputs audio using an audio library.

### Development Environment

- Windows
- Rust
- Linux runner for agent TDD (as long as the agent can perform TDD) + ALSA SDK and configuration (to enable TDD in a headless environment).

### Libraries Used

- Nuked-OPM
- **Rust**: cpal

## Project Goals

### Minor Goals
- [x] As a Windows Rust executable, when `cde` is specified as a command-line argument, real-time playback of CDE (Do-Re-Mi) should occur.

### Next Goals
- mmlabc grammar
  - Priority
    - `;`
- Intermediate representation file output (including Standard MIDI Files).

## Out of Scope

- Complex MML
- Real-time MIDI message output
- Effects (LPF, overdrive/distortion, delay, etc.)
- GUI editor → For TUI editors, please refer to `cat-edit-mml` and `ym2151-tone-editor`.

## Related Projects

### cat-play-chord (Under Consideration)

[cat-play-chord](https://github.com/cat2151/cat-play-chord) is a project that generates and plays MML from chord notation using chord2mml (under consideration).

### Future Outlook for MML to SMF
- Status
  - Only CDE (Do-Re-Mi) has been implemented.
    - Goal: Initially focus on minimal implementation to smooth out problem-solving until real-time playback of CDE (Do-Re-Mi) is achieved.
  - Using SMF (Standard MIDI File).
    - Goal: Using SMF makes verification and development easier, reducing the risk of stalled development.
- MML dialect is expected to be mmlabc, as there is existing know-how and a clear format.
- Development is expected to proceed with a TDD agent; hallucinations will be addressed if they occur.

### SMF to Nuked-OPM friendly JSON (Under Consideration)
- Example
  - Delay Vibrato
    - Based on the `tone modify delay vibrato` value in `tone settings toml`,
      - From an OPM sound driver perspective,
        - Generate soft LFO register events every tick.
    - Separating SMF and toml is for simplifying the MIDI implementation on the SMF side.
      - This makes it easier to make breaking changes to timbres and OPM sound driver-like processing on the toml side, following the ETC principle.
- The following passes are envisioned:
  - SMF to Intermediate Representation *Assumes JSON representing SMF as text.
  - Intermediate Representation to Intermediate Representation *n times *Delay vibrato is envisioned here.
  - Intermediate Representation to Nuked-OPM friendly JSON.
- Development is expected to proceed with a Linux Python TDD agent; hallucinations will be addressed if they occur.

### Nuked-OPM friendly JSON player
- Implemented as a library: `ym2151-log-play-server`.
  - This application also uses this library.
- Purpose: To facilitate development.
  - Easier to debug and reduces the risk of stalled development.

### Real-time FM Tone Editor
- Implemented: `ym2151-tone-editor`.

#### Draft Ideas
- The following is a draft and differs from the implemented specifications; it is temporarily placed here.
- Purpose of writing here
  - Rubber duck debugging.
- Purpose
  - For verification.
- Priority
  - Prioritize ease of development and the ability to quickly sketch timbres with minimal operations.
- Operation
  - Right hand: Increase/decrease values assigned to mouse x,y respectively.
  - Left hand: WASD+QE for cursor movement, SPACE to confirm, ESC to cancel, Z for UNDO (this is still vague).
  - x,y switch between DecayRate, ModulatorTL, FB, MUL with cursor movement.
  - WASD+QE, UNDO are vague.
  - Instead of cursor movement and confirmation, pressing a specific key to instantly apply an effect and enable fast editing might increase editing speed, so that will also be tested (e.g., using a configuration TOML file).
- Sound
  - Switch between pluck and long tone timbres with cursor movement.
  - OP connection algorithm is 2-op parallel, detune has 4 different values.
- Display
  - Windows TUI 80x24.
  - Timbre parameter display in mdx note.x format.
  - While running, save timbre in mdx note.x format to clipboard 1 second after the last value change; limit output to only this (temporary).
- All specifications are temporary, frequent breaking changes will occur, prioritizing ease of development.
- Even this is still too many specifications (too many to start small), so begin small with a temporary implementation based on a more focused set of specifications.

#### Draft Ideas
- The following is a draft and differs from the implemented specifications; it is temporarily placed here.
- Output: Play each time a key is pressed.
  - Internally, it plays MML.
  - First, verify with an extremely simple implementation.
- Purpose: For verification, to provide an experience of timbre creation, one of the highlights of a synthesizer.
  - While various specifications are necessary to provide a better success experience, these will be excluded from the initial implementation (otherwise, it becomes endless and unfocused).
    - Start small. Take one step at a time. That's global optimization.
- Separate into an independent TUI application repository. Start small.

### cat-edit-mml
- Implemented.
- mmlabc editor, playback engine is ym2151-log-play-server.
- Output: Plays each time a key is pressed.
- Purpose: To provide the experience of "C" playing "Do" when the 'C' key is pressed.
- Separate into an independent TUI application repository. Start small.

## Appendix

### Build, Install, and Run for Developers

```powershell
# Build & Run *In the cloned directory
cargo run --release cegb

# Install *In the cloned directory
cargo install --path .

# Run *Once installed, you can run it from any directory like this
cat-play-mml cegb
```

## License

This project is released under the [MIT License](LICENSE).

*The English README.md is automatically generated by GitHub Actions using Gemini's translation based on README.ja.md.