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

`cat-play-mml` is a CLI tool that plays music using Music Macro Language (MML). If you input the string `cde`, it will play the music `Do-Re-Mi`. It is for Windows.

## Quick Start Guide

### Environment Setup
- Install `Rust` and `Zig` on Windows.

### Installation
```
cargo install --git https://github.com/cat2151/cat-play-mml.git --branch main --bin cat-play-mml
```

That's it! It will be installed on your Windows machine directly from GitHub.

### Playing Music
```
cat-play-mml cde
```

Do-Re-Mi will play.

## Future Considerations for Related Project Issues

- [mmlabc-to-smf-rust](https://github.com/cat2151/mmlabc-to-smf-rust/issues):
  - Implement MML `;`. `c;e;g` will become a Do-Mi-Sol chord. To allow separate timbres to be assigned in the future, it should be ch1 Do, ch2 Mi, ch3 Sol (1-based notation) rather than ch1 Do-Mi-Sol.

- [smf-to-ym2151log-rust](https://github.com/cat2151/smf-to-ym2151log-rust/issues)
  - Along with the MML `;` implementation, support multiple channels. Chords within a channel will be considered separately, and for now, undefined behavior is acceptable.

- [ym2151-log-player-rust](https://github.com/cat2151/ym2151-log-player-rust/issues)

### Key Features

- **Simple, Play Instantly**: Just pass "cde" as an argument to play Do-Re-Mi.
- **Low Latency**: Real-time music playback.

## What is MML (Music Macro Language)?

MML is a language for describing music using text. It uses notations such as the following:

- `c`, `d`, `e`, `f`, `g`, `a`, `b`: musical notes (Do, Re, Mi, Fa, Sol, La, Si)

### Planned for Future Implementation
- `o4`: Octave setting (4th octave)
- `l4`: Note length setting (quarter note)
- `t120`: Tempo setting (BPM 120)
- `<`, `>`: Raise/lower octave
- `+`, `-`: Raise/lower semitone
- `r`: Rest

## Technical Details

### Architecture

1.  **Parser**: Converts MML text to AST using tree-sitter.
2.  **Intermediate Representation**: Converts AST to a music data structure.
3.  **Audio Generation**: Generates audio waveforms from the intermediate representation.
4.  **Playback**: Outputs audio using an audio library.

### Development Environment

-   Windows
-   Rust
-   Zig cc (mingw and msys2 are prohibited)
-   Linux runner for agent's TDD (it's sufficient for the agent to be able to do TDD) + ALSA SDK and setup (to enable TDD in a headless environment)

### Libraries Used

-   Nuked-OPM
-   **Rust**: cpal

## Project Goals

### Short-term Goal
- [x] For the Windows Rust executable, when "cde" is specified as a command-line argument, Do-Re-Mi should play in real-time.

### Next Goal
- mmlabc syntax
  - High Priority: `;`
- File output of intermediate representation (including Standard MIDI files)

## Out of Scope

-   Complex MML
-   Real-time MIDI message output
-   Effects (e.g., LPF, overdrive/distortion, delay)
-   GUI editor

## Related Projects

### cat-play-chord (Under Consideration)

[cat-play-chord](https://github.com/cat2151/cat-play-chord) is a project (under consideration) that generates MML from chord notation using chord2mml and plays it.

### Future Outlook for MML to SMF
-   Current Status
    -   Only 'cde' has been implemented so far.
        -   Goal: Initially, focus on a minimal implementation to smoothly resolve issues until real-time Do-Re-Mi playback is achieved.
    -   Using SMF (Standard MIDI File)
        -   Goal: Using SMF makes verification and development easier, reducing the risk of development stagnation.
    -   We plan to use mmlabc as the MML dialect, as we have expertise in it and its format is clear.
    -   We plan to proceed with a TDD agent; if "hallucinations" occur, we will re-evaluate.

### smf to Nuked-OPM friendly JSON (Under Consideration)
-   Example
    -   Delay vibrato
        -   Based on the 'tone modify delay vibrato' value in the tone settings toml,
            -   from an OPM sound driver perspective,
                -   generate soft LFO register events for each tick.
        -   Separating SMF and toml is intended to simplify the MIDI implementation on the SMF side.
        -   This makes destructive changes to timbres and OPM sound driver-like processing easier on the toml side, following the ETC principle.
-   The following passes are envisioned:
    -   SMF to Intermediate Representation *Assumes JSON representing SMF as text
    -   Intermediate Representation to Intermediate Representation *n times *Delay vibrato is envisioned here
    -   Intermediate Representation to Nuked-OPM friendly JSON
-   We plan to proceed with a Linux Python TDD agent; if "hallucinations" occur, we will re-evaluate.

### Nuked-OPM friendly JSON player
-   Already implemented (log player in a separate repository)
-   Its purpose is to facilitate development.
    -   Easier debugging and reduced risk of development stagnation.

### Real-time FM Tone Editor (Provisional, Under Consideration)
-   Purpose of writing this here
    -   Rubber ducking
-   Usage
    -   For verification
-   Priority
    -   Prioritize ease of development and the ability to quickly sketch out timbres with minimal operations.
-   Operations
    -   Right hand: Increase/decrease assigned values with mouse x,y respectively.
    -   Left hand: Cursor movement with WASD+QE, SPACE for confirmation, ESC for cancellation, Z for UNDO. (Details are fuzzy)
    -   x,y switch between DecayRate, ModulatorTL, FB, MUL with cursor movement.
    -   WASD+QE, UNDO are fuzzy.
    -   Instead of cursor movement and confirmation, pressing a specific key to instantly apply an effect for faster editing might be more efficient; this should also be explored, perhaps with a settings toml file.
-   Sound
    -   Timbres switch between pluck and long tone with cursor movement.
    -   OP connection algorithm is 2-op parallel, detune has 4 different values.
-   Display
    -   Windows TUI 80x24
    -   Tone parameter display is in mdx note.x format.
    -   While running, save the tone in mdx note.x format to the clipboard 1 second after the last numerical change; output is limited to this. (Provisional)
-   All specifications are provisional, destructive changes will be frequent, prioritizing ease of development. Even with this, the specifications are still too numerous (too many to start small), so we will begin with a provisional implementation based on a more focused set of specifications.

## Additional Notes

### Build, Install, and Run Methods for Developers

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

*The English README.md is automatically generated from README.ja.md via Gemini's translation as part of GitHub Actions.