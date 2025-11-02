Last updated: 2025-11-03

# Project Overview

## プロジェクト概要
- `cat-play-mml`は、Music Macro Language (MML) を解析し音楽を再生するマルチ言語対応プロジェクトです。
- tree-sitterを用いてMMLをASTにパースし、Python、Rust、Go、TypeScriptで実装されています。
- リアルタイムの低レイテンシ再生とシンプルなAPIを提供し、手軽にMML音楽を楽しめます。

## 技術スタック
- フロントエンド: Tone.js (Web Audio APIを介した音声ライブラリ)、Web Audio API (ブラウザネイティブの音声処理技術)
- 音楽・オーディオ: MML (Music Macro Language、音楽記法パーサーの対象言語)、Tone.js、Web Audio API
- 開発ツール: Node.js runtime (JavaScript実行環境)
- テスト: pytest (Python向けテストフレームワーク、`pytest.ini`より推測)
- ビルドツール: (現在、特定のビルドツールの情報はありません)
- 言語機能: (現在、特定の言語機能に焦点を当てた情報はありません)
- 自動化・CI/CD: GitHub Actions (CI/CD自動化、プロジェクト要約自動生成、Issue自動管理、README多言語翻訳、i18n automation)
- 開発標準: EditorConfig (コード統一ルール)、Ruff (Python向けLinter/Formatter、`ruff.toml`より推測)

## ファイル階層ツリー
```
📄 .editorconfig
📄 .gitignore
📁 .vscode/
  📊 extensions.json
  📊 settings.json
📄 LICENSE
📖 README.ja.md
📖 README.md
📁 docs/
  📄 .gitkeep
  📖 AGENT_INSTRUCTIONS.md
  📖 PROJECT_SETUP_COMPLETION.md
  📖 QUICKSTART.md
📁 generated-docs/
📁 grammar/
  📄 .gitkeep
📄 pytest.ini
📄 requirements.txt
📄 ruff.toml
📁 src/
  📖 IMPLEMENTATION_PLAN_SUMMARY.md
  📁 go/
    📖 IMPLEMENTATION_PLAN.md
    📖 README.md
    📄 go.mod
    📄 main.go
  📁 python/
    📖 IMPLEMENTATION_PLAN.md
    📖 README.md
    📄 audio_player.py
    📄 mml_parser.py
    📄 play_mml.py
  📁 rust/
    📄 .gitignore
    📄 Cargo.toml
    📖 IMPLEMENTATION_PLAN.md
    📖 README.md
    📁 src/
      📄 main.rs
  📁 typescript/
    📁 browser/
      📄 .gitignore
      📖 IMPLEMENTATION_PLAN.md
      📖 README.md
      🌐 index.html
      📊 package.json
      📁 src/
        📘 main.ts
      📊 tsconfig.json
    📁 deno/
      📖 IMPLEMENTATION_PLAN.md
      📖 README.md
      📊 deno.json
      📘 main.ts
📁 tests/
  📄 .gitkeep
```

