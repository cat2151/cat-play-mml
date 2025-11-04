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

`cat-play-mml` is a CLI tool that plays music using Music Macro Language (MML). If you input the string `cde`, it will play the music `do-re-mi`. It is for Windows.

## Quick Start Guide

### Environment Setup
- Install `Rust` and `Zig` on Windows.

### Installation
```
cargo install --git https://github.com/cat2151/cat-play-mml.git --branch main --bin cat-play-mml
```

That's it! It will be installed on your Windows from GitHub.

### Play Music
```
cat-play-mml cde
```

It will play `do-re-mi`.

## Future Plans and Potential Issues for Related Projects

- [mmlabc-to-smf-rust](https://github.com/cat2151/mmlabc-to-smf-rust/issues)：
  - Implement MML `;`. `c;e;g` will represent a C major chord (Do-Mi-Sol). To allow assigning different timbres in the future, it should be ch1 C, ch2 E, ch3 G (1-based notation) instead of ch1 C major chord.
  - Output MIDI Program Change 0 (0-based notation) at the beginning of each SMF channel. Do not output for channels without note output.

- [smf-to-ym2151log-rust](https://github.com/cat2151/smf-to-ym2151log-rust/issues)
  - Support multiple channels in conjunction with the MML `;` implementation. Chords within a channel will be considered separately; for now, undefined behavior is acceptable.
  - Set MIDI Program Change 0 (0-based notation) to an acoustic grand piano-like timbre. More specifically, it should be anything but a sine wave.

- [ym2151-log-player-rust](https://github.com/cat2151/ym2151-log-player-rust/issues)

### Key Features

- **Simple, Instant Playback**: Just pass "cde" as an argument to play `do-re-mi`.
- **Low Latency**: Real-time music playback.

## What is MML (Music Macro Language)?

MML is a language for describing music using text. It uses notations such as:

- `c`, `d`, `e`, `f`, `g`, `a`, `b`: Musical scale notes (Do, Re, Mi, Fa, Sol, La, Si)

### Future Implementations
- `o4`: Octave setting (4th octave)
- `l4`: Note length setting (quarter note)
- `t120`: Tempo setting (BPM 120)
- `<`, `>`: Octave up/down
- `+`, `-`: Semitone up/down
- `r`: Rest

## Technical Details

### Architecture

1.  **Parser**: Converts MML text into an AST using tree-sitter.
2.  **Intermediate Representation**: Converts the AST into a music data structure.
3.  **Audio Generation**: Generates audio waveforms from the intermediate representation.
4.  **Playback**: Outputs audio using an audio library.

### Development Environment

- Windows
- Rust
- Zig cc (mingw and msys2 are prohibited)
- Linux runner for agent's TDD (as long as the agent can perform TDD) + ALSA SDK and configuration (to enable TDD even in a headless environment).

### Libraries Used

- Nuked-OPM
- **Rust**: cpal

## Project Goals

### Short-term Goal
- [x] As a Windows Rust `.exe` version, when `cde` is specified as a command-line argument, `do-re-mi` should play in real-time.

### Next Goals
- mmlabc grammar
  - Priority
    - `;`
- File output of intermediate representation (including Standard MIDI Files)

## Out of Scope

- Complex MML
- Real-time MIDI message output
- Effects (e.g., LPF, overdrive/distortion, delay)
- GUI editor

## Related Projects

### cat-play-chord (Under consideration)

[cat-play-chord](https://github.com/cat2151/cat-play-chord) is a project (under consideration) that generates and plays MML from chord notation using chord2mml.

### Future Outlook for MML to SMF
- **Status**
  - Only `cde` has been implemented.
    - **Aim**: To smoothly resolve issues until `do-re-mi` plays in real-time by focusing on a minimal implementation initially.
  - SMF is being used.
    - **Aim**: Using SMF makes verification and development easier, reducing the risk of development stalling.
- The MML dialect is assumed to be mmlabc, which has existing know-how and a clear format.
- It's planned to proceed with a TDD agent; hallucinations will be addressed if they occur.

### SMF to Nuked-OPM friendly JSON (Under consideration)
- **Example**
  - Delay vibrato
    - Based on the `tone modify delay vibrato` value in `tone settings toml`,
      - In terms of OPM sound driver,
        - Generate soft LFO register events for each tick.
    - Separating SMF and TOML is for simplifying the MIDI implementation on the SMF side.
      - The TOML side makes it easier to perform destructive changes to timbre and OPM sound driver-like processing, following the ETC principle.
- **Anticipated Passes**
  - SMF to intermediate representation (assuming JSON representing SMF as text)
  - Intermediate representation to intermediate representation (n times; delay vibrato is anticipated here)
  - Intermediate representation to Nuked-OPM friendly JSON
- It's planned to proceed with a Linux Python TDD agent; hallucinations will be addressed if they occur.

### Nuked-OPM friendly JSON player
- Implemented (log player in a separate repository)
- Its purpose is to facilitate development.
  - Easier debugging, reduces the risk of development stalling.

### Real-time FM Tone Editor (Provisional, Under consideration)
- **Purpose of writing here**
  - Rubber ducking
- **Usage**
  - For verification
- **Priority**
  - Prioritize ease of development and the ability to quickly sketch timbres with minimal operations.
- **Operations**
  - Right hand: Mouse X, Y to increase/decrease assigned numerical values, respectively.
  - Left hand: WASD+QE for cursor movement, SPACE to confirm, ESC to cancel, Z for UNDO (this is still vague).
  - X, Y switch between DecayRate, ModulatorTL, FB, MUL with cursor movement.
  - WASD+QE, UNDO are still vague.
  - Instead of cursor movement and confirmation, pressing a specific key to instantly apply an effect for faster editing might be more efficient; this approach will also be tested, possibly with a configuration TOML file.
- **Sound**
  - Timbre switches between pluck and long tone via cursor movement.
  - OP connection algorithm is 2-op parallel, detune has 4 different values.
- **Display**
  - Windows TUI 80x24
  - Timbre parameter display in MDX note.x format.
  - While running, save timbre in MDX note.x format to clipboard 1 second after the last numerical change; output is limited to this (provisional).
- All specifications are provisional; destructive changes will be frequent, prioritizing ease of development.
- Even this is still too many specifications (too many to start small), so we will begin with a smaller, provisional implementation with more focused specifications.

## Appendix

### For Developers: Build, Install, and Run Instructions

```powershell
# Build & Run (in cloned directory)
cargo run --release cegb

# Install (in cloned directory)
cargo install --path .

# Run (Once installed, you can run it from any directory like this)
cat-play-mml cegb
```

## License

This project is released under the [MIT License](LICENSE).

*(The English `README.md` is automatically generated by GitHub Actions using a Gemini translation based on `README.ja.md`)*