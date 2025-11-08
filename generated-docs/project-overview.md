Last updated: 2025-11-09

# Project Overview

## プロジェクト概要
- Music Macro Language (MML) 形式の音楽データを解析し、再生します。
- コマンドラインインターフェース (CLI) から手軽にMML音楽を再生できるツールです。
- Web Audio APIとTone.jsを活用し、高品質な音声再生を実現します。

## 技術スタック
- フロントエンド: このプロジェクトは主にCLIツールであり、直接的なWebフロントエンド技術は使用していません。Web Audio APIは音声処理のバックエンドとして機能します。
- 音楽・オーディオ:
    - Tone.js: Web Audio APIを抽象化し、より扱いやすくするJavaScript音声ライブラリです。
    - Web Audio API: ブラウザ上で高度な音声処理を行うためのAPIで、Tone.jsを通じて利用されています。
    - MML (Music Macro Language): 音楽をテキスト形式で記述するための記法で、このプロジェクトではMMLパーサーを使用して音楽データを解釈します。
- 開発ツール:
    - Node.js runtime: JavaScriptコードを実行するための環境です。
- テスト: プロジェクト情報からは具体的なテスト関連技術は確認できませんでした。
- ビルドツール: プロジェクト情報からは具体的なビルド・パース関連技術は確認できませんでした。
- 言語機能: プロジェクト情報からは具体的な言語仕様・機能は確認できませんでした。
- 自動化・CI/CD:
    - GitHub Actions: コードの変更やイベントに応じて自動的にタスクを実行するCI/CD（継続的インテグレーション/継続的デリバリー）プラットフォームです。以下のワークフローが設定されています。
        - プロジェクト要約自動生成: プロジェクトの概要を自動で生成します。
        - Issue自動管理: プロジェクトの課題（Issue）を自動で管理します。
        - README多言語翻訳: READMEファイルを複数の言語に自動で翻訳します。
        - i18n automation: 国際化（i18n）関連の自動化ワークフローです。
- 開発標準:
    - EditorConfig: 異なるIDEやエディタを使用する開発者間で、コードのインデントスタイルや文字コードなどのコーディングスタイルを統一するための設定ファイルです。

## ファイル階層ツリー
```
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
📁 src/
  📄 main.rs
```

## ファイル詳細説明
- **`.editorconfig`**: コーディングスタイル（インデント、改行コードなど）を定義し、複数の開発者やエディタ間で一貫性を保つための設定ファイルです。
- **`.gitignore`**: Gitのバージョン管理から除外するファイルやディレクトリを指定します。
- **`.vscode/extensions.json`**: VS Codeを推奨設定で利用するための、推奨拡張機能のリストです。
- **`.vscode/settings.json`**: VS Codeのワークスペース固有の設定を定義するファイルです。
- **`Cargo.lock`**: Rustプロジェクトのビルド時に使用された正確な依存ライブラリのバージョンを記録するファイルです。
- **`Cargo.toml`**: Rustプロジェクトのメタデータ（プロジェクト名、バージョンなど）と依存ライブラリを定義するファイルです。
- **`IMPLEMENTATION_PLAN.md`**: プロジェクトの実装計画について記述されたドキュメントです。
- **`IMPLEMENTATION_SUMMARY.md`**: プロジェクトの実装の要約について記述されたドキュメントです。
- **`LICENSE`**: プロジェクトのライセンス情報が記載されています。
- **`PHASE2_INVESTIGATION_REPORT.md`**: プロジェクトの第2フェーズに関する調査報告書です。
- **`PHASE2_STATUS.md`**: プロジェクトの第2フェーズの現在の進捗状況を示すドキュメントです。
- **`QUICK_REFERENCE.md`**: プロジェクトに関する簡易参照ガイドです。
- **`README.ja.md`**: プロジェクトの概要、使い方、セットアップ方法などを日本語で説明するメインのドキュメントです。
- **`README.md`**: プロジェクトの概要、使い方、セットアップ方法などを英語で説明するメインのドキュメントです。
- **`TESTING_GUIDE.md`**: プロジェクトのテスト方法に関するガイドドキュメントです。
- **`_config.yml`**: プロジェクトの設定ファイルであり、Jekyllなどの静的サイトジェネレーターで使用されることがあります。
- **`generated-docs/`**: GitHub Actionsなどにより自動生成されたドキュメントが格納されるディレクトリです。
- **`issue-notes/`**: プロジェクトの特定の課題（Issue）に関する詳細なメモや情報が格納されているディレクトリです（例: `11.md`, `13.md`など）。
- **`src/main.rs`**: このRustプロジェクトのメインとなるソースコードファイルです。MMLの解析と音楽再生ロジックがここに実装されていると推測されます。

## 関数詳細説明
プロジェクト情報からは、具体的な関数の詳細（役割、引数、戻り値、機能）を特定できませんでした。`src/main.rs`にMMLパーサーとプレイヤーの主要なロジックが含まれていると推測されます。

## 関数呼び出し階層ツリー
```
関数呼び出し階層を分析できませんでした。

---
Generated at: 2025-11-09 07:02:57 JST
