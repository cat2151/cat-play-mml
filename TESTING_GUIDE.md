# Testing Guide for Server/Client Architecture

## Prerequisites
- Windows OS
- Rust toolchain installed
- Zig compiler installed (for Nuked-OPM C code compilation)

## Build Instructions

```bash
# Build the release version
cargo build --release

# The executable will be at:
# target/release/cat-play-mml.exe
```

## Test Scenarios

### Scenario 1: First Run (Server Auto-Start)

**Test Steps:**
1. Open PowerShell or Command Prompt
2. Run: `cat-play-mml cde`

**Expected Behavior:**
- Console output shows:
  - "Processing MML input..."
  - "Step 1: Converting MML to SMF..."
  - "Step 2: Converting SMF to YM2151 log..."
  - "Server is not running. Starting server..."
  - "Starting server process with JSON: ..."
  - "Server process spawned successfully"
  - "Operation completed."
- Command prompt returns immediately (non-blocking)
- Music "Do-Re-Mi" should start playing in the background
- A background server process should be running

**Verification:**
```powershell
# Check if server process is running
Get-Process cat-play-mml
```

### Scenario 2: Second Run (Client to Running Server)

**Test Steps:**
1. With server still running from Scenario 1
2. Run: `cat-play-mml efg`

**Expected Behavior:**
- Console output shows:
  - "Processing MML input..."
  - JSON generation messages
  - "Server is running. Sending JSON to server as client..."
  - Client sends message to server
  - "Operation completed."
- Command returns immediately
- Music changes to "Mi-Fa-So"
- Previous music stops

### Scenario 3: Stop Playback

**Test Steps:**
1. With server running and playing music
2. Run: `cat-play-mml --stop`

**Expected Behavior:**
- Console output: "Sending stop command to server..."
- Music stops (silence)
- Server process remains running
- Command returns immediately

### Scenario 4: Resume Playback After Stop

**Test Steps:**
1. With server stopped (from Scenario 3)
2. Run: `cat-play-mml cde`

**Expected Behavior:**
- Music starts playing again
- Server was already running, so it acts as client
- "Do-Re-Mi" plays

### Scenario 5: Shutdown Server

**Test Steps:**
1. With server running
2. Run: `cat-play-mml --shutdown`

**Expected Behavior:**
- Console output: "Sending shutdown command to server..."
- Music stops
- Server process terminates
- Command returns immediately

**Verification:**
```powershell
# Server process should not be found
Get-Process cat-play-mml
# Should return: "Get-Process: Cannot find a process with the name 'cat-play-mml'."
```

### Scenario 6: Multiple Rapid Commands

**Test Steps:**
1. Ensure no server is running
2. Run these commands quickly:
   ```
   cat-play-mml cde
   cat-play-mml efg
   cat-play-mml gab
   ```

**Expected Behavior:**
- First command starts server
- Second and third commands send to existing server
- Music should change with each command
- All commands return immediately
- Only one server process should be running

### Scenario 7: Manual Server Start

**Test Steps:**
1. Create a test JSON file (or use existing one)
2. Run: `cat-play-mml --server test.json`

**Expected Behavior:**
- Server starts in foreground (blocking mode)
- Detailed server logs appear
- Music starts playing
- Process does not return to command prompt
- In another terminal, client commands should work

### Scenario 8: File Input Types

**Test MML File:**
1. Create `test.mml` with content: `cdefgab`
2. Run: `cat-play-mml test.mml`

**Expected Behavior:**
- Reads from file
- Plays the scale

**Test MIDI File:**
1. Create or obtain a `.mid` file
2. Run: `cat-play-mml test.mid`

**Expected Behavior:**
- Converts MIDI to YM2151 log
- Plays the music

**Test JSON File:**
1. Run: `cat-play-mml output.json` (if you have a YM2151 log JSON)

**Expected Behavior:**
- Directly uses the JSON
- Plays the music

### Scenario 9: Error Handling

**Test: Stop with No Server**
1. Ensure no server is running
2. Run: `cat-play-mml --stop`

**Expected Behavior:**
- Error message: "Failed to stop playback"
- Details about server connection failure
- Non-zero exit code

**Test: Shutdown with No Server**
1. Ensure no server is running
2. Run: `cat-play-mml --shutdown`

**Expected Behavior:**
- Error message: "Failed to shutdown server"
- Details about server connection failure
- Non-zero exit code

## Cleanup After Testing

```powershell
# Stop any running server
cat-play-mml --shutdown

# Or force kill if needed
Stop-Process -Name cat-play-mml -Force

# Clean up temp files
Remove-Item $env:TEMP\cat_play_mml_temp.json -ErrorAction SilentlyContinue
```

## Common Issues and Troubleshooting

### Server Won't Start
- Check if port/named pipe is already in use
- Check Windows Event Viewer for errors
- Ensure audio device is available

### Client Can't Connect
- Verify server is running: `Get-Process cat-play-mml`
- Check named pipe path: `\\.\pipe\ym2151_server`
- Check for permission issues

### No Sound
- Verify audio output device is working
- Check Windows sound mixer
- Try with different MML input

### Multiple Server Processes
- Should not happen - indicates a bug
- Shutdown all: `cat-play-mml --shutdown` multiple times
- Force kill: `Stop-Process -Name cat-play-mml -Force`

## Performance Testing

### Latency Test
1. Run: `cat-play-mml cde` and measure time to return
2. Expected: < 1 second for command to complete
3. Expected: Music starts within 500ms

### Rapid Command Test
1. Send 10 commands in quick succession
2. All should complete successfully
3. Music should reflect the last command

### Long Playback Test
1. Create MML with long duration (e.g., multiple measures)
2. Verify server continues playing in background
3. Verify you can execute other commands during playback

## Success Criteria

All scenarios should pass with:
- ✅ Correct behavior as described
- ✅ No crashes or panics
- ✅ No memory leaks (verify with long-running tests)
- ✅ Proper error messages for error cases
- ✅ Fast command return times (< 1 second)
- ✅ Clean shutdown with no orphaned processes
