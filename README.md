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

`cat-play-mml` is a CLI tool that plays music using Music Macro Language (MML). If you input the string `cde`, it will play the music "Do-Re-Mi". It is for Windows.

## Quick Start Guide

### Setup
- Install `Rust` and `Zig` on Windows.

### Installation
```
cargo install --git https://github.com/cat2151/cat-play-mml.git --branch main --bin cat-play-mml
```

That's it! It will be installed from GitHub to your Windows system.

### Playback
```
cat-play-mml cde
```

You'll hear "Do-Re-Mi".

## Future Plans (to be added to related project issues)

- [mmlabc-to-smf-rust](https://github.com/cat2151/mmlabc-to-smf-rust/issues):
  - Implement MML `;`. `c;e;g` will form a C major chord. To allow assigning different timbres in the future, it should be ch1 C, ch2 E, ch3 G (1-based notation) rather than ch1 C E G.
  - Output MIDI Program Change 0 (0-based notation) at the beginning of each SMF channel. Do not output to channels without note output.

- [smf-to-ym2151log-rust](https://github.com/cat2151/smf-to-ym2151log-rust/issues)
  - With the implementation of MML `;`, support multiple channels. For chords within a channel, this will be considered separately and can remain undefined behavior for now.
  - MIDI Program Change 0 (0-based notation) will be set to an acoustic grand piano-like timbre. More specifically, it just needs to not be a sine wave.

- [ym2151-log-play-server](https://github.com/cat2151/ym2151-log-play-server/issues)

### Key Features

- **Simple, instant playback**: Just pass "cde" as an argument to play "Do-Re-Mi".
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

#### Manual Server Startup (Advanced)

Start the server by specifying a JSON file:

```
cat-play-mml --server output.json
```

## What is MML (Music Macro Language)?

MML is a language for describing music in text. It uses notation such as:

- `c`, `d`, `e`, `f`, `g`, `a`, `b`: Musical scale (Do, Re, Mi, Fa, Sol, La, Ti)

### Future Implementation Plans
- `o4`: Octave setting (4th octave)
- `l4`: Note length setting (quarter note)
- `t120`: Tempo setting (BPM 120)
- `<`, `>`: Raise/lower octave
- `+`, `-`: Semitone up/down
- `r`: Rest

## Technical Details

### Architecture

1.  **Parser**: Converts MML text to AST using tree-sitter.
2.  **Intermediate Representation**: Converts AST to music data structure.
3.  **Audio Generation**: Generates audio waveforms from the intermediate representation.
4.  **Playback**: Outputs audio using an audio library.

### Development Environment

- Windows
- Rust
- Zig cc (mingw and msys2 are prohibited)
- Linux runner for agent TDD (as long as the agent can perform TDD) + ALSA SDK and configuration (to enable TDD in headless environments)

### Libraries Used

- Nuked-OPM
- **Rust**: cpal

## Project Goals

### Minor Goal
- [x] For the Windows Rust version (exe), playing "Do-Re-Mi" in real-time when "cde" is specified as a command-line argument.

### Next Goals
- `mmlabc` grammar
  - Priority
    - `;`
- Intermediate representation file output (including Standard MIDI Files)

## Out of Scope

- Complex MML
- Real-time MIDI message output
- Effects (LPF, Overdrive/Distortion, Delay, etc.)
- GUI Editor

## Related Projects

### cat-play-chord (Under Consideration)

[cat-play-chord](https://github.com/cat2151/cat-play-chord) is a project (under consideration) that generates and plays MML from chord notation using `chord2mml`.

### Future Outlook for MML to SMF
- Status
  - Only 'cde' has been implemented.
    - Goal: Initially focus on minimal implementation to smooth problem-solving until real-time "Do-Re-Mi" playback.
  - Using SMF
    - Goal: Using SMF makes verification and development easier, reducing the risk of project stagnation.
- MML dialect is assumed to be `mmlabc`, with existing know-how and clear format.
- Assumed to proceed with a TDD agent; if hallucinations occur, they will be investigated.

### SMF to Nuked-OPM friendly JSON (Under Consideration)
- Example
  - Delay Vibrato
    - Based on the `tone modify delay vibrato` value in `tone settings toml`,
      - From an OPM sound driver perspective,
        - Generate soft LFO register events for each tick.
    - Separating SMF and toml is for simplifying the MIDI implementation on the SMF side.
      - The toml side allows for easier destructive changes to timbre and OPM sound driver-like processing, following the ETC principle.
- The following passes are envisioned:
  - SMF to intermediate representation * (assuming JSON representation of SMF as text)
  - Intermediate representation to intermediate representation * (n times) * (Delay vibrato is envisioned here)
  - Intermediate representation to Nuked-OPM friendly JSON
- Assumed to proceed with a Linux Python TDD agent; if hallucinations occur, they will be investigated.

### Nuked-OPM friendly JSON player
- Implemented (log player in a separate repository)
- Purpose: To facilitate development.
  - Easier debugging, reduces the risk of project stagnation.

### Real-time FM Tone Editor (Provisional, Under Consideration)
- Purpose of writing here
  - Rubber ducking
- Purpose
  - For verification
- Priority
  - Prioritize ease of development and the ability to quickly sketch timbres with minimal operations.
- Operation
  - Right hand: Mouse X, Y to increase/decrease assigned values respectively.
  - Left hand: WASD+QE for cursor movement, SPACE to confirm, ESC to cancel, Z for UNDO (this is vague).
  - X, Y switch between DecayRate, ModulatorTL, FB, MUL with cursor movement.
  - WASD+QE, UNDO are vague.
  - Instead of cursor movement and confirmation, pressing a specific key to apply an effect instantly for faster editing is also an option to try (e.g., via a settings TOML file).
- Sound
  - Switch between pluck and long tone timbres with cursor movement.
  - OP connection algorithm is 2-op parallel, detune has 4 different values.
- Display
  - Windows TUI 80x24
  - Timbre parameter display in `mdx note.x` format.
  - While running, save timbre in `mdx note.x` format to clipboard 1 second after the last value change; limit output to this (provisional).
- All specifications are provisional, frequent breaking changes, prioritizing ease of development.
- Even this is still too many specifications (too much for a small start), so start small with a provisional implementation of a more focused set of specifications.

### cat-edit-ym2151-tone
- Output: Play on each key press.
  - Internally, it will play MML.
  - First, verify with an extremely simple implementation.
- Purpose: For verifying the provision of a timbre creation experience, which is one of the joys of synthesizers.
  - While various specifications are needed to provide a better success experience, these will be excluded from the initial implementation (otherwise, it becomes endless and gets lost).
    - Start small. Take one step at a time. That is global optimization.
- Create as a separate repository, a standalone TUI application. Start small.

### cat-edit-mmlabc
- Output: Play on each key press.
- Purpose: Provide the experience of hearing "Do" when 'c' is pressed.
- Create as a separate repository, a standalone TUI application. Start small.

## Appendix

### Build, Install, and Run Methods for Developers

```powershell
# Build & Run * (in the cloned directory)
cargo run --release cegb

# Install * (in the cloned directory)
cargo install --path .

# Run * (Once installed, you can run it from any directory like this)
cat-play-mml cegb
```

## License

This project is released under the [MIT License](LICENSE).

* The English README.md is automatically generated by GitHub Actions using Gemini's translation based on README.ja.md.
