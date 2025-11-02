Last updated: 2025-11-02

# Project Overview

## プロジェクト概要
- Music Macro Language (MML) を解析し、音楽を再生する多言語対応のプロジェクトです。
- tree-sitterを用いてMMLを構文解析し、Python、Rust、Go、TypeScriptなど複数の言語で実装されています。
- 低レイテンシでのリアルタイム音楽再生と、シンプルなAPIによる簡単な利用が特徴です。

## 技術スタック
- フロントエンド:
    - **Tone.js**: Web Audio APIの上に構築された、ブラウザでの音声生成と操作を容易にするJavaScriptフレームワーク。
    - **Web Audio API**: ウェブブラウザで高度な音声処理を行うためのAPI（Tone.jsを介して利用）。
- 音楽・オーディオ:
    - **MML (Music Macro Language)**: 音楽をテキストで記述するための簡易的なプログラミング言語。プロジェクトの入力形式として使用されます。
- 開発ツール:
    - **Node.js runtime**: JavaScriptを実行するための環境で、TypeScript版のビルドや開発に利用されます。
- テスト:
    - (情報なし)
- ビルドツール:
    - (情報なし)
- 言語機能:
    - (情報なし)
- 自動化・CI/CD:
    - **GitHub Actions**: プロジェクトの継続的インテグレーションとデリバリーを自動化するサービス。以下のワークフローが含まれます。
        - **プロジェクト要約自動生成**: プロジェクトの概要を自動で生成します。
        - **Issue自動管理**: GitHubのIssue（課題）を自動で管理します。
        - **README多言語翻訳**: READMEファイルを複数の言語に自動翻訳します。
        - **i18n automation**: 国際化（i18n）関連の自動化処理を行います。
