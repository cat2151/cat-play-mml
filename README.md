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

## Overview

`cat-play-mml` is a CLI tool that plays music written in Music Macro Language (MML). Inputting the string `cde` will play the melody `C-D-E`. It is designed for Windows.

## Quick Start Guide

### Environment Setup
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
C-D-E will play.

## Future Development Plans (to be documented in related project issues)

- [mmlabc-to-smf-rust](https://github.com/cat2151/mmlabc-to-smf-rust/issues):
  - Implement MML `;`. `c;e;g` will form a C major chord. To allow assigning different timbres in the future, it should be ch1 C, ch2 E, ch3 G (1-based notation), not ch1 C-E-G.
  - Output MIDI Program Change 0 (0-based notation) at the beginning of each SMF channel. Do not output for channels without note data.

- [smf-to-ym2151log-rust](https://github.com/cat2151/smf-to-ym2151log-rust/issues)
  - With the implementation of MML `;`, support multiple channels. Chords within a channel will be considered separately and can remain undefined behavior for now.
  - MIDI Program Change 0 (0-based notation) should result in an acoustic grand piano-like timbre. More specifically, it just needs to not be a sine wave.

- [ym2151-log-play-server](https://github.com/cat2151/ym2151-log-play-server/issues)

### Key Features

- **Simple, Instant Playback**: Just pass "cde" as an argument to play C-D-E.
- **Low Latency**: Real-time music playback.
- **Background Playback**: Perform other operations while music plays in server mode.

### Usage

#### Basic Usage (Automatic Server Startup)

```
cat-play-mml cde
```
On the first run, the server will automatically start, and playback will begin in the background. The command will exit immediately, allowing you to enter the next command.

For subsequent runs, playback commands are sent to the already running server:

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

Start the server by specifying a JSON file:

```
cat-play-mml --server output.json
```

## What is MML (Music Macro Language)?

MML is a language for describing music using text. It uses notations such as:

- `c`, `d`, `e`, `f`, `g`, `a`, `b`: Notes (C, D, E, F, G, A, B)

### Planned Features
- `o4`: Octave setting (4th octave)
- `l4`: Note length setting (quarter note)
- `t120`: Tempo setting (BPM 120)
- `<`, `>`: Octave up/down
- `+`, `-`: Semitone up/down (sharps/flats)
- `r`: Rest

## Technical Details

### Architecture

1. **Parser**: Converts MML text into an AST using tree-sitter.
2. **Intermediate Representation**: Transforms the AST into a music data structure.
3. **Audio Generation**: Generates audio waveforms from the intermediate representation.
4. **Playback**: Outputs audio using an audio library.

### Development Environment

- Windows
- Rust
- Zig cc (mingw and msys2 are prohibited)
- Linux runner for agent's TDD (as long as the agent can perform TDD) + ALSA SDK and configuration (to enable TDD even in a headless environment)

### Libraries Used

- Nuked-OPM
- **Rust**: cpal

## Project Goals

### Short-term Goal
- [x] As a Windows Rust executable, when `cde` is specified as a command-line argument, C-D-E should play in real-time.

### Next Goals
- mmlabc grammar
  - Priority
    - `;`
- Output of intermediate representation files (including Standard MIDI Files)

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
  - Only `cde` implemented.
    - Aim: Initially, focus on minimal implementation to smoothly resolve issues until real-time C-D-E playback is achieved.
  - Using SMF.
    - Aim: Using SMF makes verification and development easier, reducing the risk of project stagnation.
- MML dialect is assumed to be `mmlabc`, as there is existing knowledge and a clear format.
- Development is intended to proceed with a TDD agent; hallucinations will be addressed if they occur.

### SMF to Nuked-OPM Friendly JSON (Under Consideration)
- Example
  - Delay Vibrato
    - Based on the `tone modify delay vibrato` value in `tone settings toml`.
      - From an OPM sound driver perspective:
        - Generate a soft LFO register event for each tick.
    - The separation of SMF and TOML is to simplify the MIDI implementation on the SMF side.
      - This allows for easier destructive changes to timbres and OPM sound driver-like processing on the TOML side, adhering to the ETC principle.
- The following passes are envisioned:
  - SMF to Intermediate Representation (JSON representing SMF as text is assumed).
  - Intermediate Representation to Intermediate Representation (n times, delay vibrato is envisioned here).
  - Intermediate Representation to Nuked-OPM friendly JSON.
- Development is intended to proceed with a Linux Python TDD agent; hallucinations will be addressed if they occur.

### Nuked-OPM Friendly JSON Player
- Already implemented (log player in a separate repository).
- Purpose: To facilitate development.
  - Easier debugging, reduces the risk of project stagnation.

### Real-time FM Tone Editor (Provisional, Under Consideration)
- Purpose of writing here:
  - Rubber ducking.
- Use case:
  - For verification.
- Priority:
  - Prioritize ease of development and the ability to quickly sketch timbres with minimal operations.
- Operation:
  - Right hand: Mouse x,y for increasing/decreasing assigned numerical values.
  - Left hand: WASD+QE for cursor movement, SPACE for confirmation, ESC for cancellation, Z for UNDO (this is still vague).
  - x,y switches between DecayRate, ModulatorTL, FB, MUL via cursor movement.
  - WASD+QE, UNDO are still vague.
  - Instead of cursor movement and confirmation, a single key press that instantly applies an effect for faster editing might be better for editing speed, so that will also be explored (e.g., via a settings TOML file).
- Sound:
  - Switch between pluck and long tone timbres with cursor movement.
  - OP connection algorithm is 2-op parallel, detune has 4 distinct values.
- Display:
  - Windows TUI 80x24.
  - Timbre parameter display in MDX note.x format.
  - While running, save timbre in MDX note.x format to clipboard 1 second after the last numerical change; output is provisionally limited to this.
- All specifications are provisional, destructive changes will be frequent, prioritizing ease of development.
- Even this is still too many specifications (too many to start small), so we will start small with a more focused provisional implementation.

### cat-edit-ym2151-tone
- Output: Plays a sound with each key press.
  - Internally, it plays MML.
  - First, verify with an extremely simple implementation.
- Purpose: For verifying the provision of a timbre creation experience, which is one of the joys of synthesizers.
  - While many specifications are needed to provide a better successful experience, these will be excluded from the initial implementation (otherwise, it becomes endless and unfocused).
    - Start small. Take one step at a time. That is global optimization.
- This will be a separate repository as an independent TUI application. Start small.

### cat-edit-mmlabc
- Output: Plays a sound with each key press.
- Purpose: To provide the experience of C playing when 'c' is pressed.
- This will be a separate repository as an independent TUI application. Start small.

## Additional Notes

### Build, Install, and Run for Developers

```powershell
# Build & Run (in the cloned directory)
cargo run --release cegb

# Install (in the cloned directory)
cargo install --path .

# Run (once installed, you can run from any directory like this)
cat-play-mml cegb
```

## License

This project is released under the [MIT License](LICENSE).

*The English README.md is automatically generated from README.ja.md via Gemini translation and GitHub Actions.*