## ファイル詳細説明
-   `.editorconfig`: エディタ設定ファイル。インデントスタイルや文字コードなど、プロジェクト全体のコードスタイルを統一するためのルールを定義します。
-   `.gitignore`: Gitによるバージョン管理から除外するファイルやディレクトリを指定し、不要なファイルがリポジトリに含まれるのを防ぎます。
-   `.vscode/extensions.json`: Visual Studio Codeユーザー向けに、プロジェクトでの開発を支援する推奨拡張機能をリストします。
-   `.vscode/settings.json`: Visual Studio Codeのワークスペース設定ファイル。プロジェクト固有のエディタ動作やリンター設定などを定義します。
-   `LICENSE`: プロジェクトのライセンス情報が記載されており、ソフトウェアの利用、配布、改変に関する条件を示します。
-   `README.ja.md`: プロジェクトの概要、目的、使用方法、セットアップ手順などを日本語で説明するドキュメントです。
-   `README.md`: プロジェクトのメインドキュメント。概要、機能、セットアップ方法などを英語で説明します。
-   `docs/.gitkeep`: `docs`ディレクトリが空でもGitで管理されるようにするためのプレースホルダーファイルです。
-   `docs/AGENT_INSTRUCTIONS.md`: 自動化エージェントがプロジェクト内で実行すべきタスクや指示に関するドキュメントです。
-   `docs/PROJECT_SETUP_COMPLETION.md`: プロジェクトのセットアップが完了した際の確認事項や後続手順を記したドキュメントです。
-   `docs/QUICKSTART.md`: プロジェクトを素早く開始し、基本的な機能を試すための簡潔なガイドを提供します。
-   `generated-docs/`: プロジェクトのビルドプロセスなどによって自動生成されるドキュメントを格納するディレクトリです。
-   `grammar/.gitkeep`: `grammar`ディレクトリが空でもGitで管理されるようにするためのプレースホルダーファイルです。MMLのtree-sitter文法定義がここに格納されることが想定されます。
-   `pytest.ini`: Pythonのテストフレームワークである`pytest`の設定ファイルです。テストの実行方法や検出に関する設定を定義します。
-   `requirements.txt`: Pythonプロジェクトが依存する外部ライブラリとそのバージョンを列挙したファイルです。
-   `ruff.toml`: Pythonの高速なLinterおよびFormatterであるRuffの設定ファイルです。コードの品質と一貫性を保つためのルールを定義します。
-   `src/IMPLEMENTATION_PLAN_SUMMARY.md`: プロジェクト全体の各言語実装における計画の概要をまとめたドキュメントです。
-   `src/go/IMPLEMENTATION_PLAN.md`: Go言語によるMMLパーサーおよびプレイヤーの実装に関する具体的な計画を記したドキュメントです。
-   `src/go/README.md`: Go言語実装の概要、使い方、特定の情報を提供するドキュメントです。
-   `src/go/go.mod`: Goモジュールの依存関係管理ファイル。Go言語プロジェクトの依存ライブラリを定義します。
-   `src/go/main.go`: Go言語版MMLプレイヤーのメインエントリーポイントファイル。MMLのパースと再生処理を制御します。
-   `src/python/IMPLEMENTATION_PLAN.md`: PythonによるMMLパーサーおよびプレイヤーの実装に関する具体的な計画を記したドキュメントです。
-   `src/python/README.md`: Python実装の概要、使い方、特定の情報を提供するドキュメントです。
-   `src/python/audio_player.py`: Python版MMLプレイヤーの音声再生ロジックを扱うファイルです。
-   `src/python/mml_parser.py`: Python版MMLプレイヤーのMML文字列を解析し、抽象構文木(AST)などを生成するロジックを実装しています。
-   `src/python/play_mml.py`: Python版MMLプレイヤーの実行スクリプト。コマンドライン引数としてMML文字列を受け取り、パース・再生を行います。
-   `src/rust/.gitignore`: Rust版プロジェクトに特化したGit無視設定ファイルです。
-   `src/rust/Cargo.toml`: Rustプロジェクトの依存関係、メタデータ、ビルド設定を管理するファイルです。
-   `src/rust/IMPLEMENTATION_PLAN.md`: RustによるMMLパーサーおよびプレイヤーの実装に関する具体的な計画を記したドキュメントです。
-   `src/rust/README.md`: Rust実装の概要、使い方、特定の情報を提供するドキュメントです。
-   `src/rust/src/main.rs`: Rust言語版MMLプレイヤーのメインエントリーポイントファイルです。
-   `src/typescript/browser/.gitignore`: TypeScriptのブラウザ版プロジェクトに特化したGit無視設定ファイルです。
-   `src/typescript/browser/IMPLEMENTATION_PLAN.md`: TypeScript (ブラウザ版) によるMMLパーサーおよびプレイヤーの実装に関する具体的な計画を記したドキュメントです。
-   `src/typescript/browser/README.md`: TypeScript (ブラウザ版) 実装の概要、使い方、特定の情報を提供するドキュメントです。
-   `src/typescript/browser/index.html`: ブラウザ版MMLプレイヤーのウェブページのエントリーポイント。HTML構造を定義し、スクリプトを読み込みます。
-   `src/typescript/browser/package.json`: TypeScript (ブラウザ版) プロジェクトの依存関係、スクリプト、メタデータを管理するファイルです。
-   `src/typescript/browser/src/main.ts`: TypeScript (ブラウザ版) のメインスクリプトファイル。ブラウザ上でのMMLのパースと再生ロジックを実装します。
-   `src/typescript/browser/tsconfig.json`: TypeScriptコンパイラの設定ファイル。コンパイルオプションなどを定義します。
-   `src/typescript/deno/IMPLEMENTATION_PLAN.md`: TypeScript (Deno版) によるMMLパーサーおよびプレイヤーの実装に関する具体的な計画を記したドキュメントです。
-   `src/typescript/deno/README.md`: TypeScript (Deno版) 実装の概要、使い方、特定の情報を提供するドキュメントです。
-   `src/typescript/deno/deno.json`: Denoプロジェクトの依存関係、スクリプト、タスクを定義するファイルです。
-   `src/typescript/deno/main.ts`: TypeScript (Deno版) のメインスクリプトファイル。Deno環境でMMLのパースと再生ロジックを実装し、コマンドライン引数を処理します。
-   `tests/.gitkeep`: `tests`ディレクトリが空でもGitで管理されるようにするためのプレースホルダーファイルです。

## 関数詳細説明
-   `main` (src/typescript/deno/main.ts)
    -   役割: Deno環境でMMLプレイヤーの主要な処理を実行するエントリーポイント関数です。
    -   引数: なし (Denoのグローバルな`Deno.args`を直接参照する可能性があります)。
    -   戻り値: なし。
    -   機能: コマンドライン引数を解析し、MML文字列の処理と音楽再生のロジックをオーケストレートすることが期待されます。

## 関数呼び出し階層ツリー
```
- if (src/typescript/deno/main.ts)
  - main (src/typescript/deno/main.ts)

---
Generated at: 2025-11-03 07:03:23 JST
