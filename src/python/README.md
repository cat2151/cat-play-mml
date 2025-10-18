# Python MML Player

Python implementation of the MML (Music Macro Language) parser and player.

## Status

✅ Implemented and working

## Requirements

- Python 3.8+
- pip

## Installation

```bash
pip install -r requirements.txt
```

## Usage

```bash
python src/python/play_mml.py "cde"
```

This will play the notes C, D, E (ドレミ) in sequence.

### More Examples

```bash
# Play a scale
python src/python/play_mml.py "cdefgab"

# With rests
python src/python/play_mml.py "cderder"
```

## Features

- Simple MML parser supporting basic notes (c, d, e, f, g, a, b)
- Rest notes (r)
- Low-latency audio playback using sounddevice
- Sine wave synthesis with envelope (attack/release)

## Implementation Details

See [IMPLEMENTATION_PLAN.md](IMPLEMENTATION_PLAN.md) for details.
