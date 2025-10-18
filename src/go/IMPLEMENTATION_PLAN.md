# Go実装計画書

## 概要

Go版のMML（Music Macro Language）パーサーとプレーヤーを実装します。

## 目標

コマンドライン引数として"cde"を受け取り、ドレミを音声で再生するシンプルなアプリケーションを作成します。

## 使用ライブラリ

- **github.com/hajimehoshi/oto**: シンプルな低レイテンシオーディオライブラリ
- **標準ライブラリ**: パーサー実装用

## プロジェクト構造

```
src/go/
├── go.mod
├── go.sum
├── main.go              # メインエントリーポイント
├── parser/
│   └── parser.go        # MMLパーサー
├── audio/
│   └── player.go        # オーディオ生成・再生
├── README.md
└── IMPLEMENTATION_PLAN.md (このファイル)
```

## データ構造

```go
type Note struct {
    Pitch     rune    // 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'r'
    Octave    int     // 4 (default)
    Duration  float64 // 0.5 (秒)
    Frequency float64 // 計算された周波数 (Hz)
}
```

## 実装ステップ

### Phase 1: プロジェクトセットアップ

1. **go.modの作成**

```bash
cd src/go
go mod init github.com/cat2151/cat-play-mml/go
go get github.com/hajimehoshi/oto/v2
```

2. **ディレクトリ構造の作成**
   ```bash
   mkdir -p src/go/parser src/go/audio
   ```

### Phase 2: MMLパーサーの実装

**ファイル**: `src/go/parser/parser.go`

```go
package parser

import (
    "math"
    "strings"
)

type Note struct {
    Pitch     rune
    Octave    int
    Duration  float64
    Frequency float64
}

func NewNote(pitch rune, octave int, duration float64) Note {
    return Note{
        Pitch:     pitch,
        Octave:    octave,
        Duration:  duration,
        Frequency: calculateFrequency(pitch, octave),
    }
}

func calculateFrequency(pitch rune, octave int) float64 {
    // A4 = 440Hz を基準に計算
    noteOffsets := map[rune]int{
        'c': -9, 'd': -7, 'e': -5, 'f': -4,
        'g': -2, 'a': 0, 'b': 2, 'r': 0,
    }

    offset, ok := noteOffsets[pitch]
    if !ok || pitch == 'r' {
        return 0.0 // 休符または無効な音符
    }

    semitones := (octave-4)*12 + offset
    return 440.0 * math.Pow(2.0, float64(semitones)/12.0)
}

func ParseMML(mml string) []Note {
    notes := make([]Note, 0)
    currentOctave := 4
    currentDuration := 0.5

    for _, ch := range strings.ToLower(mml) {
        switch ch {
        case 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'r':
            note := NewNote(ch, currentOctave, currentDuration)
            notes = append(notes, note)
        default:
            // スペースなどは無視
        }
    }

    return notes
}
```

### Phase 3: オーディオ生成の実装

**ファイル**: `src/go/audio/player.go`

```go
package audio

import (
    "bytes"
    "encoding/binary"
    "math"
    "time"

    "github.com/cat2151/cat-play-mml/go/parser"
    "github.com/hajimehoshi/oto/v2"
)

const (
    SampleRate = 48000
    BitDepth   = 2 // 16-bit
)

type Player struct {
    context *oto.Context
}

func NewPlayer() (*Player, error) {
    ctx, ready, err := oto.NewContext(SampleRate, 1, BitDepth)
    if err != nil {
        return nil, err
    }
    <-ready

    return &Player{context: ctx}, nil
}

func (p *Player) PlayNotes(notes []parser.Note) error {
    var buf bytes.Buffer

    for _, note := range notes {
        samples := generateSineWave(note.Frequency, note.Duration)
        if err := binary.Write(&buf, binary.LittleEndian, samples); err != nil {
            return err
        }
    }

    player := p.context.NewPlayer(bytes.NewReader(buf.Bytes()))
    player.Play()

    // 再生が終わるまで待つ
    duration := time.Duration(0)
    for _, note := range notes {
        duration += time.Duration(note.Duration * float64(time.Second))
    }
    time.Sleep(duration + 100*time.Millisecond)

    return nil
}

func (p *Player) Close() error {
    return p.context.Close()
}

func generateSineWave(frequency float64, duration float64) []int16 {
    numSamples := int(SampleRate * duration)
    samples := make([]int16, numSamples)

    if frequency == 0.0 {
        // 休符 - 無音
        return samples
    }

    attackSamples := int(SampleRate * 0.01)
    releaseSamples := int(SampleRate * 0.05)

    for i := 0; i < numSamples; i++ {
        t := float64(i) / SampleRate
        value := math.Sin(2 * math.Pi * frequency * t)

        // エンベロープ（アタック・リリース）
        envelope := 1.0
        if i < attackSamples {
            envelope = float64(i) / float64(attackSamples)
        } else if i > numSamples-releaseSamples {
            envelope = float64(numSamples-i) / float64(releaseSamples)
        }

        value *= envelope * 0.3 // 音量を30%に

        // 16-bit PCMに変換
        samples[i] = int16(value * 32767)
    }

    return samples
}
```

