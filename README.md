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

`cat-play-mml` is a CLI tool that plays music using Music Macro Language (MML). If you input the string `cde`, it will play the notes `Do-Re-Mi`. It is designed for Windows and written in Rust.

## Quickstart Guide

### Setup
- Install `Rust` and `Zig` on Windows.

### Installation
```
cargo install --git https://github.com/cat2151/cat-play-mml.git --branch main --bin cat-play-mml
```

That's it! It will be installed on your Windows machine directly from GitHub.

### Playback
```
cat-play-mml cde
```

Do-Re-Mi will play.

## Future Issues for Related Projects

- [mmlabc-to-smf-rust](https://github.com/cat2151/mmlabc-to-smf-rust/issues):
  - Implement MML `;`. `c;e;g` will form a C major chord. To allow separate timbres for each note in the future, it should be ch1 C, ch2 E, ch3 G (1-based notation) rather than ch1 C-E-G.
  - Output MIDI Program Change 0 (0-based notation) at the beginning of each SMF channel. Do not output to channels without note events.

- [smf-to-ym2151log-rust](https://github.com/cat2151/smf-to-ym2151log-rust/issues)
  - Support multiple channels with the implementation of MML `;`. Chords within a channel will be considered separately; for now, undefined behavior is acceptable.
  - MIDI Program Change 0 (0-based notation) should use an acoustic grand piano-like timbre. More specifically, anything but a sine wave would suffice.

- [ym2151-log-play-server](https://github.com/cat2151/ym2151-log-play-server/issues)

### Key Features

- **Simple, instant playback**: Just pass "cde" as an argument to play Do-Re-Mi.
- **Low Latency**: Real-time music playback.
- **Background Playback**: Perform other operations while music plays in server mode.

### Usage

#### Basic Usage (Automatic Server Startup)

```
cat-play-mml cde
```

On the first run, the server will automatically start, and playback will begin in the background. The command will exit immediately, allowing you to enter the next command.

For subsequent runs, playback will be sent to the already running server:

```
cat-play-mml efg
```

#### Server Control

To stop playback:

```
cat-play-mml --stop
```

To shut down the server:

```
cat-play-mml --shutdown
```

#### Manual Server Startup (Advanced Users)

```
cat-play-mml --server
```

## What is MML (Music Macro Language)?

MML is a language used to describe music in text. It uses notation such as:

- `c`, `d`, `e`, `f`, `g`, `a`, `b`: Musical scale (Do, Re, Mi, Fa, Sol, La, Si)
- `o4`: Octave setting (4th octave)
- `l4`: Note length setting (quarter note)
- `t120`: Tempo setting (BPM 120)
- `<`, `>`: Octave up/down
- `+`, `-`: Semitone up/down
- `r`: Rest

## Technical Details

### Architecture

1. **Parser**: Uses tree-sitter to convert MML text into an AST.
2. **Intermediate Representation**: Converts the AST into a music data structure.
3. **Audio Generation**: Generates audio waveforms from the intermediate representation.
4. **Playback**: Outputs audio using an audio library.

### Development Environment

- Windows
- Rust
- Zig cc (mingw and msys2 are prohibited)
- Linux runner for agent TDD (as long as the agent can do TDD) + ALSA SDK and configuration (to enable TDD in a headless environment).

### Libraries Used

- Nuked-OPM
- **Rust**: cpal

## Project Goals

### Minor Goals
- [x] As a Windows Rust .exe, when "cde" is specified as a command-line argument, Do-Re-Mi plays in real-time.

### Next Goals
- mmlabc grammar
  - Priority:
    - `;`
- Intermediate representation file output (including Standard MIDI files)

## Out of Scope

- Complex MML
- Real-time MIDI message output
- Effects (e.g., LPF, overdrive/distortion, delay)
- GUI editor → For TUI editors, please refer to `cat-edit-mml` and `ym2151-tone-editor`.

## Related Projects

### cat-play-chord (Under Consideration)

[cat-play-chord](https://github.com/cat2151/cat-play-chord) is a project that generates and plays MML from chord notation using chord2mml (under consideration).

### Future Outlook for MML to SMF
- Status
  - Only "cde" has been implemented.
    - Goal: Initially, focusing on minimal implementation to streamline problem-solving until Do-Re-Mi plays in real-time.
  - Using SMF
    - Goal: Using SMF makes verification and development easier, reducing the risk of stalled development.
- Expected to use mmlabc dialect, as there is expertise and the format is clear.
- Expected to proceed with a TDD agent; will review if hallucinations occur.

### SMF to Nuked-OPM friendly JSON (Under Consideration)
- Example
  - Delay Vibrato
    - Based on the `tone modify delay vibrato` value in `tone settings toml,`
      - From an OPM sound driver perspective,
        - Generate soft LFO register events every tick.
    - Separating SMF and toml is intended to simplify the MIDI implementation on the SMF side.
      - This makes it easier to make destructive changes to timbre and OPM sound driver-like processing on the toml side (ETC principle).
- The following passes are envisioned:
  - SMF to intermediate representation *assuming SMF represented as text JSON*
  - Intermediate representation to intermediate representation *n times* *delay vibrato is envisioned here*
  - Intermediate representation to Nuked-OPM friendly JSON
- Expected to proceed with a Linux Python TDD agent; will review if hallucinations occur.

### Nuked-OPM friendly JSON Player
- Implemented as a library: `ym2151-log-play-server`
  - This application also uses this library.
- Its purpose is to facilitate development.
  - Easier to debug, reducing the risk of stalled development.

### Real-time FM Tone Editor
- Implemented: `ym2151-tone-editor`

#### Idea, Draft
- The following is a draft and differs from the already implemented specifications. It is placed here for now.
- Purpose of writing this here:
  - Rubber ducking
- Usage:
  - For verification
- Priority:
  - Prioritize ease of development and rough sketching of timbres with minimal operations.
- Operations:
  - Right hand: Increase/decrease values assigned to mouse x,y respectively.
  - Left hand: WASD+QE for cursor movement, SPACE for confirmation, ESC for cancellation, Z for UNDO (this is still vague).
  - x,y switch between DecayRate, ModulatorTL, FB, MUL with cursor movement.
  - WASD+QE, UNDO are vague.
  - Instead of cursor movement and confirmation, pressing a specific key to instantly apply effects for faster editing might be quicker, so that will also be tested, perhaps with a settings toml file.
- Sound:
  - Switch between pluck and long tone timbres with cursor movement.
  - OP connection algorithm is 2-op parallel, detune has 4 different values.
- Display:
  - Windows TUI 80x24
  - Timbre parameter display is in mdx note.x format.
  - While running, save the timbre to the clipboard in mdx note.x format 1 second after the last value change; temporarily, output will be limited to this.
- All specifications are temporary, destructive changes will be frequent, prioritizing ease of development.
- Even this is still too many specifications (too much for a small start), so start small with a temporary implementation with even more focused specifications.

#### Idea, Draft
- The following is a draft and differs from the already implemented specifications. It is placed here for now.
- Output: Plays a note with each key press.
  - Internally, it plays MML.
  - First, verify with an extremely simple implementation.
- Purpose: For verifying the provision of a timbre creation experience, which is one of the essential pleasures of a synthesizer.
  - While various specifications are necessary to provide a better success experience, these will be excluded from the initial implementation (otherwise, it becomes endless and gets lost).
    - Start small. One step at a time. That is global optimization.
- Create a separate repository as an independent TUI application. Start small.

### cat-edit-mml
- Implemented
- mmlabc editor, playback engine is ym2151-log-play-server
- Output: Plays a note with each key press.
- Purpose: To provide the experience of "Do" playing when "c" is pressed.
- Create a separate repository as an independent TUI application. Start small.

## Appendix

### Build, Install, and Run for Developers

```powershell
# Build & Run *in the cloned directory*
cargo run --release cegb

# Install *in the cloned directory*
cargo install --path .

# Run *Once installed, you can run from any directory like this*
cat-play-mml cegb
```

## License

This project is released under the [MIT License](LICENSE).

*The English README.md is automatically generated by GitHub Actions using Gemini's translation based on README.ja.md.