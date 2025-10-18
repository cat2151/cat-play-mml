# Rust実装計画書

## 概要

Rust版のMML（Music Macro Language）パーサーとプレーヤーを実装します。

## 目標

コマンドライン引数として"cde"を受け取り、ドレミを音声で再生するシンプルなアプリケーションを作成します。

## 使用ライブラリ

- **cpal**: クロスプラットフォームオーディオI/Oライブラリ
- **標準ライブラリ**: パーサー実装用

## プロジェクト構造

```
src/rust/
├── Cargo.toml
├── src/
│   ├── main.rs          # メインエントリーポイント
│   ├── parser.rs        # MMLパーサー
│   ├── audio.rs         # オーディオ生成・再生
│   └── lib.rs           # ライブラリルート
├── README.md
└── IMPLEMENTATION_PLAN.md (このファイル)
```

## データ構造

```rust
#[derive(Debug, Clone)]
pub struct Note {
    pub pitch: char,      // 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'r'
    pub octave: u8,       // 4 (default)
    pub duration: f32,    // 0.5 (秒)
    pub frequency: f32,   // 計算された周波数 (Hz)
}
```

## 実装ステップ

### Phase 1: プロジェクトセットアップ

1. **Cargo.tomlの作成**

```toml
[package]
name = "cat-play-mml-rust"
version = "0.1.0"
edition = "2021"

[dependencies]
cpal = "0.15"
```

2. **ディレクトリ構造の作成**
   ```bash
   mkdir -p src/rust/src
   ```

### Phase 2: MMLパーサーの実装

**ファイル**: `src/rust/src/parser.rs`

```rust
#[derive(Debug, Clone)]
pub struct Note {
    pub pitch: char,
    pub octave: u8,
    pub duration: f32,
    pub frequency: f32,
}

impl Note {
    pub fn new(pitch: char, octave: u8, duration: f32) -> Self {
        let frequency = Self::calculate_frequency(pitch, octave);
        Self {
            pitch,
            octave,
            duration,
            frequency,
        }
    }

    fn calculate_frequency(pitch: char, octave: u8) -> f32 {
        // A4 = 440Hz を基準に計算
        let note_offset = match pitch.to_ascii_lowercase() {
            'c' => -9,
            'd' => -7,
            'e' => -5,
            'f' => -4,
            'g' => -2,
            'a' => 0,
            'b' => 2,
            'r' => return 0.0, // 休符
            _ => 0,
        };

        let semitones = (octave as i32 - 4) * 12 + note_offset;
        440.0 * 2.0_f32.powf(semitones as f32 / 12.0)
    }
}

pub fn parse_mml(mml: &str) -> Vec<Note> {
    let mut notes = Vec::new();
    let current_octave = 4;
    let current_duration = 0.5;

    for ch in mml.chars() {
        match ch.to_ascii_lowercase() {
            'c' | 'd' | 'e' | 'f' | 'g' | 'a' | 'b' | 'r' => {
                notes.push(Note::new(ch, current_octave, current_duration));
            }
            _ => {} // スペースなどは無視
        }
    }

    notes
}
```

### Phase 3: オーディオ生成の実装

**ファイル**: `src/rust/src/audio.rs`

