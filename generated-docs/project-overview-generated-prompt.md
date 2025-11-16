Last updated: 2025-11-17


# プロジェクト概要生成プロンプト（来訪者向け）

## 生成するもの：
- projectを3行で要約する
- プロジェクトで使用されている技術スタックをカテゴリ別に整理して説明する
- プロジェクト全体のファイル階層ツリー（ディレクトリ構造を図解）
- プロジェクト全体のファイルそれぞれの説明
- プロジェクト全体の関数それぞれの説明
- プロジェクト全体の関数の呼び出し階層ツリー

## 生成しないもの：
- Issues情報（開発者向け情報のため）
- 次の一手候補（開発者向け情報のため）
- ハルシネーションしそうなもの（例、存在しない機能や計画を勝手に妄想する等）

## 出力フォーマット：
以下のMarkdown形式で出力してください：

```markdown
# Project Overview

## プロジェクト概要
[以下の形式で3行でプロジェクトを要約]
- [1行目の説明]
- [2行目の説明]
- [3行目の説明]

## 技術スタック
[使用している技術をカテゴリ別に整理して説明]
- フロントエンド: [フロントエンド技術とその説明]
- 音楽・オーディオ: [音楽・オーディオ関連技術とその説明]
- 開発ツール: [開発支援ツールとその説明]
- テスト: [テスト関連技術とその説明]
- ビルドツール: [ビルド・パース関連技術とその説明]
- 言語機能: [言語仕様・機能とその説明]
- 自動化・CI/CD: [自動化・継続的統合関連技術とその説明]
- 開発標準: [コード品質・統一ルール関連技術とその説明]

## ファイル階層ツリー
```
[プロジェクトのディレクトリ構造をツリー形式で表現]
```

## ファイル詳細説明
[各ファイルの役割と機能を詳細に説明]

## 関数詳細説明
[各関数の役割、引数、戻り値、機能を詳細に説明]

## 関数呼び出し階層ツリー
```
[関数間の呼び出し関係をツリー形式で表現]
```
```


以下のプロジェクト情報を参考にして要約を生成してください：

## プロジェクト情報
名前: 
説明: # cat-play-mml

🎵 Music Macro Language (MML) Parser and Player

<p align="left">
  <a href="README.ja.md"><img src="https://img.shields.io/badge/🇯🇵-Japanese-red.svg" alt="Japanese"></a>
  <a href="README.md"><img src="https://img.shields.io/badge/🇺🇸-English-blue.svg" alt="English"></a>
</p>

## Quick Links
| 項目 | リンク |
|------|--------|
| 📊 開発状況 | [generated-docs/development-status](generated-docs/development-status.md) |

## 概要

`cat-play-mml` は、Music Macro Language (MML) によって音楽を再生するCLIツールです。文字列`cde`を入力すれば、音楽`ドレミ`を再生します。Windows用です。

## クイックスタートガイド

### 環境構築
- Windowsに、`Rust` と `Zig` をインストールしてください

### インストール
```
cargo install --git https://github.com/cat2151/cat-play-mml.git --branch main --bin cat-play-mml
```

これだけ！GitHubからあなたのWindowsへインストールされます

### 演奏
```
cat-play-mml cde
```

ドレミが鳴ります

## 今後、関連プロジェクトのissuesに書く予定の候補

