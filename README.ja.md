# cat-play-mml

🎵 Music Macro Language (MML) Parser and Player

<p align="left">
  <a href="README.ja.md"><img src="https://img.shields.io/badge/🇯🇵-Japanese-red.svg" alt="Japanese"></a>
  <a href="README.md"><img src="https://img.shields.io/badge/🇺🇸-English-blue.svg" alt="English"></a>
</p>

## Quick Links
| 項目 | リンク |
|------|--------|
| 📊 開発状況 | [generated-docs/development-status](generated-docs/development-status.md) |

## 状況

Windows Rust版 exe の基本実装が完了しました。コマンドライン引数でMMLを指定して音楽を再生できます。

## クイックスタートガイド

```powershell
cargo run --release cegb
```

## 概要

`cat-play-mml` は、Music Macro Language (MML) を解析して音楽を再生するマルチ言語対応のプロジェクトです。tree-sitterを使用してMMLをASTにパースし、中間表現に変換して音楽を再生します。

### 主な特徴

- **MMLパーサー**: tree-sitterベースのMML文法定義
- **低レイテンシ**: リアルタイム音楽再生
- **シンプルなコマンドライン引数**: 引数に"cde"を渡すだけでドレミを再生

## MML (Music Macro Language) とは

MMLは、テキストで音楽を記述する言語です。以下のような記法を使用します：

- `c`, `d`, `e`, `f`, `g`, `a`, `b`: 音階（ド、レ、ミ、ファ、ソ、ラ、シ）

### 今後実装予定
- `o4`: オクターブ設定（4番目のオクターブ）
- `l4`: 音長設定（4分音符）
- `t120`: テンポ設定（BPM 120）
- `<`, `>`: オクターブの上げ下げ
- `+`, `-`: 半音上げ下げ
- `r`: 休符

## 技術詳細

### アーキテクチャ

1. **パーサー**: tree-sitterを使用してMMLテキストをASTに変換
2. **中間表現**: ASTを音楽データ構造に変換
3. **オーディオ生成**: 中間表現から音声波形を生成
4. **再生**: オーディオライブラリを使用して音声を出力

### 開発環境

- Windows
- Rust
- Zig cc（mingwとmsys2は禁止）
- agentのTDD用にLinux runner（agentがTDDできさえすればよい） + ALSA SDKと設定（ヘッドレス環境でもTDD可能にするため）

### 使用するオーディオライブラリ

- **Rust**: cpal

## プロジェクトのゴール

- Windows Rust版 exeとして、cdeをコマンドライン引数に指定したら、リアルタイム演奏でドレミが鳴ること

## 実装される見込みのもの

- 中間表現のファイル出力（StandardMIDIファイルを含む）

## スコープ外

- 複雑なMML
- リアルタイムMIDIメッセージ出力
- エフェクトなど（LPF、オーバードライブ/ディストーション、ディレイなど）
- GUIエディタ

## 関連プロジェクト

### cat-play-chord 検討中

[cat-play-chord](https://github.com/cat2151/cat-play-chord) は、chord2mmlを利用してコード表記からMMLを生成して演奏するプロジェクトです（検討中）。

### mml to smf の今後の見通し
- 状況
  - cdeだけ実装した
    - 狙い、最初は最低限の実装に絞り込むことで、リアルタイム演奏でドレミが鳴るまでの問題解決をスムーズにする
  - SMFを使っている
    - 狙い、SMFを使うことで確認と開発がしやすくなり、開発が頓挫するリスクを下げられる
- MML方言はmmlabcを使う想定、ノウハウがあり、formatが明確
- TDD agent で進める想定、ハルシネーションが出たら検討する

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

### Nuked-OPM friendly JSON player
- 実装済（別リポジトリのlog player）
- 用途は、開発をしやすくする用
  - デバッグしやすく、開発が頓挫するリスクを下げられる

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

※英語版README.mdは、README.ja.mdを元にGeminiの翻訳でGitHub Actionsにより自動生成しています
