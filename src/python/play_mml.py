#!/usr/bin/env python3
"""MML Player - Main Entry Point

Play Music Macro Language (MML) strings as audio.
"""

import sys

from audio_player import play_notes
from mml_parser import parse_mml


def main():
    """Main function."""
    if len(sys.argv) < 2:
        print("使用法: python play_mml.py <MML文字列>")
        print('例: python play_mml.py "cde"')
        sys.exit(1)

    mml_string = sys.argv[1]
    print(f"MMLを再生します: {mml_string}")

    try:
        notes = parse_mml(mml_string)
        print(f"パース成功: {len(notes)}個の音符")

        play_notes(notes)
        print("再生完了")

    except Exception as e:
        print(f"エラー: {e}", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()
