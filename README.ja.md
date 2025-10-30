# cat-play-mml

🎵 Music Macro Language (MML) Parser and Player

## 状況

まだagentに生成させた直後です

これから動作確認します

## 概要

`cat-play-mml` は、Music Macro Language (MML) を解析して音楽を再生するマルチ言語対応のプロジェクトです。tree-sitterを使用してMMLをASTにパースし、各言語で中間表現に変換して音楽を再生します。

### 主な特徴

- **MMLパーサー**: tree-sitterベースのMML文法定義
- **マルチ言語対応**: Python、Rust、Go、TypeScript（ブラウザ版・Deno版）での実装
- **低レイテンシ**: リアルタイム音楽再生
- **シンプルなAPI**: 引数に"cde"を渡すだけでドレミを再生

## デモ

MML文字列で音楽を表現します：

```bash
# ドレミを再生
python src/python/play_mml.py "cde"

# より複雑な例
python src/python/play_mml.py "t120 o4 cdefgab>c"
```

## MML (Music Macro Language) とは

MMLは、テキストで音楽を記述する言語です。以下のような記法を使用します：

- `c`, `d`, `e`, `f`, `g`, `a`, `b`: 音階（ド、レ、ミ、ファ、ソ、ラ、シ）
- `o4`: オクターブ設定（4番目のオクターブ）
- `l4`: 音長設定（4分音符）
- `t120`: テンポ設定（BPM 120）
- `>`, `<`: オクターブの上げ下げ
- `+`, `#`, `-`: 半音上げ下げ
- `r`: 休符

## インストール

### 必要な環境

- Python 3.8+
- pip

### Pythonライブラリのインストール

```bash
pip install -r requirements.txt
```

## 使用方法

### Python版

```bash
python src/python/play_mml.py "cde"
```

### Rust版（実装予定）

```bash
cd src/rust
cargo run -- "cde"
```

### Go版（実装予定）

```bash
cd src/go
go run main.go "cde"
```

### TypeScript版（ブラウザ版・実装予定）

```bash
cd src/typescript/browser
npm install
npm run dev
```

### TypeScript版（Deno版・実装予定）

```bash
cd src/typescript/deno
deno run --allow-all main.ts "cde"
```

## プロジェクト構造

```
cat-play-mml/
├── LICENSE                 # MITライセンス
├── README.md              # このファイル
├── pytest.ini            # pytest設定
├── ruff.toml              # コード品質ツール設定
├── requirements.txt       # Python依存関係
├── .editorconfig         # エディタ設定
├── grammar/              # tree-sitter文法定義
│   └── mml/             # MML文法
├── docs/                 # ドキュメント
├── tests/                # テスト
└── src/
    ├── python/           # Python実装
    │   ├── play_mml.py  # メインプログラム
    │   ├── mml_parser.py # MMLパーサー
    │   └── audio_player.py # オーディオ再生
    ├── rust/             # Rust実装（予定）
    │   ├── Cargo.toml
    │   └── src/
    ├── go/               # Go実装（予定）
    │   ├── go.mod
    │   └── main.go
    ├── typescript/       # TypeScript実装（予定）
    │   ├── browser/     # ブラウザ版
    │   └── deno/        # Deno版
    └── IMPLEMENTATION_PLAN_SUMMARY.md
```

## 技術詳細

### アーキテクチャ

1. **パーサー**: tree-sitterを使用してMMLテキストをASTに変換
2. **中間表現**: ASTを各言語の音楽データ構造に変換
3. **オーディオ生成**: 中間表現から音声波形を生成
4. **再生**: オーディオライブラリを使用して音声を出力

### 使用するオーディオライブラリ

- **Python**: sounddevice + numpy
- **Rust**: cpal (予定)
- **Go**: oto (予定)
- **TypeScript (Browser)**: Web Audio API (予定)
- **TypeScript (Deno)**: FFI経由でネイティブオーディオライブラリ (予定)

## 開発

### コード品質の維持