### Phase 4: メインプログラムの実装

**ファイル**: `src/go/main.go`

```go
package main

import (
    "fmt"
    "os"

    "github.com/cat2151/cat-play-mml/go/audio"
    "github.com/cat2151/cat-play-mml/go/parser"
)

func main() {
    if len(os.Args) < 2 {
        fmt.Fprintf(os.Stderr, "使用法: %s <MML文字列>\n", os.Args[0])
        fmt.Fprintf(os.Stderr, "例: %s \"cde\"\n", os.Args[0])
        os.Exit(1)
    }

    mmlString := os.Args[1]
    fmt.Printf("MMLを再生します: %s\n", mmlString)

    notes := parser.ParseMML(mmlString)
    fmt.Printf("パース成功: %d個の音符\n", len(notes))

    player, err := audio.NewPlayer()
    if err != nil {
        fmt.Fprintf(os.Stderr, "オーディオプレーヤーの初期化エラー: %v\n", err)
        os.Exit(1)
    }
    defer player.Close()

    if err := player.PlayNotes(notes); err != nil {
        fmt.Fprintf(os.Stderr, "再生エラー: %v\n", err)
        os.Exit(1)
    }

    fmt.Println("再生完了")
}
```

### Phase 5: READMEの作成

**ファイル**: `src/go/README.md`

```markdown
# Go MML Player

Music Macro Language (MML) parser and player implemented in Go.

## Requirements

- Go 1.19+

## Build

\`\`\`bash
cd src/go
go build -o mml-player
\`\`\`

## Usage

\`\`\`bash
go run main.go "cde"
\`\`\`

Or with the compiled binary:

\`\`\`bash
./mml-player "cde"
\`\`\`

## Features

- Simple MML parser supporting basic notes (c, d, e, f, g, a, b)
- Rest notes (r)
- Low-latency audio playback using oto
- Cross-platform support (Windows, macOS, Linux)

## Example

\`\`\`bash
go run main.go "cdefgab"
\`\`\`

## Dependencies

- github.com/hajimehoshi/oto/v2 - Low-latency audio library
```

## ビルドとテスト

```bash
# 依存関係の取得
cd src/go
go mod tidy

# ビルド
go build -o mml-player

# 実行
go run main.go "cde"

# または
./mml-player "cde"

# テスト
go test ./...
```

## 将来の拡張（オプション）

1. **オクターブ制御** (`o4`, `>`, `<`)
2. **音長制御** (`l4`, `c4`)
3. **テンポ制御** (`t120`)
4. **並行処理**: goroutineを使った効率的な音声生成
5. **バッファリング**: チャネルを使ったストリーミング再生

## 技術的な注意点

### otoの選択理由

- シンプルで使いやすいAPI
- 低レイテンシ
- クロスプラットフォーム対応
- hajimehoshi氏（Ebiten作者）による信頼性の高いライブラリ

### エラーハンドリング

Goの慣用的なエラーハンドリングを使用

### メモリ管理

バイトバッファを使った効率的なデータ管理

## 推定工数

- **Phase 1**: 30分（セットアップ）
- **Phase 2**: 2時間（パーサー実装）
- **Phase 3**: 2時間（オーディオ実装）
- **Phase 4**: 1時間（統合とテスト）
- **Phase 5**: 30分（ドキュメント）

**合計**: 約6時間

## 成功基準

- ✅ `go run main.go "cde"` でドレミが再生される
- ✅ クロスプラットフォームで動作する
- ✅ エラーハンドリングが適切に実装されている
- ✅ Goの標準的なプロジェクト構造に従っている
- ✅ `go fmt` と `go vet` が通る
