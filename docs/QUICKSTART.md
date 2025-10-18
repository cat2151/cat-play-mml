# クイックスタートガイド

cat-play-mml プロジェクトをすぐに試すためのガイドです。

## Python版（実装済み）

### 1. 必要なもの

- Python 3.8以上
- pip

### 2. インストール

```bash
# リポジトリのクローン
git clone https://github.com/cat2151/cat-play-mml.git
cd cat-play-mml

# 依存関係のインストール
pip install -r requirements.txt
```

### 3. 実行

```bash
# ドレミを再生
python src/python/play_mml.py "cde"

# 音階全体を再生
python src/python/play_mml.py "cdefgab"

# 休符を含む
python src/python/play_mml.py "crcrcr"
```

### 4. 期待される出力

```
MMLを再生します: cde
パース成功: 3個の音符
再生完了
```

音声が再生されます（オーディオデバイスが必要）。

## その他の言語（実装予定）

### Rust

```bash
cd src/rust
cargo run -- "cde"
```

詳細: [src/rust/IMPLEMENTATION_PLAN.md](../src/rust/IMPLEMENTATION_PLAN.md)

### Go

```bash
cd src/go
go run main.go "cde"
```

詳細: [src/go/IMPLEMENTATION_PLAN.md](../src/go/IMPLEMENTATION_PLAN.md)

### TypeScript (ブラウザ版)

```bash
cd src/typescript/browser
npm install
npm run dev
```

ブラウザで http://localhost:5173 を開く

詳細: [src/typescript/browser/IMPLEMENTATION_PLAN.md](../src/typescript/browser/IMPLEMENTATION_PLAN.md)

### TypeScript (Deno版)

```bash
cd src/typescript/deno
deno run --allow-all main.ts "cde"
```

詳細: [src/typescript/deno/IMPLEMENTATION_PLAN.md](../src/typescript/deno/IMPLEMENTATION_PLAN.md)

## トラブルシューティング

### Python: sounddeviceがインストールできない

**macOS**:
```bash
brew install portaudio
pip install sounddevice
```

**Ubuntu/Debian**:
```bash
sudo apt-get install libportaudio2
pip install sounddevice
```

**Windows**:
```bash
pip install sounddevice
```

### Rust: ALSAエラー

**Ubuntu/Debian**:
```bash
sudo apt-get install libasound2-dev
```

### オーディオが再生されない

1. オーディオデバイスが接続されているか確認
2. 音量が0でないか確認
3. 他のアプリケーションでオーディオが動作するか確認

## 開発

### コードフォーマット（Python）

```bash
ruff format src/ tests/
```

### リントチェック（Python）

```bash
ruff check src/ tests/
```

### テスト実行（Python）

```bash
pytest
```

## 詳細情報

- [README.md](../README.md): プロジェクト概要
- [IMPLEMENTATION_PLAN_SUMMARY.md](../src/IMPLEMENTATION_PLAN_SUMMARY.md): 実装計画サマリー
- 各言語の実装計画: `src/<言語>/IMPLEMENTATION_PLAN.md`