このプロジェクトでは [ruff](https://docs.astral.sh/ruff/) を使用してコード品質を維持しています。

```bash
# フォーマット
ruff format src/ tests/

# リントチェック
ruff check src/ tests/

# 自動修正可能な問題を修正
ruff check --fix src/ tests/
```

### テスト実行

```bash
# Python
pytest

# Rust
cd src/rust && cargo test

# Go
cd src/go && go test ./...
```

## 今後の予定

- [x] プロジェクト初期構成
- [ ] tree-sitter MML文法定義（オプション）
- [x] Python実装
- [ ] Rust実装 - [実装計画書](src/rust/IMPLEMENTATION_PLAN.md)
- [ ] Go実装 - [実装計画書](src/go/IMPLEMENTATION_PLAN.md)
- [ ] TypeScript（ブラウザ版）実装 - [実装計画書](src/typescript/browser/IMPLEMENTATION_PLAN.md)
- [ ] TypeScript（Deno版）実装 - [実装計画書](src/typescript/deno/IMPLEMENTATION_PLAN.md)

**実装計画の詳細**: [実装計画書サマリー](src/IMPLEMENTATION_PLAN_SUMMARY.md)をご覧ください。

## プロジェクトのゴール

- MMLパーサーの実装をLLM chatbotで生成できるか検証する
- 各言語でシンプルな実装が可能か検証する
- 各言語でLLM agentによる移植が可能か検証する

## スコープ外

- 複雑なMML拡張構文（マクロ、ループなど）
- MIDI出力
- エフェクト処理
- GUI エディタ

## 関連プロジェクト

### cat-play-chord 検討中

[cat-play-chord](https://github.com/cat2151/cat-play-chord) は、chord2mmlを利用してコード表記からMMLを生成して演奏するプロジェクトです（検討中）。

### mml to smf 検討中
- 以下のpassを想定
  - mml to CST by tree-sitter
  - CST to 中間表現
  - 中間表現 to 中間表現 ※n回
  - 中間表現 to SMF ※SMFをtextで表現したjson to SMF の想定
- MML方言はmmlabcを使う想定、ノウハウがあり、formatが明確
- SMFを使うことで確認と開発がしやすくなり、開発が頓挫するリスクを下げられる想定
- Linux Python TDD agent で進める想定、ハルシネーションが出たら検討する

### smf to Nuked-OPM friendly JSON 検討中
- 例
  - ディレイビブラート
    - tone settings toml の tone modify delay vibratoの値を元に、
      - OPMサウンドドライバ的に、
        - 1tickごとにソフトLFOのregister eventを生成
    - SMFとtomlを分離するのは、SMF側のMIDIインプリメンテーションをシンプルにする用
      - toml側で音色やOPMサウンドドライバ的処理の破壊的変更をしやすくなる、ETC原則
- 以下のpassを想定
  - SMF to 中間表現 ※SMFをtextで表現したjsonを想定
  - 中間表現 to 中間表現 ※n回 ※ディレイビブラートはここの想定
  - 中間表現 to Nuked-OPM friendly JSON
- Linux Python TDD agent で進める想定、ハルシネーションが出たら検討する

### Nuked-OPM friendly JSON player 検討中
- 用途は、開発をしやすくする用
- これならデバッグしやすく、開発が頓挫するリスクを下げられる想定
- 人力でやる想定、agentではTDD困難なためハルシネーションが多い想定

### リアルタイム FM tone editor 仮 検討中
- ここに書く目的
  - ラバーダッキング
- 用途
  - 検証用
- 優先順位
  - 開発の楽さと、少ない操作で音色のラフスケッチをする用途、を優先
- 操作
  - 右手 : mouse x,yでそれぞれにアサインした数値の増減
  - 左手 : WASD+QEでカーソル移動、SPACEで決定、ESCでキャンセル、ZでUNDO、ふわっとしている
  - x,yは、DecayRate, ModulatorTL, FB, MULをカーソル移動で切り替え
  - WASD+QE、UNDO、はふわっとしている
  - カーソル移動と決定、のかわりに、あるキーを押すと一発で効果が出て高速editできる、のほうがeditスピードは出るのでそれも試す、設定tomlファイルとか
- 音
  - 音色はpluckとロングトーンをカーソル移動で切り替え
  - OP接続アルゴリズムは2op並列、detuneは4つ別の値
- 表示
  - Windows TUI 80x24
  - 音色パラメータ表示はmdx note.x形式
  - 起動中は、最後の数値変更から1秒でクリップボードにmdx note.x形式で音色保存、出力はこれだけに絞る、仮
- すべての仕様は仮で、破壊的変更を頻繁に行う、開発が楽なことを優先
- これだけでもまだ仕様が多すぎる（小さく始めるには多すぎる）ので、もっと絞った仕様での仮の実装から小さく始める

## ライセンス

このプロジェクトは [MIT License](LICENSE) の下で公開されています。
