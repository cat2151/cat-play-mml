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

`cat-play-mml` is a CLI tool that plays music using Music Macro Language (MML). Inputting the string `cde` will play the music "do-re-mi". It's for Windows and written in Rust.

## Status
- Breaking changes occur frequently. Many MML commands are not yet implemented. README maintenance is in progress and will be advanced in the future.

## Features

- 100ms
  - Sound plays 100ms to 500ms after executing the CLI.
    - Just specify `"cde"` as a command-line argument!

## Quick Start Guide

### Environment Setup
- Install `Rust` on Windows.

### Installation
```
cargo install --git https://github.com/cat2151/cat-play-mml.git --branch main --bin cat-play-mml
```

That's it! It will be installed from GitHub to your Windows machine.

### Playback
```
cat-play-mml cde
```

"Do-re-mi" will play.

## Future plans for related project issues

- [mmlabc-to-smf-rust](https://github.com/cat2151/mmlabc-to-smf-rust/issues):
  - Implement MML `;`. `c;e;g` will form a C major chord. Each note should be assignable to a different timbre in the future, meaning ch1 C, ch2 E, ch3 G (1-based notation), not ch1 C major chord.
  - Output MIDI Program Change 0 (0-based notation) at the beginning of each SMF channel. Do not output for channels without note data.

- [smf-to-ym2151log-rust](https://github.com/cat2151/smf-to-ym2151log-rust/issues)
  - Support multiple channels due to MML `;` implementation. Chords within a channel will be considered separately and are currently undefined behavior.
  - MIDI Program Change 0 (0-based notation) will represent an acoustic grand piano-like timbre. More specifically, it should just not be a sine wave.

- [ym2151-log-play-server](https://github.com/cat2151/ym2151-log-play-server/issues)

### Key Features

- **Simple, instant playback**: Just pass "cde" as an argument to play "do-re-mi".
- **Low latency**: Real-time music playback.
- **Background playback**: Play music in server mode while performing other operations.

### Usage

#### Basic Usage (Automatic server startup)

```
cat-play-mml cde
```

The first time you run it, the server will automatically start, and playback will begin in the background. The command will exit immediately, allowing you to enter the next command.

For subsequent executions, it sends playback data to the already running server:

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

#### Manual Server Startup (Advanced)

```
cat-play-mml --server
```

## What is MML (Music Macro Language)?

MML is a language for describing music using text. It uses notations such as:

- `c`, `d`, `e`, `f`, `g`, `a`, `b`: Scale notes (C, D, E, F, G, A, B)
- `o4`: Octave setting (4th octave)
- `l4`: Note length setting (quarter note)
- `t120`: Tempo setting (BPM 120)
- `<`, `>`: Raise/lower octave
- `+`, `-`: Raise/lower by a semitone
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
- Linux runner for agent TDD (as long as the agent can do TDD) + ALSA SDK and configuration (to enable TDD in a headless environment)

### Libraries Used

- Nuked-OPM
- **Rust**: cpal

## Project Goals

### Priority Goals
- [x] Provide the experience of running `cat-play-mml "cde"` as a Windows Rust exe, playing "do-re-mi" in real-time.

### Next Goals
- mmlabc grammar
  - Priority
    - `;`
- File output of intermediate representation (including Standard MIDI File)

## Out of Scope

- Complex MML
- Real-time MIDI message output
- Effects (LPF, overdrive/distortion, delay, etc.)
- GUI editor → For TUI editors, please refer to `cat-edit-mml` and `ym2151-tone-editor`.

## Related Projects

### cat-play-chord (Under consideration)

[cat-play-chord](https://github.com/cat2151/cat-play-chord) is a project (under consideration) that generates and plays MML from chord notation using chord2mml.

### Future Outlook for MML to SMF
- Status
  - Only `cde` implemented.
    - Goal: Initially focus on minimal implementation to smoothly resolve issues until real-time "do-re-mi" playback is achieved.
  - Using SMF.
    - Goal: Using SMF makes verification and development easier, reducing the risk of development stalling.
- MML dialect is assumed to be mmlabc, as there is existing know-how and a clear format.
- Expected to proceed with TDD agent; if hallucinations occur, they will be investigated.

### SMF to Nuked-OPM friendly JSON (Under consideration)
- Example
  - Delay vibrato
    - Based on the `tone modify delay vibrato` value in `tone settings toml`,
      - From an OPM sound driver perspective,
        - Generate soft LFO register events per tick.
    - Separating SMF and toml is for simplifying MIDI implementation on the SMF side.
      - It makes it easier to introduce breaking changes for timbres and OPM sound driver-like processing on the toml side, following the ETC principle.
- The following passes are envisioned:
  - SMF to intermediate representation (assuming JSON representing SMF as text).
  - Intermediate representation to intermediate representation (n times) - delay vibrato is envisioned here.
  - Intermediate representation to Nuked-OPM friendly JSON.
- Expected to proceed with Linux Python TDD agent; if hallucinations occur, they will be investigated.

### Nuked-OPM friendly JSON player
- Already implemented as a library: `ym2151-log-play-server`
  - This application also uses this library.
- Purpose: To facilitate development.
  - Easier debugging, reduces the risk of development stalling.

### Real-time FM tone editor
- Implemented: `ym2151-tone-editor`

#### Proposal, Draft
- The following is a draft and differs from the implemented specifications; for now, it's just kept here.
- Purpose of writing here
  - Rubber ducking
- Use case
  - For verification
- Priority
  - Prioritize ease of development and quick sketching of timbres with minimal operations.
- Operations
  - Right hand: Mouse x,y for increasing/decreasing assigned values respectively.
  - Left hand: WASD+QE for cursor movement, SPACE for confirm, ESC for cancel, Z for UNDO (this part is fuzzy).
  - x,y for DecayRate, ModulatorTL, FB, MUL, switchable with cursor movement.
  - WASD+QE, UNDO are fuzzy.
  - Instead of cursor movement and confirmation, pressing a specific key to instantly apply an effect for faster editing will also be tried, perhaps via a configuration toml file.
- Sound
  - Switch between pluck and long tone timbres with cursor movement.
  - OPM connection algorithm is 2op parallel, detune has 4 different values.
- Display
  - Windows TUI 80x24
  - Timbre parameter display in mdx note.x format.
  - While running, save timbre to clipboard in mdx note.x format 1 second after the last numerical change; output is limited to this for now.
- All specifications are temporary, frequent breaking changes will occur, prioritizing ease of development.
- Even this is still too many specifications (too many to start small), so start small with a temporary implementation with even more limited specifications.

#### Proposal, Draft
- The following is a draft and differs from the implemented specifications; for now, it's just kept here.
- Output: Play each time a key is pressed.
  - Internally, it plays MML.
  - First, verify with an extremely simple implementation.
- Use case: For verifying the experience of timbre creation, one of the highlights of a synthesizer.
  - While many specifications are needed to provide a better success experience, these are excluded from the initial implementation (otherwise, it becomes endless and gets lost).
    - Start small. Do it step by step. That's optimal overall.
- Make it a separate repository as an independent TUI application. Start small.

### cat-edit-mml
- Implemented
- mmlabc editor, playback engine is ym2151-log-play-server.
- Output: Play each time a key is pressed.
- Use case: Provide the experience of playing "do" when 'c' is pressed.
- Make it a separate repository as an independent TUI application. Start small.

## Notes

### For Developers: How to Build, Install, and Run

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

*The English README.md is automatically generated by GitHub Actions using Gemini's translation of README.ja.md.*