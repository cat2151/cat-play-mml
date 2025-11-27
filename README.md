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

`cat-play-mml` is a CLI tool for playing music using Music Macro Language (MML). If you input the string `cde`, it will play the music "Do-Re-Mi". It is for Windows and written in Rust.

## Features

- 100ms
  - Sound plays 100ms to 500ms after executing the CLI
    - Just specify `"cde"` as a command-line argument!

## Quick Start Guide

### Prerequisites
- Install `Rust` on Windows.

### Installation
```
cargo install --git https://github.com/cat2151/cat-play-mml.git --branch main --bin cat-play-mml
```

That's it! It will be installed on your Windows from GitHub.

### Playing Music
```
cat-play-mml cde
```

"Do-Re-Mi" will play.

## Future Plans: Candidates for issues in related projects

- [mmlabc-to-smf-rust](https://github.com/cat2151/mmlabc-to-smf-rust/issues):
  - Implement MML `;`. `c;e;g` will be a C major chord. Each note will be assigned to a different channel in the future, so it will be ch1 C, ch2 E, ch3 G (1-based notation) instead of ch1 C-E-G.
  - Output MIDI Program Change 0 (0-based notation) at the beginning of each SMF channel. Do not output for channels without note output.

- [smf-to-ym2151log-rust](https://github.com/cat2151/smf-to-ym2151log-rust/issues)
  - Support multiple channels in conjunction with MML `;` implementation. Chord processing within a channel is to be considered separately and remains undefined behavior for now.
  - MIDI Program Change 0 (0-based notation) will represent an acoustic grand piano-like timbre. More specifically, it should be something other than a pure sine wave.

- [ym2151-log-play-server](https://github.com/cat2151/ym2151-log-play-server/issues)

### Key Features

- **Simple, instant playback**: Pass "cde" as an argument to play "Do-Re-Mi".
- **Low latency**: Real-time music playback.
- **Background playback**: Play music in server mode while performing other operations.

### How to Use

#### Basic Usage (Automatic Server Start)

```
cat-play-mml cde
```

On the first run, the server will automatically start, and playback will begin in the background. The command will finish quickly, allowing you to enter the next command.

On subsequent runs, music will be sent to the already running server:

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

MML is a language for describing music in text. It uses the following notation:

- `c`, `d`, `e`, `f`, `g`, `a`, `b`: Musical notes (C, D, E, F, G, A, B)
- `o4`: Octave setting (4th octave)
- `l4`: Note length setting (quarter note)
- `t120`: Tempo setting (BPM 120)
- `<`, `>`: Raise/lower octave
- `+`, `-`: Sharpen/flatten by a semitone
- `r`: Rest

## Technical Details

### Architecture

1.  **Parser**: Uses tree-sitter to convert MML text into an AST.
2.  **Intermediate Representation**: Converts the AST into a music data structure.
3.  **Audio Generation**: Generates audio waveforms from the intermediate representation.
4.  **Playback**: Outputs audio using an audio library.

### Development Environment

- Windows
- Rust
- Linux runner for agent TDD (as long as the agent can perform TDD) + ALSA SDK and configuration (to enable TDD in a headless environment).

### Libraries Used

- Nuked-OPM
- **Rust**: cpal

## Project Goals

### Small Goals
- [x] As a Windows Rust executable, when `cde` is specified as a command-line argument, "Do-Re-Mi" should play in real-time.

### Next Goals
- mmlabc grammar
  - Priority
    - `;`
- Intermediate representation file output (including Standard MIDI File)

## Out of Scope

- Complex MML
- Real-time MIDI message output
- Effects (LPF, overdrive/distortion, delay, etc.)
- GUI editor -> For TUI editors, please refer to `cat-edit-mml` and `ym2151-tone-editor`.

## Related Projects

### cat-play-chord (Under consideration)

[cat-play-chord](https://github.com/cat2151/cat-play-chord) is a project (under consideration) that generates and plays MML from chord notation using `chord2mml`.

### Future Outlook for MML to SMF
- Status
  - Only `cde` is implemented.
    - Goal: Initially focus on minimal implementation to streamline problem-solving until real-time "Do-Re-Mi" playback.
  - SMF is used.
    - Goal: Using SMF makes verification and development easier, reducing the risk of development stalling.
- MML dialect is assumed to be `mmlabc`, with existing expertise and a clear format.
- Intended to proceed with TDD agent; will address hallucinations if they arise.

### smf to Nuked-OPM friendly JSON (Under consideration)
- Example
  - Delay vibrato
    - Based on the `tone modify delay vibrato` values in the tone settings `toml`:
      - From an OPM sound driver perspective:
        - Generate soft LFO register events every tick.
    - Separating SMF and `toml` simplifies the MIDI implementation on the SMF side.
      - Makes destructive changes to timbre and OPM sound driver-like processing easier on the `toml` side (ETC principle).
- The following passes are envisioned:
  - SMF to intermediate representation (JSON representing SMF as text is assumed).
  - Intermediate representation to intermediate representation (N times, delay vibrato is envisioned here).
  - Intermediate representation to Nuked-OPM friendly JSON.
- Intended to proceed with Linux Python TDD agent; will address hallucinations if they arise.

### Nuked-OPM friendly JSON player
- Implemented as a library: `ym2151-log-play-server`
  - This application also uses this library.
- Purpose: To facilitate development.
  - Easier debugging, reducing the risk of development stalling.

### Real-time FM tone editor
- Implemented: `ym2151-tone-editor`

#### Draft, Proposal
- The following is a draft and differs from the implemented specifications. It is kept here for now.
- Purpose of writing here
  - Rubber ducking
- Use case
  - For verification
- Priority
  - Prioritize ease of development and creating rough tone sketches with minimal operations.
- Operations
  - Right hand: Mouse x,y for increasing/decreasing assigned numerical values.
  - Left hand: WASD+QE for cursor movement, SPACE for confirmation, ESC for cancellation, Z for UNDO (still vague).
  - x,y switches between DecayRate, ModulatorTL, FB, MUL with cursor movement.
  - WASD+QE, UNDO are vague.
  - Instead of cursor movement and confirmation, try a method where pressing a key immediately applies an effect for faster editing (e.g., using a settings `toml` file).
- Sound
  - Switch between pluck and long tone timbres with cursor movement.
  - OPM connection algorithm is 2-op parallel, detune has 4 different values.
- Display
  - Windows TUI 80x24.
  - Tone parameter display in MDX note.x format.
  - While running, save tone in MDX note.x format to clipboard 1 second after the last numeric change. Temporarily limit output to this.
- All specifications are provisional and subject to frequent destructive changes, prioritizing ease of development.
- Even this is still too many specifications (too many to start small), so begin with a provisional implementation with more constrained specifications.

#### Draft, Proposal
- The following is a draft and differs from the implemented specifications. It is kept here for now.
- Output: Play music with each key press.
  - Internally, it plays MML.
  - First, verify with an extremely simple implementation.
- Purpose: For verifying the experience of tone creation, one of the main attractions of synthesizers.
  - Providing a better successful experience requires various specifications, but these will be excluded from the initial implementation (otherwise, it becomes endless and gets lost).
    - Start small. Take one step at a time. That is overall optimization.
- Make it a separate repository as an independent TUI application. Start small.

### cat-edit-mml
- Implemented
- `mmlabc` editor, playback engine is `ym2151-log-play-server`.
- Output: Play music with each key press.
- Purpose: Provide the experience of playing "Do" when 'C' is pressed.
- Make it a separate repository as an independent TUI application. Start small.

## Additional Notes

### Build, Install, and Run for Developers

```powershell
# Build & Run (in the cloned directory)
cargo run --release cegb

# Install (in the cloned directory)
cargo install --path .

# Run (once installed, you can run it from any directory like this)
cat-play-mml cegb
```

## License

This project is licensed under the [MIT License](LICENSE).

※The English `README.md` is automatically generated by GitHub Actions using Gemini's translation based on `README.ja.md`.