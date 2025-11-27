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

`cat-play-mml` is a CLI tool that plays music using Music Macro Language (MML). If you input the string `cde`, it will play the music "Do-Re-Mi". It's for Windows and is written in Rust.

## Status
- Breaking changes occur frequently. Many MML commands are unimplemented. The README is partially maintained and will be updated in the future.

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

That's it! It will be installed on your Windows machine directly from GitHub.

### Play
```
cat-play-mml cde
```

"Do-Re-Mi" will play.

## Future Plans for Related Project Issues

- [mmlabc-to-smf-rust](https://github.com/cat2151/mmlabc-to-smf-rust/issues):
  - Implement MML `;`. `c;e;g` will form a C major chord. In the future, different timbres can be assigned to each, so it would be ch1 C, ch2 E, ch3 G (1-based description) rather than ch1 C E G.
  - Output MIDI Program Change 0 (0-based description) at the beginning of each SMF channel. Do not output for channels without note output.

- [smf-to-ym2151log-rust](https://github.com/cat2151/smf-to-ym2151log-rust/issues)
  - Support multiple channels due to MML `;` implementation. For chords within a channel, this will be considered separately; for now, it remains an undefined behavior.
  - MIDI Program Change 0 (0-based description) should result in an acoustic grand piano-like timbre. More specifically, it should be something other than a sine wave.

- [ym2151-log-play-server](https://github.com/cat2151/ym2151-log-play-server/issues)

### Key Features

- **Simple, instant playback**: Pass "cde" as an argument to play "Do-Re-Mi".
- **Low latency**: Real-time music playback.
- **Background playback**: Play music in server mode while performing other operations.

### Usage

#### Basic Usage (Automatic Server Startup)

```
cat-play-mml cde
```

On the first run, the server automatically starts, and playback begins in the background. The command finishes immediately, allowing you to enter the next command.

For subsequent runs, the playback is sent to the already running server:

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

#### Manual Server Startup (Advanced Users)

```
cat-play-mml --server
```

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

1.  **Parser**: Uses tree-sitter to convert MML text into an AST.
2.  **Intermediate Representation**: Converts the AST into music data structures.
3.  **Audio Generation**: Generates audio waveforms from the intermediate representation.
4.  **Playback**: Outputs audio using an audio library.

### Development Environment

- Windows
- Rust
- Linux runner for agent TDD (as long as the agent can do TDD) + ALSA SDK and configuration (to enable TDD in a headless environment).

### Libraries Used

- Nuked-OPM
- **Rust**: cpal

## Project Goals

### Small Goal
- [x] As a Windows Rust .exe, when `cde` is specified as a command-line argument, "Do-Re-Mi" plays in real-time.

### Next Goals
- mmlabc grammar
  - Priority
    - `;`
- File output of intermediate representation (including Standard MIDI File)

## Out of Scope

- Complex MML
- Real-time MIDI message output
- Effects (LPF, overdrive/distortion, delay, etc.)
- GUI editor → For TUI editor, please refer to `cat-edit-mml` and `ym2151-tone-editor`.

## Related Projects

### cat-play-chord Under Consideration

[cat-play-chord](https://github.com/cat2151/cat-play-chord) is a project (under consideration) that uses `chord2mml` to generate MML from chord notation and play it.

### Future Outlook for MML to SMF
- Status
  - Only `cde` is implemented.
    - Goal: Initially focus on minimal implementation to smoothly resolve issues until real-time "Do-Re-Mi" playback.
  - Using SMF.
    - Goal: Using SMF makes verification and development easier, reducing the risk of development stalling.
- Assumed MML dialect is mmlabc, due to existing know-how and clear format.
- Expected to proceed with TDD agent; will consider if hallucinations occur.

### SMF to Nuked-OPM friendly JSON Under Consideration
- Example
  - Delay vibrato
    - Based on the `tone modify delay vibrato` values in `tone settings toml`,
      - For an OPM sound driver,
        - Generate soft LFO register events every tick.
    - Separating SMF and TOML is to simplify the MIDI implementation on the SMF side.
      - This makes it easier to introduce breaking changes for timbre and OPM sound driver processing on the TOML side (ETC principle).
- The following passes are envisioned:
  - SMF to Intermediate Representation (JSON representing SMF as text is assumed).
  - Intermediate Representation to Intermediate Representation (N times) (Delay vibrato is envisioned here).
  - Intermediate Representation to Nuked-OPM friendly JSON.
- Expected to proceed with Linux Python TDD agent; will consider if hallucinations occur.

### Nuked-OPM friendly JSON player
- Already implemented as a library: `ym2151-log-play-server`
  - This application also uses this library.
- Purpose: To facilitate development.
  - Easier debugging, reduces the risk of development stalling.

### Real-time FM tone editor
- Implemented: `ym2151-tone-editor`

#### Draft, Proposal
- The following is a draft and differs from the implemented specifications; it is placed here for now.
- Purpose of writing here
  - Rubber ducking.
- Use case
  - For verification.
- Priority
  - Prioritize ease of development and quick sketching of timbres with minimal operations.
- Operations
  - Right hand: Mouse x, y for increasing/decreasing assigned numerical values.
  - Left hand: WASD+QE for cursor movement, SPACE for confirmation, ESC for cancel, Z for UNDO (fuzzy).
  - x, y: Switch between DecayRate, ModulatorTL, FB, MUL by cursor movement.
  - WASD+QE, UNDO are fuzzy.
  - Instead of cursor movement and confirmation, pressing a key for an instant effect and faster editing might be better, so try that too (e.g., via a settings TOML file).
- Sound
  - Switch between pluck and long tone timbres by cursor movement.
  - OPM connection algorithm is 2-op parallel, detune has 4 different values.
- Display
  - Windows TUI 80x24.
  - Timbre parameter display in mdx note.x format.
  - While running, save timbre to clipboard in mdx note.x format 1 second after the last numerical change; restrict output to this only, for now.
- All specifications are provisional, frequent breaking changes will occur, prioritizing ease of development.
- Even this has too many specifications (too many to start small), so start small with a provisional implementation with even more restricted specifications.

#### Draft, Proposal
- The following is a draft and differs from the implemented specifications; it is placed here for now.
- Output: Play music each time a key is pressed.
  - Internally, it plays MML.
  - First, verify with an extremely simple implementation.
- Purpose: For verifying the experience of timbre creation, which is one of the main attractions of synthesizers.
  - To provide a better successful experience, various specifications are needed, but these are excluded from the initial implementation (otherwise, it becomes endless and you get lost).
    - Start small. Take one step at a time. That's the global optimum.
- Make it a separate repository as an independent TUI app. Start small.

### cat-edit-mml
- Implemented.
- An mmlabc editor, with `ym2151-log-play-server` as the playback engine.
- Output: Play music each time a key is pressed.
- Purpose: To provide the experience of "Do" playing when you press 'c'.
- Make it a separate repository as an independent TUI app. Start small.

## Additional Information

### How to Build, Install, and Run for Developers

```powershell
# Build & Run (in the cloned directory)
cargo run --release cegb

# Install (in the cloned directory)
cargo install --path .

# Run (Once installed, you can run it from any directory like this)
cat-play-mml cegb
```

## License

This project is licensed under the [MIT License](LICENSE).

*The English README.md is automatically generated by GitHub Actions using Gemini's translation of README.ja.md.*