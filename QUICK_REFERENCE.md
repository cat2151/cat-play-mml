# Quick Reference Guide

## Basic Usage

### Play Music (Auto-manages server)
```powershell
# Play a simple melody
cat-play-mml cde

# Play from a file
cat-play-mml song.mml

# Play from MIDI
cat-play-mml music.mid

# Play from YM2151 log JSON
cat-play-mml output.json
```

**What happens:**
- First time: Starts server in background, plays music, returns immediately
- Next times: Sends to existing server, switches music, returns immediately

### Control Commands

```powershell
# Stop playback (server keeps running)
cat-play-mml --stop

# Shutdown server
cat-play-mml --shutdown

# Start server manually (advanced)
cat-play-mml --server output.json
```

## Common Workflows

### Play a sequence of songs
```powershell
cat-play-mml cde      # Starts server, plays Do-Re-Mi
cat-play-mml efg      # Switches to Mi-Fa-So
cat-play-mml gab      # Switches to So-La-Ti
cat-play-mml --stop   # Stops playback
```

### Clean shutdown
```powershell
cat-play-mml --shutdown
```

### Check if server is running
```powershell
Get-Process cat-play-mml
```

## MML Syntax Reference

Currently supported:
- `c d e f g a b` - Notes (Do Re Mi Fa So La Ti)
- Octaves and lengths will be added in future updates

## File Types Supported

| Extension | Description |
|-----------|-------------|
| `.mml` | MML source file |
| `.mid` | Standard MIDI file |
| `.json` | YM2151 log file |
| (none) | Treated as MML string |

## Troubleshooting

### "Failed to stop playback" or "Failed to shutdown server"
**Problem:** Server is not running  
**Solution:** This is normal if you haven't started playback yet

### No sound
**Check:**
1. Is audio device working? Test with other apps
2. Is server actually running? `Get-Process cat-play-mml`
3. Try playing simple MML: `cat-play-mml cde`

### Multiple server processes
**Problem:** Shouldn't happen, but if you see multiple processes  
**Solution:**
```powershell
cat-play-mml --shutdown   # Try graceful shutdown first
Stop-Process -Name cat-play-mml -Force  # Force if needed
```

### Command hangs
**Problem:** Stuck waiting for response  
**Possible causes:**
- Server crashed but pipe still exists
- Permission issues
**Solution:**
```powershell
# Kill all instances
Stop-Process -Name cat-play-mml -Force
# Delete temp file
Remove-Item $env:TEMP\cat_play_mml_temp.json
# Try again
cat-play-mml cde
```

## Tips

### Fast music switching
Commands return immediately, so you can chain them:
```powershell
cat-play-mml song1.mml; Start-Sleep -Seconds 5; cat-play-mml song2.mml
```

### Background playback
The whole point! Do this:
```powershell
cat-play-mml background-music.mml
# Now do other work in the same terminal
cd C:\Projects\my-project
code .
# Music keeps playing!
```

### Clean up before closing terminal
```powershell
cat-play-mml --shutdown
```

## Architecture Notes

- **Server process:** Background process handling playback
- **Client process:** Your commands (returns immediately)
- **Communication:** Windows named pipes (`\\.\pipe\ym2151_server`)
- **Temp file:** `%TEMP%\cat_play_mml_temp.json`

## See Also

- `TESTING_GUIDE.md` - Comprehensive testing procedures
- `IMPLEMENTATION_SUMMARY.md` - Technical architecture details
- `README.ja.md` - Full project documentation