- 開発標準:
    - **EditorConfig**: 異なるエディタやIDE間でコードのスタイル（インデント、改行コードなど）を統一するための設定ファイルです。

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
-   **`.editorconfig`**: 異なるエディタやIDE間でコードスタイルを統一するための設定ファイルです。
-   **`.gitignore`**: Gitがバージョン管理の対象としないファイルやディレクトリを指定するファイルです。
-   **`.vscode/extensions.json`**: Visual Studio Codeのワークスペースで推奨される拡張機能リストです。
-   **`.vscode/settings.json`**: Visual Studio Codeのワークスペース固有の設定ファイルです。
-   **`LICENSE`**: プロジェクトのライセンス情報が記述されています。
-   **`README.ja.md`**: プロジェクトの概要、目的、使い方などを日本語で説明するドキュメントです。
-   **`README.md`**: プロジェクトの概要、目的、使い方などを英語で説明するドキュメントです。
-   **`docs/.gitkeep`**: 空の`docs`ディレクトリをGitで管理するためのプレースホルダーファイルです。
-   **`docs/AGENT_INSTRUCTIONS.md`**: プロジェクトの自動化エージェントに対する指示が記述されたドキュメントです。
-   **`docs/PROJECT_SETUP_COMPLETION.md`**: プロジェクトのセットアップ完了に関する情報が記述されたドキュメントです。
-   **`docs/QUICKSTART.md`**: プロジェクトを素早く使い始めるためのガイドドキュメントです。
-   **`generated-docs/`**: 自動生成されるドキュメントが格納されるディレクトリです。
-   **`grammar/.gitkeep`**: 空の`grammar`ディレクトリをGitで管理するためのプレースホルダーファイルです。tree-sitterのMML文法定義などが格納される可能性があります。
-   **`pytest.ini`**: Pythonのテストフレームワークであるpytestの設定ファイルです。
-   **`requirements.txt`**: Pythonプロジェクトの依存ライブラリをリストアップしたファイルです。
-   **`ruff.toml`**: Pythonの高速なリンター・フォーマッターであるRuffの設定ファイルです。
-   **`src/IMPLEMENTATION_PLAN_SUMMARY.md`**: 全体の実装計画の概要が記述されたドキュメントです。
-   **`src/go/IMPLEMENTATION_PLAN.md`**: Go言語版の実装計画が記述されたドキュメントです。
-   **`src/go/README.md`**: Go言語版MMLプレイヤーに関する説明ドキュメントです。
-   **`src/go/go.mod`**: Goモジュールの定義ファイルで、依存関係などが管理されます。
-   **`src/go/main.go`**: Go言語版MMLプレイヤーのメインエントリポイントとなるソースコードです。MMLの解析と再生ロジックが含まれます。
-   **`src/python/IMPLEMENTATION_PLAN.md`**: Python言語版の実装計画が記述されたドキュメントです。
-   **`src/python/README.md`**: Python言語版MMLプレイヤーに関する説明ドキュメントです。
-   **`src/python/audio_player.py`**: Python版MMLプレイヤーで音声再生を担当するロジックが含まれるファイルです。
-   **`src/python/mml_parser.py`**: Python版MMLプレイヤーでMML文字列の解析を担当するロジックが含まれるファイルです。
-   **`src/python/play_mml.py`**: Python版MMLプレイヤーのメインスクリプトで、MML文字列を受け取り、解析して音楽を再生する処理を行います。
-   **`src/rust/.gitignore`**: Rust版MMLプレイヤー固有のGit無視ファイルです。
-   **`src/rust/Cargo.toml`**: Rustプロジェクトのビルド設定や依存関係を定義するファイルです。
-   **`src/rust/IMPLEMENTATION_PLAN.md`**: Rust言語版の実装計画が記述されたドキュメントです。
-   **`src/rust/README.md`**: Rust言語版MMLプレイヤーに関する説明ドキュメントです。
-   **`src/rust/src/main.rs`**: Rust言語版MMLプレイヤーのメインエントリポイントとなるソースコードです。
-   **`src/typescript/browser/.gitignore`**: TypeScriptブラウザ版固有のGit無視ファイルです。
-   **`src/typescript/browser/IMPLEMENTATION_PLAN.md`**: TypeScriptブラウザ版の実装計画が記述されたドキュメントです。
-   **`src/typescript/browser/README.md`**: TypeScriptブラウザ版MMLプレイヤーに関する説明ドキュメントです。
-   **`src/typescript/browser/index.html`**: ブラウザでMMLプレイヤーを実行するためのHTMLファイルです。
-   **`src/typescript/browser/package.json`**: TypeScriptブラウザ版プロジェクトの依存関係やスクリプトを定義するファイルです。
-   **`src/typescript/browser/src/main.ts`**: TypeScriptブラウザ版MMLプレイヤーのメインエントリポイントとなるソースコードです。
-   **`src/typescript/browser/tsconfig.json`**: TypeScriptコンパイラの設定ファイルです。
-   **`src/typescript/deno/IMPLEMENTATION_PLAN.md`**: TypeScript Deno版の実装計画が記述されたドキュメントです。
-   **`src/typescript/deno/README.md`**: TypeScript Deno版MMLプレイヤーに関する説明ドキュメントです。
-   **`src/typescript/deno/deno.json`**: Denoプロジェクトの設定ファイルです。
-   **`src/typescript/deno/main.ts`**: TypeScript Deno版MMLプレイヤーのメインエントリポイントとなるソースコードです。
-   **`tests/.gitkeep`**: 空の`tests`ディレクトリをGitで管理するためのプレースホルダーファイルです。

## 関数詳細説明
-   **`main` (src/typescript/deno/main.ts)**:
    -   **役割**: Deno環境でMMLプレイヤーを実行する際のエントリポイントです。
    -   **引数**: コマンドライン引数を受け取る可能性がありますが、現在の情報では明示されていません。
    -   **戻り値**: なし。
    -   **機能**: MML文字列の解析、音楽再生の開始といった主要な処理フローを制御すると推測されます。

## 関数呼び出し階層ツリー
```
- if (src/typescript/deno/main.ts)
  - main (src/typescript/deno/main.ts)

---
Generated at: 2025-11-02 13:52:25 JST
