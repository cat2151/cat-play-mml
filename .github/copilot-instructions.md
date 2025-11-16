# GitHub Copilot Instructions for cat-play-mml

## Project Overview

`cat-play-mml` is a Music Macro Language (MML) Parser and Player project designed to parse MML text and play music in real-time. The project aims to create a Windows Rust executable that can play Do-Re-Mi when "cde" is specified as a command-line argument.

## Technical Stack

### Primary Technologies
- **Rust**: Main implementation language for the Windows executable
- **tree-sitter**: MML grammar parsing
- **cpal**: Cross-platform audio library (WASAPI on Windows)
- **Nuked-OPM**: YM2151 sound chip emulator

### Supporting Technologies
- **Python**: Development and testing scripts
- **Go**: Potential future components
- **TypeScript/JavaScript**: Potential future components

## Architecture

The project follows a pipeline architecture:

```
MML Text Input (e.g., "cde")
    ↓
[mmlabc-to-smf] → Standard MIDI File (in-memory)
    ↓
[smf-to-ym2151log] → YM2151 Register Log (JSON)
    ↓
[ym2151-log-player] → Audio Playback
```

### Component Repositories

1. **mmlabc-to-smf-rust**: https://github.com/cat2151/mmlabc-to-smf-rust
   - Parses MML text using tree-sitter and converts to Standard MIDI File format

2. **smf-to-ym2151log-rust**: https://github.com/cat2151/smf-to-ym2151log-rust
   - Converts SMF to YM2151 register write log (JSON format)

3. **ym2151-log-play-server**: https://github.com/cat2151/ym2151-log-play-server
   - Plays YM2151 register logs using Nuked-OPM emulator

## Code Style and Conventions

### EditorConfig Standards
Follow the `.editorconfig` file in the repository root:

- **All files**: UTF-8 encoding, LF line endings, insert final newline, trim trailing whitespace
- **Rust files**: 4 spaces, max line length 100 characters
- **Python files**: 4 spaces, max line length 120 characters
- **Go files**: Tab indentation, size 4
- **TypeScript/JavaScript**: 2 spaces
- **YAML/JSON**: 2 spaces
- **TOML**: 2 spaces
- **Markdown**: 2 spaces, preserve trailing whitespace

### Rust-Specific Guidelines

- Use `anyhow` for error handling in applications
- Use `clap` with derive feature for command-line argument parsing
- Prefer memory-based data passing over file I/O for performance
- Use Cargo git dependencies for integrating external crates
- Follow Rust naming conventions (snake_case for functions/variables, CamelCase for types)
- Keep code modular and maintainable with clear separation of concerns

### Git and Version Control

- Use meaningful commit messages
- Follow conventional commits format when applicable
- Tag releases with semantic versioning (e.g., v0.1.0)

## Project Goals and Constraints

### Current Goals
- Implement a single Windows executable that accepts MML as command-line argument
- Real-time music playback with low latency
- No intermediate file creation (all processing in memory)
- Support for basic MML syntax (initially "cde" for Do-Re-Mi)

### In Scope
- File output of intermediate representation (including Standard MIDI Files)
- Basic MML commands:
  - Notes: `c`, `d`, `e`, `f`, `g`, `a`, `b` (e.g., `c4` for quarter note C)
  - Octave: `o` (e.g., `o4` for 4th octave)
  - Length: `l` (e.g., `l8` for eighth notes)
  - Tempo: `t` (e.g., `t120` for 120 BPM)
  - Rest: `r` (e.g., `r4` for quarter rest)
- Octave shifting: `<` (lower), `>` (raise)
- Semitone adjustment: `+` (raise), `-` (lower)

### Out of Scope
- Complex MML features
- Real-time MIDI message output
- Effects processing (LPF, overdrive/distortion, delay)
- GUI editor
- Non-Windows platforms (currently)

## Development Approach

### Test-Driven Development (TDD)
- Consider using TDD agents for new feature development
- Write tests before implementing functionality when practical
- Ensure tests cover edge cases and error scenarios

### Error Handling
- Provide clear, user-friendly error messages
- Handle common error scenarios:
  - Invalid MML syntax
  - SMF conversion errors
  - Audio backend initialization failures
  - Audio device unavailable

### Performance Considerations
- Minimize latency for real-time playback
- Optimize memory usage for large MML files
- Use efficient data structures for audio processing
- Profile code to identify bottlenecks

## Documentation Standards

