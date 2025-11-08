# Implementation Summary: Server/Client Architecture

## Overview
This implementation transforms cat-play-mml from a blocking, single-threaded player into a server/client architecture that allows background music playback while continuing to use the command line.

## Architecture

### Components

1. **Main Application** (`cat-play-mml.exe`)
   - Can operate in three modes: normal client, server, or control commands
   - Handles MML parsing, SMF conversion, and YM2151 log generation
   - Manages server lifecycle automatically

2. **Server Process** (detached child process)
   - Runs in background
   - Listens on Windows named pipe: `\\.\pipe\ym2151_server`
   - Handles playback commands (PLAY, STOP, SHUTDOWN)
   - Uses ym2151-log-play-server crate

3. **IPC Protocol** (via named pipes)
   - Commands: PLAY <path>, STOP, SHUTDOWN
   - Responses: OK, ERROR <message>
   - Text-based protocol with newline delimiters

## Workflow

### First Run (No Server)
```
User: cat-play-mml cde
  ↓
[Main Process]
  1. Parse "cde" as MML
  2. Generate SMF
  3. Generate YM2151 log JSON
  4. Save to temp file
  5. Check if server running → NO
  6. Spawn server process with --server <temp.json>
  7. Exit immediately
  ↓
[Server Process] (background)
  - Start named pipe listener
  - Load and play JSON
  - Wait for client commands
```

### Subsequent Runs (Server Running)
```
User: cat-play-mml efg
  ↓
[Main Process]
  1. Parse "efg" as MML
  2. Generate SMF
  3. Generate YM2151 log JSON
  4. Save to temp file
  5. Check if server running → YES
  6. Connect to server
  7. Send PLAY command with temp file path
  8. Wait for OK response
  9. Exit immediately
  ↓
[Server Process] (already running)
  - Receive PLAY command
  - Stop current playback
  - Load new JSON
  - Start new playback
  - Send OK response
```

### Stop Command
```
User: cat-play-mml --stop
  ↓
[Main Process]
  1. Connect to server
  2. Send STOP command
  3. Wait for OK response
  4. Exit
  ↓
[Server Process]
  - Receive STOP command
  - Stop playback (silence)
  - Keep running
  - Send OK response
```

### Shutdown Command
```
User: cat-play-mml --shutdown
  ↓
[Main Process]
  1. Connect to server
  2. Send SHUTDOWN command
  3. Wait for OK response
  4. Exit
  ↓
[Server Process]
  - Receive SHUTDOWN command
  - Stop playback
  - Send OK response
  - Exit cleanly
```

## Key Implementation Details

### Server Detection
- Attempts to connect to named pipe `\\.\pipe\ym2151_server`
- If connection succeeds, server is running
- If connection fails, no server is running

### Process Spawning (Windows-specific)
```rust
Command::new(exe_path)
    .arg("--server")
    .arg(json_path)
    .creation_flags(CREATE_NEW_PROCESS_GROUP | DETACHED_PROCESS)
    .spawn()
```
- `CREATE_NEW_PROCESS_GROUP`: Creates new process group
- `DETACHED_PROCESS`: Detaches from parent console
- Allows parent to exit while child continues

### Temporary File Management
- Uses `std::env::temp_dir()` for platform-appropriate temp directory
- Filename: `cat_play_mml_temp.json`
- Overwritten on each run
- Limitation: Multiple simultaneous instances may conflict (acceptable for single-user Windows)

### Error Handling
- All operations return `Result<T, anyhow::Error>`
- Proper context added with `.context()` for debugging
- Client errors (server not running) reported clearly
- Server errors propagated from server process

## Code Changes

### Cargo.toml
```toml
ym2151-log-play-server = { git = "https://github.com/cat2151/ym2151-log-play-server" }
```

### src/main.rs

**Added Imports:**
- `std::process::Command` - for spawning server
- `ym2151_log_play_server::client` - for client operations
- `ym2151_log_play_server::server::Server` - for server mode

**Updated Args Struct:**
- `input: Option<String>` - now optional
- Added `--server` flag with path parameter
- Added `--stop` flag
- Added `--shutdown` flag

**New Functions:**
- `is_server_running()` - detects server via named pipe
- `spawn_server_process()` - spawns detached server process
- `generate_json_from_input()` - extracted JSON generation logic

**Main Function Logic:**
1. Check for `--server` mode → run as server
2. Check for `--stop` → send stop command
3. Check for `--shutdown` → send shutdown command
4. Otherwise: normal playback mode with auto server management

## Benefits

1. **Non-blocking Operation**: Commands return immediately
2. **Background Playback**: Music continues while using terminal
3. **Easy Switching**: Quickly change music with new commands
4. **Resource Efficient**: Single server process handles all playback
5. **Clean Shutdown**: Explicit shutdown command prevents orphaned processes

## Limitations

1. **Windows Only**: Uses Windows named pipes
2. **Single Instance**: Temp file may conflict with multiple instances
3. **No Persistence**: Server shuts down when commanded or crashes
4. **No Playlist**: Only plays one track at a time

## Security Considerations

1. **Named Pipe Security**: Pipes are local-only, no network exposure
2. **No Shell Injection**: Direct process spawning, no shell commands
3. **File System Safety**: Uses platform temp directory
4. **Input Validation**: Existing MML parser handles input validation
5. **Process Isolation**: Server runs as separate process

## Future Enhancements

Potential improvements (out of scope for current implementation):
- Unique temp file names (UUID) for multi-instance support
- Persistent server configuration
- Playlist/queue support
- Server status command
- Log file for server diagnostics
- Graceful server restart on crash

## Testing Requirements

See `TESTING_GUIDE.md` for comprehensive testing procedures.

Must be tested on Windows as it uses Windows-specific APIs:
- Named pipes (`\\.\pipe\*`)
- Process creation flags
- Windows temp directory structure

Cannot be fully tested on Linux due to platform dependencies.
