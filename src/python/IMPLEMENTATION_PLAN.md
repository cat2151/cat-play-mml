# Python実装計画書

## 概要

Python版のMML（Music Macro Language）パーサーとプレーヤーを実装します。

## 目標

コマンドライン引数として"cde"を受け取り、ドレミを音声で再生するシンプルなアプリケーションを作成します。

## 使用ライブラリ

- **sounddevice**: リアルタイムオーディオ入出力
- **numpy**: 音声波形の生成と数値計算

## アーキテクチャ

```
play_mml.py (メイン)
  ├─ mml_parser.py (パーサー)
  │   └─ parse_mml() -> List[Note]
  │
  └─ audio_player.py (オーディオ再生)
      ├─ generate_sine_wave()
      └─ play_notes()
```

## データ構造

### Note クラス

```python
@dataclass
class Note:
    pitch: str          # 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'r' (rest)
    octave: int         # 4 (default)
    duration: float     # 0.5 (秒)
    frequency: float    # 計算された周波数 (Hz)
```

## 実装ステップ

### Phase 1: 基本構造の作成

1. **プロジェクト構造**
   ```
   src/python/
   ├── play_mml.py       # メインエントリーポイント
   ├── mml_parser.py     # MMLパーサー
   └── audio_player.py   # 音声生成・再生
   ```

2. **依存関係の確認**
   - requirements.txtに記載済み
   - インストール: `pip install -r requirements.txt`

### Phase 2: MMLパーサーの実装

**ファイル**: `src/python/mml_parser.py`

```python
from dataclasses import dataclass
from typing import List

@dataclass
class Note:
    pitch: str
    octave: int = 4
    duration: float = 0.5
    frequency: float = 0.0

def note_to_frequency(pitch: str, octave: int) -> float:
    """音階名とオクターブから周波数を計算"""
    # A4 = 440Hz を基準に計算
    # C4 = 261.63Hz
    note_offsets = {
        'c': -9, 'd': -7, 'e': -5, 'f': -4,
        'g': -2, 'a': 0, 'b': 2
    }
    offset = note_offsets.get(pitch.lower(), 0)
    semitones = (octave - 4) * 12 + offset
    return 440.0 * (2 ** (semitones / 12.0))

def parse_mml(mml_string: str) -> List[Note]:
    """MML文字列をパースしてNoteのリストを返す"""
    notes = []
    current_octave = 4
    current_duration = 0.5

    for char in mml_string.lower():
        if char in 'cdefgab':
            freq = note_to_frequency(char, current_octave)
            notes.append(Note(
                pitch=char,
                octave=current_octave,
                duration=current_duration,
                frequency=freq
            ))
        elif char == 'r':
            # 休符
            notes.append(Note(
                pitch='r',
                octave=current_octave,
                duration=current_duration,
                frequency=0.0
            ))

    return notes
```

### Phase 3: オーディオ再生の実装

**ファイル**: `src/python/audio_player.py`

```python
import numpy as np
import sounddevice as sd
from typing import List
from mml_parser import Note

SAMPLE_RATE = 48000

def generate_sine_wave(frequency: float, duration: float, sample_rate: int = SAMPLE_RATE) -> np.ndarray:
    """正弦波を生成"""
    if frequency == 0.0:
        # 休符は無音
        return np.zeros(int(sample_rate * duration))

    t = np.linspace(0, duration, int(sample_rate * duration), False)
    wave = np.sin(2 * np.pi * frequency * t)

    # エンベロープ（アタック・リリース）を追加して音を滑らかに
    attack_time = 0.01
    release_time = 0.05
    attack_samples = int(sample_rate * attack_time)
    release_samples = int(sample_rate * release_time)

    envelope = np.ones_like(wave)
    if len(wave) > attack_samples:
        envelope[:attack_samples] = np.linspace(0, 1, attack_samples)
    if len(wave) > release_samples:
        envelope[-release_samples:] = np.linspace(1, 0, release_samples)

    return wave * envelope * 0.3  # 音量を30%に

def play_notes(notes: List[Note], sample_rate: int = SAMPLE_RATE):
    """Noteのリストを順番に再生"""
    audio_data = []

    for note in notes:
        wave = generate_sine_wave(note.frequency, note.duration, sample_rate)
        audio_data.append(wave)

    # 全ての音を結合
    full_audio = np.concatenate(audio_data)

    # 再生
    sd.play(full_audio, sample_rate)
    sd.wait()  # 再生が終わるまで待機
```

### Phase 4: メインプログラムの実装

**ファイル**: `src/python/play_mml.py`

```python
#!/usr/bin/env python3
import sys
from mml_parser import parse_mml
from audio_player import play_notes

def main():
    if len(sys.argv) < 2:
        print("使用法: python play_mml.py <MML文字列>")
        print("例: python play_mml.py \"cde\"")
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
```

### Phase 5: テストと検証

1. **基本テスト**
   ```bash
   python src/python/play_mml.py "cde"
   ```
   期待結果: ドレミの音が順番に再生される

2. **拡張テスト**（将来の拡張）
   - 休符のテスト: `"cdefgabr>c"`
   - 長い曲のテスト

## 実行例

```bash
# インストール
pip install -r requirements.txt

# 実行
python src/python/play_mml.py "cde"

# 出力例:
# MMLを再生します: cde
# パース成功: 3個の音符
# 再生完了
```

## 将来の拡張（オプション）

以下の機能は、基本実装後に追加可能です：

1. **オクターブ制御**
   - `o4`: オクターブを4に設定
   - `>`: オクターブを1つ上げる
   - `<`: オクターブを1つ下げる

2. **音長制御**
   - `l4`: デフォルトの音長を4分音符に設定
   - `c4`: この音符だけ4分音符

3. **テンポ制御**
   - `t120`: テンポをBPM 120に設定

4. **半音階**
   - `c+` または `c#`: C#
   - `c-`: C♭

## 技術的な注意点

### 音声のプチノイズ対策

音の開始と終了時にクリックノイズが発生する可能性があるため、エンベロープ（アタック・リリース）を実装しています。

### サンプルレート

48000Hzを使用。これは一般的な音声再生に十分な品質です。

### ブロッキング再生

`sd.wait()`を使用して、音声の再生が完了するまで待機します。

## 推定工数

- **Phase 1-2**: 1時間（パーサー実装）
- **Phase 3**: 1時間（オーディオ再生実装）
- **Phase 4**: 30分（統合とテスト）
- **Phase 5**: 30分（ドキュメント作成）

**合計**: 約3時間

## 成功基準

- ✅ `python src/python/play_mml.py "cde"` でドレミが再生される
- ✅ エラーハンドリングが適切に実装されている
- ✅ コードがruffの基準に準拠している
- ✅ READMEに使用方法が記載されている
