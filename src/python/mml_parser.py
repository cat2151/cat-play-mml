"""MML (Music Macro Language) Parser

This module provides functionality to parse MML strings into musical notes.
"""

from dataclasses import dataclass
from typing import List


@dataclass
class Note:
    """Represents a musical note."""

    pitch: str  # 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'r' (rest)
    octave: int = 4  # Default octave
    duration: float = 0.5  # Duration in seconds
    frequency: float = 0.0  # Calculated frequency in Hz


def note_to_frequency(pitch: str, octave: int) -> float:
    """Calculate frequency from pitch and octave.

    Args:
        pitch: Note pitch ('c', 'd', 'e', 'f', 'g', 'a', 'b')
        octave: Octave number (4 is middle octave)

    Returns:
        Frequency in Hz (A4 = 440Hz)
    """
    # A4 = 440Hz is the reference
    # C4 = 261.63Hz
    note_offsets = {"c": -9, "d": -7, "e": -5, "f": -4, "g": -2, "a": 0, "b": 2}

    offset = note_offsets.get(pitch.lower(), 0)
    semitones = (octave - 4) * 12 + offset
    return 440.0 * (2 ** (semitones / 12.0))


def parse_mml(mml_string: str) -> List[Note]:
    """Parse MML string into a list of notes.

    Args:
        mml_string: MML string (e.g., "cde", "cdefgab")

    Returns:
        List of Note objects
    """
    notes = []
    current_octave = 4
    current_duration = 0.5

    for char in mml_string.lower():
        if char in "cdefgab":
            freq = note_to_frequency(char, current_octave)
            notes.append(Note(pitch=char, octave=current_octave, duration=current_duration, frequency=freq))
        elif char == "r":
            # Rest (silence)
            notes.append(Note(pitch="r", octave=current_octave, duration=current_duration, frequency=0.0))
        # Ignore other characters (spaces, etc.)

    return notes
