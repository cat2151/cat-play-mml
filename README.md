# cat-play-mml

🎵 Music Macro Language (MML) Parser and Player

## Status

A Windows Rust executable is planned for implementation.

## Overview

`cat-play-mml` is a multi-language project that parses Music Macro Language (MML) and plays music. It uses tree-sitter to parse MML into an AST, converts it to an intermediate representation, and then plays the music.

### Key Features

- **MML Parser**: tree-sitter based MML grammar definition
- **Low Latency**: Real-time music playback
- **Simple Command-line Arguments**: Just pass "cde" as an argument to play Do-Re-Mi.

## What is MML (Music Macro Language)?

MML is a language for describing music using text. It uses the following notation:

- `c`, `d`, `e`, `f`, `g`, `a`, `b`: Musical notes (Do, Re, Mi, Fa, So, La, Si)
- `o4`: Octave setting (4th octave)
- `l4`: Note length setting (quarter note)
- `t120`: Tempo setting (BPM 120)
- `<`, `>`: Change octave up/down
- `+`, `-`: Semitone up/down
- `r`: Rest

## Technical Details

### Architecture

1.  **Parser**: Converts MML text to AST using tree-sitter
2.  **Intermediate Representation**: Converts AST to a music data structure
3.  **Audio Generation**: Generates audio waveforms from the intermediate representation
4.  **Playback**: Outputs audio using an audio library

### Audio Libraries Used

-   **Rust**: cpal

## Project Goals

-   As a Windows Rust executable, when "cde" is specified as a command-line argument, Do-Re-Mi should play in real-time.

## Anticipated Implementations

-   File output of intermediate representation (including Standard MIDI Files)

## Out of Scope

-   Complex MML
-   Real-time MIDI message output
-   Effects (e.g., LPF, overdrive/distortion, delay)
-   GUI editor

## Related Projects

### cat-play-chord (Under Consideration)

[cat-play-chord](https://github.com/cat2151/cat-play-chord) is a project that generates and plays MML from chord notation using chord2mml (under consideration).

### Future Outlook for MML to SMF

-   **Status**
    -   Only "cde" has been implemented.
        -   **Goal**: Initially, focus on a minimal implementation to smoothly resolve issues until real-time Do-Re-Mi playback is achieved.
    -   SMF is being used.
        -   **Goal**: Using SMF makes verification and development easier, reducing the risk of development stalling.
-   The MML dialect is assumed to be `mmlabc`, with existing know-how and a clear format.
-   Expected to proceed with a TDD agent; hallucinations will be considered if they occur.

### SMF to Nuked-OPM friendly JSON (Under Consideration)

-   **Example**
    -   **Delay Vibrato**
        -   Based on the `tone modify delay vibrato` value in the tone settings TOML,
            -   From an OPM sound driver perspective,
                -   Generate soft LFO register events every tick.
        -   Separating SMF and TOML is intended to simplify the MIDI implementation on the SMF side.
            -   This makes it easier to introduce breaking changes to timbre and OPM sound driver-like processing on the TOML side (ETC principle).
-   **The following passes are anticipated:**
    -   SMF to Intermediate Representation _(Assumes JSON representing SMF as text)_
    -   Intermediate Representation to Intermediate Representation _(n times; delay vibrato is expected here)_
    -   Intermediate Representation to Nuked-OPM friendly JSON
-   Expected to proceed with a Linux Python TDD agent; hallucinations will be considered if they occur.

### Nuked-OPM friendly JSON player

-   Already implemented (log player in a separate repository)
-   **Purpose**: To facilitate development.
    -   Easier to debug and reduces the risk of development stalling.

### Real-time FM tone editor (Provisional, Under Consideration)

-   **Purpose of writing this here**
    -   Rubber ducking
-   **Use Case**
    -   For verification
-   **Priority**
    -   Prioritize ease of development and the ability to sketch out rough timbres with minimal operations.
-   **Operation**
    -   **Right hand**: Mouse x,y for increasing/decreasing values assigned to each.
    -   **Left hand**: WASD+QE for cursor movement, SPACE to confirm, ESC to cancel, Z to undo (this part is vague/flexible).
    -   x,y switch between DecayRate, ModulatorTL, FB, MUL with cursor movement.
    -   WASD+QE, UNDO are vague/flexible.
    -   Instead of cursor movement and confirmation, pressing a specific key to instantly apply an effect for fast editing might increase edit speed, so this will also be tried (e.g., via a settings TOML file).
-   **Sound**
    -   Switch between pluck and long tone timbres with cursor movement.
    -   OP connection algorithm is 2-op parallel, detune has 4 different values.
-   **Display**
    -   Windows TUI 80x24
    -   Timbre parameter display in mdx note.x format.
    -   While running, save the timbre in mdx note.x format to the clipboard 1 second after the last value change; output is limited to this (provisional).
-   All specifications are provisional, frequent breaking changes will occur, prioritizing ease of development.
-   Even this is still too many specifications (too many for a small start), so we will start small with a provisional implementation with more focused specifications.

## License

This project is released under the [MIT License](LICENSE).