- [mmlabc-to-smf-rust](https://github.com/cat2151/mmlabc-to-smf-rust/issues)：
  - MML `;` を実装する。`c;e;g` はドミソの和音となる。それぞれ今後別の音色を割り当てられるよう、ch1ドミソでなく、ch1ド,ch2ミ,ch3ソ（1base記述）である。
  - MIDI プログラムチェンジ 0（0base記述）を、SMFの各ch先頭に出力する。note出力のないchには出力しない。

- [smf-to-ym2151log-rust](https://github.com/cat2151/smf-to-ym2151log-rust/issues)
  - MML `;` 実装にともない、複数chに対応する。ch内の和音については別途検討とし今は未定義動作のままでよい。
  - MIDI プログラムチェンジ 0（0base記述）は、アコースティックグランドピアノ 風の音色にする。より具体的には、sine waveでなければよい程度。

- [ym2151-log-play-server](https://github.com/cat2151/ym2151-log-play-server/issues)

### 主な特徴

- **シンプル、すぐ鳴らせる**: 引数に"cde"を渡すだけでドレミを再生
- **低レイテンシ**: リアルタイム音楽再生
- **バックグラウンド演奏**: サーバーモードで演奏しながら他の操作が可能

### 使い方

#### 基本的な使い方（自動サーバー起動）

```
cat-play-mml cde
```

初回実行時、自動的にサーバーが起動し、バックグラウンドで演奏が始まります。コマンドはすぐに終了し、次のコマンドを入力できます。

2回目以降の実行では、既に起動しているサーバーに演奏を送信します：

```
cat-play-mml efg
```

#### サーバーの制御

演奏を停止：

```
cat-play-mml --stop
```

サーバーをシャットダウン：

```
cat-play-mml --shutdown
```

#### 手動サーバー起動（上級者向け）

JSONファイルを指定してサーバーを起動：

```
cat-play-mml --server output.json
```

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

### 使用するライブラリ

- Nuked-OPM
- **Rust**: cpal

## プロジェクトのゴール

### 小ゴール
- [x] Windows Rust版 exeとして、cdeをコマンドライン引数に指定したら、リアルタイム演奏でドレミが鳴ること

### 次のゴール
- mmlabcの文法
  - 優先
    - `;`
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

### cat-edit-ym2151-tone
- output : キーを押すごとに演奏する
  - 内部的にはplay mmlする
  - まず極端にシンプルな実装で検証する
- 用途 : シンセサイザーの醍醐味の一つである音色作り体験を提供する用、の検証用
  - よりよい成功体験の提供のためにはいろいろな仕様が必要であるが、それは最初の実装からは外す（でないとキリがなく迷子になる）
    - 小さく始める。一歩ずつやる。それが全体最適
- 独立したTUIアプリとして切り分けたリポジトリにする。小さく始める

### cat-edit-mmlabc
- output : キーを押すごとに演奏する
- 用途 : cを押したらドが鳴る体験を提供する
- 独立したTUIアプリとして切り分けたリポジトリにする。小さく始める

## 補足

### 開発者向けの、ビルドとinstallと実行の方法

```powershell
# ビルド & 実行 ※cloneしたディレクトリにて
cargo run --release cegb

# インストール ※cloneしたディレクトリにて
cargo install --path .

# 実行 ※インストールすれば、このようにどのディレクトリからでも実行できます
cat-play-mml cegb
```

## ライセンス

このプロジェクトは [MIT License](LICENSE) の下で公開されています。

※英語版README.mdは、README.ja.mdを元にGeminiの翻訳でGitHub Actionsにより自動生成しています


依存関係:
{}

## ファイル階層ツリー
📄 .editorconfig
📄 .gitignore
📁 .vscode/
  📊 extensions.json
  📊 settings.json
📄 Cargo.lock
📄 Cargo.toml
📖 IMPLEMENTATION_PLAN.md
📖 IMPLEMENTATION_SUMMARY.md
📄 LICENSE
📖 PHASE2_INVESTIGATION_REPORT.md
📖 PHASE2_STATUS.md
📖 QUICK_REFERENCE.md
📖 README.ja.md
📖 README.md
📖 TESTING_GUIDE.md
📄 _config.yml
📁 generated-docs/
📁 issue-notes/
  📖 11.md
  📖 13.md
  📖 15.md
  📖 17.md
  📖 19.md
  📖 21.md
  📖 23.md
  📖 25.md
  📖 27.md
  📖 29.md
📁 src/
  📄 app.rs
  📄 cli.rs
  📄 client_manager.rs
  📄 converter.rs
  📄 input.rs
  📄 main.rs
  📄 process_manager.rs

## ファイル詳細分析


## 関数呼び出し階層
関数呼び出し階層を分析できませんでした

## プロジェクト構造（ファイル一覧）
.vscode/extensions.json
.vscode/settings.json
IMPLEMENTATION_PLAN.md
IMPLEMENTATION_SUMMARY.md
PHASE2_INVESTIGATION_REPORT.md
PHASE2_STATUS.md
QUICK_REFERENCE.md
README.ja.md
README.md
TESTING_GUIDE.md
issue-notes/11.md
issue-notes/13.md
issue-notes/15.md
issue-notes/17.md
issue-notes/19.md
issue-notes/21.md
issue-notes/23.md
issue-notes/25.md
issue-notes/27.md
issue-notes/29.md

上記の情報を基に、プロンプトで指定された形式でプロジェクト概要を生成してください。
特に以下の点を重視してください：
- 技術スタックは各カテゴリごとに整理して説明
- ファイル階層ツリーは提供された構造をそのまま使用
- ファイルの説明は各ファイルの実際の内容と機能に基づく
- 関数の説明は実際に検出された関数の役割に基づく
- 関数呼び出し階層は実際の呼び出し関係に基づく


---
Generated at: 2025-11-17 07:02:56 JST