```rust
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Sample, SampleFormat, StreamConfig};
use std::f32::consts::PI;

use crate::parser::Note;

const SAMPLE_RATE: u32 = 48000;

pub fn play_notes(notes: &[Note]) -> Result<(), Box<dyn std::error::Error>> {
    let host = cpal::default_host();
    let device = host.default_output_device()
        .ok_or("No output device available")?;

    let config = StreamConfig {
        channels: 1,
        sample_rate: cpal::SampleRate(SAMPLE_RATE),
        buffer_size: cpal::BufferSize::Default,
    };

    // 全ての音をバッファに生成
    let mut samples = Vec::new();
    for note in notes {
        let note_samples = generate_sine_wave(note.frequency, note.duration);
        samples.extend(note_samples);
    }

    let mut sample_index = 0;
    let samples_clone = samples.clone();

    let stream = device.build_output_stream(
        &config,
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            for sample in data.iter_mut() {
                *sample = if sample_index < samples_clone.len() {
                    let val = samples_clone[sample_index];
                    sample_index += 1;
                    val
                } else {
                    0.0
                };
            }
        },
        |err| eprintln!("Stream error: {}", err),
        None,
    )?;

    stream.play()?;

    // 再生が終わるまで待つ
    let duration = samples.len() as f32 / SAMPLE_RATE as f32;
    std::thread::sleep(std::time::Duration::from_secs_f32(duration + 0.1));

    Ok(())
}

fn generate_sine_wave(frequency: f32, duration: f32) -> Vec<f32> {
    let num_samples = (SAMPLE_RATE as f32 * duration) as usize;
    let mut samples = Vec::with_capacity(num_samples);

    if frequency == 0.0 {
        // 休符
        return vec![0.0; num_samples];
    }

    for i in 0..num_samples {
        let t = i as f32 / SAMPLE_RATE as f32;
        let mut value = (2.0 * PI * frequency * t).sin();

        // エンベロープ（アタック・リリース）
        let attack_samples = (SAMPLE_RATE as f32 * 0.01) as usize;
        let release_samples = (SAMPLE_RATE as f32 * 0.05) as usize;

        if i < attack_samples {
            value *= i as f32 / attack_samples as f32;
        } else if i > num_samples - release_samples {
            value *= (num_samples - i) as f32 / release_samples as f32;
        }

        samples.push(value * 0.3); // 音量を30%に
    }

    samples
}
```

### Phase 4: メインプログラムの実装

**ファイル**: `src/rust/src/main.rs`

```rust
mod parser;
mod audio;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("使用法: {} <MML文字列>", args[0]);
        eprintln!("例: {} \"cde\"", args[0]);
        std::process::exit(1);
    }

    let mml_string = &args[1];
    println!("MMLを再生します: {}", mml_string);

    let notes = parser::parse_mml(mml_string);
    println!("パース成功: {}個の音符", notes.len());

    match audio::play_notes(&notes) {
        Ok(_) => println!("再生完了"),
        Err(e) => {
            eprintln!("エラー: {}", e);
            std::process::exit(1);
        }
    }
}
```

### Phase 5: READMEの作成

**ファイル**: `src/rust/README.md`

```markdown
# Rust MML Player

Music Macro Language (MML) parser and player implemented in Rust.

## Requirements

- Rust 1.70+
- Cargo

## Build

\`\`\`bash
cd src/rust
cargo build --release
\`\`\`

## Usage

\`\`\`bash
cargo run -- "cde"
\`\`\`

Or with the compiled binary:

\`\`\`bash
./target/release/cat-play-mml-rust "cde"
\`\`\`

## Features

- Simple MML parser supporting basic notes (c, d, e, f, g, a, b)
- Rest notes (r)
- Low-latency audio playback using cpal
- Cross-platform support (Windows, macOS, Linux)

## Example

\`\`\`bash
cargo run -- "cdefgab"
\`\`\`
```

## ビルドとテスト

```bash
# ビルド
cd src/rust
cargo build

# 実行
cargo run -- "cde"

# リリースビルド
cargo build --release

# テスト
cargo test
```

## .gitignoreの追加

**ファイル**: `src/rust/.gitignore`

```
/target/
Cargo.lock
```

## 将来の拡張（オプション）

1. **オクターブ制御** (`o4`, `>`, `<`)
2. **音長制御** (`l4`, `c4`)
3. **テンポ制御** (`t120`)
4. **和音サポート**
5. **WAVファイル出力**

## 技術的な注意点

### cpalの選択理由

- クロスプラットフォーム対応
- 低レイテンシ
- Rustエコシステムで標準的

### エラーハンドリング

Result型を使用した安全なエラーハンドリング

### メモリ管理

ベクトルを使った効率的なバッファ管理

## 推定工数

- **Phase 1**: 30分（セットアップ）
- **Phase 2**: 2時間（パーサー実装）
- **Phase 3**: 2時間（オーディオ実装）
- **Phase 4**: 1時間（統合とテスト）
- **Phase 5**: 30分（ドキュメント）

**合計**: 約6時間

## 成功基準

- ✅ `cargo run -- "cde"` でドレミが再生される
- ✅ クロスプラットフォームで動作する
- ✅ エラーハンドリングが適切に実装されている
- ✅ cargoの標準的なプロジェクト構造に従っている
