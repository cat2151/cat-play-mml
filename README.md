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

`cat-play-mml` is a CLI tool that plays music using Music Macro Language (MML). Inputting the string `cde` will play the "do-re-mi" musical notes. It is for Windows and written in Rust.

## Status
- Breaking changes occur frequently. Many MML commands are not yet implemented. README maintenance is in progress and will be advanced going forward.

## Features

- Low Latency (100ms)
  - Sound plays within 100ms to 500ms after executing the CLI.
    - Just specify `"cde"` as a command-line argument!

## Quick Start Guide

### Environment Setup
- Please install `Rust` on Windows.

### Installation
```
cargo install --force --git https://github.com/cat2151/cat-play-mml.git
```

That's it! It will be installed on your Windows machine directly from GitHub.

### Playing Music
```
cat-play-mml cde
```

Do-Re-Mi will play.

## Future Plans / Candidates for related project issues

- [mmlabc-to-smf-rust](https://github.com/cat2151/mmlabc-to-smf-rust/issues):
  - Implement MML `;`. `c;e;g` will form a C major chord. To allow assigning different timbres in the future, it should be ch1 C, ch2 E, ch3 G (1-based notation) instead of a C major chord on ch1.
  - Output MIDI Program Change 0 (0-based notation) at the beginning of each SMF channel. Do not output to channels without note output.

- [smf-to-ym2151log-rust](https://github.com/cat2151/smf-to-ym2151log-rust/issues)
  - Along with the MML `;` implementation, support multiple channels. For chords within a channel, this will be considered separately for now and can remain undefined behavior.
  - MIDI Program Change 0 (0-based notation) will represent an acoustic grand piano-like timbre. More specifically, it just needs to not be a sine wave.

- [ym2151-log-play-server](https://github.com/cat2151/ym2151-log-play-server/issues)

### Key Features

- **Simple, instant playback**: Just pass "cde" as an argument to play Do-Re-Mi.
- **Low latency**: Real-time music playback.
- **Background playback**: Play music in server mode while performing other operations.

### Usage

#### Basic Usage (Automatic Server Startup)

```
cat-play-mml cde
```

Upon first execution, the server automatically starts in the background, and music playback begins. The command immediately exits, allowing you to input the next command.

For subsequent executions, it sends the music to the already running server:

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

MML is a language for describing music using text. It uses the following notation:

- `c`, `d`, `e`, `f`, `g`, `a`, `b`: Notes (C, D, E, F, G, A, B)
- `o4`: Octave setting (4th octave)
- `l4`: Note length setting (quarter note)
- `t120`: Tempo setting (BPM 120)
- `<`, `>`: Octave up/down
- `+`, `-`: Semitone up/down
- `r`: Rest

## Technical Details

### Architecture

1.  **Parser**: Uses tree-sitter to convert MML text into an AST.
2.  **Intermediate Representation**: Converts AST into musical data structures.
3.  **Audio Generation**: Generates audio waveforms from the intermediate representation.
4.  **Playback**: Outputs audio using an audio library.

### Development Environment

- Windows
- Rust
- Linux runner for agent's TDD (as long as the agent can do TDD) + ALSA SDK and setup (to enable TDD even in headless environments).

### Libraries Used

- Nuked-OPM
- **Rust**: cpal

## Project Goals

### Top Priority Goals
- [x] Provide the experience of executing `cat-play-mml "cde"` as a Windows Rust executable, resulting in real-time playback of Do-Re-Mi.

### Next Goals
- mmlabc grammar
  - Priority
    - `;`
- File output for intermediate representation (including Standard MIDI Files).

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
  - Only `cde` has been implemented.
    - Goal: Initially, focus on a minimal implementation to smoothly resolve issues leading up to real-time Do-Re-Mi playback.
  - Uses SMF
    - Goal: Using SMF makes verification and development easier, reducing the risk of development stagnation.
- The MML dialect is assumed to be mmlabc, as there is existing know-how and a clear format.
- Assumed to proceed with a TDD agent; will consider if hallucinations occur.

### SMF to Nuked-OPM friendly JSON (Under Consideration)
- Example
  - Delay vibrato
    - Based on the `tone modify delay vibrato` value in `tone settings toml`,
      - From an OPM sound driver perspective,
        - Generate soft LFO register events for each tick.
    - Separating SMF and toml is to simplify the MIDI implementation on the SMF side.
      - The toml side makes it easier to implement breaking changes for timbres and OPM sound driver-like processing, adhering to the ETC principle.
- The following passes are envisioned:
  - SMF to Intermediate Representation *Assumes SMF represented as text in JSON*
  - Intermediate Representation to Intermediate Representation *n times* *Delay vibrato is envisioned here*
  - Intermediate Representation to Nuked-OPM friendly JSON
- Assumed to proceed with a Linux Python TDD agent; will consider if hallucinations occur.

### Nuked-OPM friendly JSON player
- Already implemented as a library: `ym2151-log-play-server`
  - This application also uses this library.
- Purpose: To facilitate development.
  - Easier to debug, reduces the risk of development stagnation.

### Real-time FM Tone Editor
- Implemented: `ym2151-tone-editor`

#### Proposal, Draft
- The following is a draft and differs from the already implemented specifications; it is placed here for now.
- Purpose of writing here
  - Rubber ducking
- Use case
  - For verification
- Priority
  - Prioritize ease of development and the use case of sketching timbres with minimal operations.
- Operations
  - Right hand: Mouse X, Y to increase/decrease values assigned to each.
  - Left hand: WASD+QE for cursor movement, SPACE to confirm, ESC to cancel, Z for UNDO (this is still fuzzy).
  - X, Y switch between DecayRate, ModulatorTL, FB, MUL with cursor movement.
  - WASD+QE, UNDO are fuzzy.
  - Instead of cursor movement and confirmation, pressing a specific key to instantly apply an effect for faster editing might be more efficient, so that will also be tried, possibly with a settings toml file.
- Sound
  - Timbre switches between pluck and long tone with cursor movement.
  - OP connection algorithm is 2-op parallel, detune has 4 different values.
- Display
  - Windows TUI 80x24
  - Timbre parameter display in mdx note.x format.
  - While running, save timbre in mdx note.x format to clipboard 1 second after the last numerical change; output is temporarily limited to this.
- All specifications are provisional, subject to frequent breaking changes; ease of development is prioritized.
- Even this is still too many specifications (too many to start small), so we will start small with a provisional implementation with more limited specifications.

#### Proposal, Draft
- The following is a draft and differs from the already implemented specifications; it is placed here for now.
- Output: Plays music with each key press.
  - Internally, it `play mml`.
  - First, verify with an extremely simple implementation.
- Purpose: For verification of providing one of the core synthesizer experiences: timbre creation.
  - Many specifications are needed to provide a better successful experience, but these will be excluded from the initial implementation (otherwise, it will become endless and get lost).
    - Start small. Take one step at a time. That's global optimization.
- Split into a separate repository as an independent TUI application. Start small.

### cat-edit-mml
- Implemented
- An mmlabc editor, with `ym2151-log-play-server` as the playback engine.
- Output: Plays music with each key press.
- Purpose: To provide the experience of pressing 'c' and hearing 'do'.
- Split into a separate repository as an independent TUI application. Start small.

## Additional Notes

### For Developers: How to Build, Install, and Run

```powershell
# Build & Run *In the cloned directory*
cargo run --release cegb

# Install *In the cloned directory*
cargo install --path .

# Run *Once installed, you can run it from any directory like this*
cat-play-mml cegb
```

## License

This project is licensed under the [MIT License](LICENSE).

*The English README.md is automatically generated by GitHub Actions using Gemini's translation based on README.ja.md.