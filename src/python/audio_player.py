"""Audio Player for MML

This module provides functionality to generate and play audio from MML notes.
"""

from typing import List

import numpy as np
import sounddevice as sd
from mml_parser import Note

SAMPLE_RATE = 48000


def generate_sine_wave(frequency: float, duration: float, sample_rate: int = SAMPLE_RATE) -> np.ndarray:
    """Generate a sine wave.

    Args:
        frequency: Frequency in Hz (0 for silence/rest)
        duration: Duration in seconds
        sample_rate: Sample rate in Hz

    Returns:
        NumPy array of audio samples
    """
    if frequency == 0.0:
        # Rest - return silence
        return np.zeros(int(sample_rate * duration))

    t = np.linspace(0, duration, int(sample_rate * duration), False)
    wave = np.sin(2 * np.pi * frequency * t)

    # Add envelope (attack and release) to smooth the sound
    attack_time = 0.01  # 10ms attack
    release_time = 0.05  # 50ms release
    attack_samples = int(sample_rate * attack_time)
    release_samples = int(sample_rate * release_time)

    envelope = np.ones_like(wave)
    if len(wave) > attack_samples:
        envelope[:attack_samples] = np.linspace(0, 1, attack_samples)
    if len(wave) > release_samples:
        envelope[-release_samples:] = np.linspace(1, 0, release_samples)

    return wave * envelope * 0.3  # Set volume to 30%


def play_notes(notes: List[Note], sample_rate: int = SAMPLE_RATE):
    """Play a list of notes in sequence.

    Args:
        notes: List of Note objects
        sample_rate: Sample rate in Hz
    """
    audio_data = []

    for note in notes:
        wave = generate_sine_wave(note.frequency, note.duration, sample_rate)
        audio_data.append(wave)

    # Concatenate all notes
    full_audio = np.concatenate(audio_data)

    # Play audio
    sd.play(full_audio, sample_rate)
    sd.wait()  # Wait until playback is finished