### README Files
- Maintain both English (`README.md`) and Japanese (`README.ja.md`) versions
- Japanese version is the source, English is auto-translated via GitHub Actions
- Keep technical details accurate and up-to-date
- Include usage examples and command-line syntax

### Code Comments
- Document public APIs and complex algorithms
- Explain non-obvious design decisions
- Use doc comments for Rust modules, functions, and types
- Keep comments concise and relevant

### Implementation Plans
- Document major architectural decisions
- Include alternatives considered and reasons for choices
- Maintain traceability between requirements and implementation

## Testing Guidelines

### Unit Tests
- Write unit tests for core functionality
- Test edge cases and error conditions
- Use descriptive test names that explain what is being tested
- Keep tests focused and independent

### Integration Tests
- Test the complete pipeline from MML input to audio output
- Verify data flow between components
- Test with various MML inputs (starting with "cde")

### Manual Testing
- Test on Windows platform with actual audio hardware
- Verify real-time playback latency
- Test command-line interface usability

## Dependency Management

### Cargo Dependencies
- Use git dependencies for integrating the three main components
- Pin to specific tags or commit hashes for reproducible builds
- Document required dependency versions in `Cargo.toml`
- Minimize external dependencies to reduce build complexity

### Upstream Changes
- Upstream repositories must provide library interfaces (not just binaries)
- Coordinate changes with upstream maintainers when needed
- Test integration after updating upstream dependencies

## Build and Deployment

### Build Process
- Use `cargo build` for development builds
- Use `cargo build --release` for optimized builds
- Ensure C compiler availability (MSVC or MinGW) for Nuked-OPM
- Target: `x86_64-pc-windows-msvc` or `x86_64-pc-windows-gnu`

### Windows-Specific Considerations
- Use default console subsystem for command-line usage
- Ensure WASAPI audio backend support via cpal
- Handle Windows-specific file path conventions

## Workflow and Automation

### GitHub Actions
- Daily project summaries generated automatically
- README translation from Japanese to English on push to main
- Issue notes automatically processed

### Secrets and API Keys
- `GEMINI_API_KEY` used for AI-powered workflows
- Never commit secrets to source code
- Use GitHub secrets for sensitive data

## Best Practices

### Code Quality
- Run `cargo clippy` for Rust linting
- Run `cargo fmt` for consistent formatting
- Address compiler warnings before committing
- Review code for security vulnerabilities

### Collaboration
- Write clear issue descriptions with acceptance criteria
- Review pull requests thoroughly
- Provide constructive feedback
- Keep discussions focused and professional

### Security
- Never commit credentials or API keys
- Validate all user inputs
- Handle audio device permissions appropriately
- Be cautious with external dependencies

## Folder Structure

```
cat-play-mml/
├── .github/              # GitHub-specific files (workflows, this instructions file)
├── .vscode/              # VS Code settings
├── issue-notes/          # Issue documentation
├── src/                  # Source code
├── Cargo.toml            # Rust project configuration
├── README.md             # English documentation (auto-translated)
├── README.ja.md          # Japanese documentation (source)
├── IMPLEMENTATION_PLAN.md # Detailed implementation planning
├── LICENSE               # MIT License
├── .editorconfig         # Editor configuration
└── .gitignore            # Git ignore patterns
```

## Language-Specific Notes

### When Working with Rust
- Follow the Rust API Guidelines
- Use `Result<T, E>` for operations that can fail
- Prefer owned types over references when it simplifies the API
- Use builder patterns for complex object construction
- Leverage the type system for compile-time guarantees

### When Working with Python (if applicable)
- Follow PEP 8 style guide
- Use type hints for function signatures
- Keep functions focused and single-purpose
- Use virtual environments for dependency isolation

### When Working with YAML
- Use 2-space indentation
- Keep workflow files clear and well-commented
- Use reusable workflows when appropriate

## Communication

### Issue Tracking
- Create clear, focused issues with specific goals
- Include reproduction steps for bugs
- Tag issues appropriately
- Update issue status as work progresses

### Pull Requests
- Keep PRs focused on a single concern
- Write descriptive PR titles and descriptions
- Link to related issues
- Ensure CI passes before requesting review

## Related Resources

- [Cargo Book: Git Dependencies](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#specifying-dependencies-from-git-repositories)
- [cpal Audio Library](https://github.com/RustAudio/cpal)
- [tree-sitter](https://tree-sitter.github.io/tree-sitter/)
- [MML Syntax Reference](https://en.wikipedia.org/wiki/Music_Macro_Language)
