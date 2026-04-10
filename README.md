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

`cat-play-mml` is a CLI tool that plays music using Music Macro Language (MML). If you input the string `cde`, it will play the notes `Do-Re-Mi`. It is for Windows and written in Rust.

## Status
- Breaking changes occur frequently. Many MML commands are unimplemented. The README maintenance is in progress and will be advanced in the future.

## Features

- 100ms
  - Sound plays within 100ms to 500ms after executing the CLI.
    - Just specify `"cde"` as a command-line argument!

## Quick Start Guide

### Environment Setup
- Please install `Rust` on Windows.

### Installation
```
cargo install --git https://github.com/cat2151/cat-play-mml.git --branch main --bin cat-play-mml
```
That's it! It will be installed on your Windows from GitHub.

### Playing Music
```
cat-play-mml cde
```
Do-Re-Mi will play.

## Future Plans (Potential Issues for Related Projects)

- [mmlabc-to-smf-rust](https://github.com/cat2151/mmlabc-to-smf-rust/issues)：
  - Implement MML `;`. `c;e;g` will form a C major chord. To allow assigning different timbres in the future, it should be ch1 C, ch2 E, ch3 G (1-based notation) instead of ch1 C-E-G.
  - Output MIDI Program Change 0 (0-based notation) at the beginning of each SMF channel. Do not output to channels without note output.

- [smf-to-ym2151log-rust](https://github.com/cat2151/smf-to-ym2151log-rust/issues)
  - Support multiple channels with the implementation of MML `;`. Chords within a channel will be considered separately; for now, undefined behavior is acceptable.
  - MIDI Program Change 0 (0-based notation) should result in an acoustic grand piano-like timbre. More specifically, anything other than a sine wave is acceptable.

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

Check for new builds:

```
cat-play-mml check
```

Start self-update:

```
cat-play-mml update
```

#### Manual Server Startup (Advanced Users)

```
cat-play-mml --server
```

## What is MML (Music Macro Language)?

MML is a language for describing music in text. It uses the following notation:

- `c`, `d`, `e`, `f`, `g`, `a`, `b`: Scale notes (Do, Re, Mi, Fa, Sol, La, Si)
- `o4`: Octave setting (4th octave)
- `l4`: Note length setting (quarter note)
- `t120`: Tempo setting (BPM 120)
- `<`, `>`: Octave up/down
- `+`, `-`: Semitone up/down
- `r`: Rest

## Technical Details

### Architecture

1. **Parser**: Converts MML text to AST using tree-sitter.
2. **Intermediate Representation**: Converts AST to musical data structure.
3. **Audio Generation**: Generates audio waveforms from the intermediate representation.
4. **Playback**: Outputs audio using an audio library.

### Development Environment

- Windows
- Rust
- Linux runner for agent's TDD (as long as the agent can do TDD) + ALSA SDK and setup (to enable TDD in a headless environment).

### Libraries Used

- Nuked-OPM
- **Rust**: cpal

## Project Goals

### Top Priority Goals
- [x] Provide the experience of playing Do-Re-Mi in real-time by executing `cat-play-mml "cde"` as a Windows Rust executable.

### Next Goals
- mmlabc grammar
  - Priority
    - `;`
- Intermediate representation file output (including Standard MIDI Files).

## Out of Scope

- Complex MML
- Real-time MIDI message output
- Effects (e.g., LPF, overdrive/distortion, delay)
- GUI editor → For TUI editors, please refer to `cat-edit-mml` and `ym2151-tone-editor`.

## Related Projects

### cat-play-chord (Under Consideration)

[cat-play-chord](https://github.com/cat2151/cat-play-chord) is a project (under consideration) that generates and plays MML from chord notation using chord2mml.

### Future Outlook for MML to SMF
- Status
  - Only 'cde' has been implemented.
    - Goal: By focusing on minimal implementation initially, we aim to smooth out problem-solving until real-time Do-Re-Mi playback is achieved.
  - Using SMF
    - Goal: Using SMF makes verification and development easier, reducing the risk of stalled development.
- The MML dialect is assumed to be mmlabc, for which there is expertise and a clear format.
- Anticipated to proceed with a TDD agent; if hallucinations occur, they will be investigated.

### SMF to Nuked-OPM Friendly JSON (Under Consideration)
- Example
  - Delay Vibrato
    - Based on the `tone modify delay vibrato` value in `tone settings toml`,
      - From an OPM sound driver perspective,
        - Generate soft LFO register events for each tick.
    - Separating SMF and toml is intended to simplify the MIDI implementation on the SMF side.
      - The toml side makes it easier to introduce breaking changes for timbres and OPM sound driver-like processing, following the ETC principle.
- The following passes are envisioned:
  - SMF to intermediate representation * (JSON representing SMF as text is envisioned)
  - Intermediate representation to intermediate representation * (n times) * (Delay vibrato is envisioned here)
  - Intermediate representation to Nuked-OPM friendly JSON
- Anticipated to proceed with a Linux Python TDD agent; if hallucinations occur, they will be investigated.

### Nuked-OPM friendly JSON player
- Already implemented as a library: `ym2151-log-play-server`
  - This application also uses this library.
- Its purpose is to facilitate development.
  - Easier to debug, reducing the risk of stalled development.

### Real-time FM Tone Editor
- Implemented: `ym2151-tone-editor`

#### Draft Proposal
- The following is a draft proposal and differs from the already implemented specifications; it is placed here for now.
- Purpose of writing here
  - Rubber ducking
- Purpose
  - For verification
- Priority
  - Prioritize ease of development and the ability to sketch timbres with minimal operations.
- Operations
  - Right hand: Increase/decrease values assigned to mouse x,y respectively.
  - Left hand: WASD+QE for cursor movement, SPACE for confirmation, ESC for cancel, Z for UNDO (this is still vague).
  - x,y switches DecayRate, ModulatorTL, FB, MUL by cursor movement.
  - WASD+QE, UNDO are vague.
  - Instead of cursor movement and confirmation, pressing a specific key to instantly apply effects for fast editing might be quicker, so that will also be tried, possibly with a configuration toml file.
- Sound
  - Switch between pluck and long tone timbres with cursor movement.
  - OP connection algorithm is 2op parallel, detune has 4 distinct values.
- Display
  - Windows TUI 80x24
  - Timbre parameter display is in mdx note.x format.
  - While running, save the timbre in mdx note.x format to the clipboard 1 second after the last numerical change; output is limited to this only, for now.
- All specifications are provisional, subject to frequent breaking changes, prioritizing ease of development.
- Even with this, the specifications are still too numerous (too much to start small), so we will begin with a smaller, provisional implementation with more focused specifications.

#### Draft Proposal
- The following is a draft proposal and differs from the already implemented specifications; it is placed here for now.
- Output: Plays a note each time a key is pressed.
  - Internally, it plays MML.
  - First, verify with an extremely simple implementation.
- Purpose: For verifying the provision of a timbre creation experience, which is one of the joys of synthesizers.
  - While various specifications are necessary to provide a better successful experience, these will be excluded from the initial implementation (otherwise, it will be endless and get lost).
    - Start small. Take one step at a time. That's global optimization.
- It will be separated into an independent TUI application repository. Start small.

### cat-edit-mml
- Implemented
- mmlabc editor, playback engine is ym2151-log-play-server.
- Output: Plays a note each time a key is pressed.
- Purpose: Provide the experience of playing Do when 'c' is pressed.
- It will be separated into an independent TUI application repository. Start small.

## Additional Information

### How to Build, Install, and Run for Developers

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