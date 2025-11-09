Last updated: 2025-11-09

# Project Overview

## プロジェクト概要
- Music Macro Language (MML) を解析し、音楽として再生するCLIツールです。
- Web Audio API (Tone.js経由) を利用して、MML形式の楽曲を音声として出力します。
- 開発の自動化にはGitHub Actionsが活用され、多言語対応やプロジェクト管理が効率化されています。

## 技術スタック
- フロントエンド: 記載なし（CLIツールであり、Web Audio APIは内部的に音声生成に使用）
- 音楽・オーディオ:
  - Tone.js: Web Audio APIの複雑さを抽象化し、ブラウザで高品質な音声を生成するためのJavaScriptライブラリです。
  - Web Audio API: ウェブブラウザで高度な音声処理を行うためのAPIで、Tone.jsを介して音声出力に利用されます。
  - MML (Music Macro Language): 音符や休符、テンポなどの音楽情報をテキスト形式で記述するための記法パーサーです。
- 開発ツール:
  - Node.js runtime: JavaScriptコードを実行するための環境で、プロジェクトの開発支援やスクリプト実行に利用されます。
- テスト: 記載なし
- ビルドツール: 記載なし（RustプロジェクトではCargoが利用されますが、具体的なビルドツールとしての説明は提供されていません）
- 言語機能: 記載なし（主にRust言語が使用されていることがファイル構造から推測されます）
- 自動化・CI/CD:
  - GitHub Actions: コードの変更時に自動的にテスト、ビルド、デプロイなどのワークフローを実行するCI/CDプラットフォームです。プロジェクト要約の自動生成、Issue管理、READMEの多言語翻訳、国際化（i18n）の自動化などに利用されています。
- 開発標準:
  - EditorConfig: 異なるエディタやIDE間で一貫したコーディングスタイルを維持するための設定ファイルです。

## ファイル階層ツリー
```
.editorconfig
.gitignore
.vscode/
  extensions.json
  settings.json
Cargo.lock
Cargo.toml
IMPLEMENTATION_PLAN.md
IMPLEMENTATION_SUMMARY.md
LICENSE
PHASE2_INVESTIGATION_REPORT.md
PHASE2_STATUS.md
QUICK_REFERENCE.md
README.ja.md
README.md
TESTING_GUIDE.md
_config.yml
generated-docs/
issue-notes/
  11.md
  13.md
  15.md
  17.md
  19.md
  21.md
src/
  app.rs
  cli.rs
  client_manager.rs
  converter.rs
  input.rs
  main.rs
  process_manager.rs
  temp_file.rs
```

## ファイル詳細説明
- **.editorconfig**: 複数の開発者や異なるエディタ間でコードの整形ルール（インデントスタイル、文字コードなど）を統一するための設定ファイルです。
- **.gitignore**: Gitのバージョン管理から除外するファイルやディレクトリ（例: ビルド成果物、一時ファイルなど）を指定するファイルです。
- **.vscode/extensions.json**: Visual Studio Codeのワークスペースで推奨される拡張機能をリストアップし、チームメンバー間での開発環境の一貫性を保ちます。
- **.vscode/settings.json**: Visual Studio Codeのワークスペース固有の設定を定義し、プロジェクトのコーディングスタイルやツールの動作をカスタマイズします。
- **Cargo.lock**: Rustプロジェクトの依存関係ツリーと、それら依存関係の正確なバージョンを記録し、再現可能なビルドを保証します。
- **Cargo.toml**: Rustプロジェクトのマニフェストファイルで、プロジェクトのメタデータ（名前、バージョンなど）、依存関係、ビルド設定などを定義します。
- **IMPLEMENTATION_PLAN.md**: プロジェクトの実装に関する計画や戦略を記述したドキュメントです。
- **IMPLEMENTATION_SUMMARY.md**: 実装の主要な側面や成果をまとめた概要ドキュメントです。
- **LICENSE**: プロジェクトの利用条件を定めるライセンス情報が記載されています。
- **PHASE2_INVESTIGATION_REPORT.md**: プロジェクトの第2フェーズにおける調査結果や分析をまとめたレポートです。
- **PHASE2_STATUS.md**: プロジェクトの第2フェーズの現在の状況や進捗に関するドキュメントです。
- **QUICK_REFERENCE.md**: プロジェクトに関する重要な情報やコマンド、使い方などを素早く参照できるガイドです。
- **README.ja.md**: プロジェクトの概要、目的、使い方などを日本語で説明するメインのドキュメントです。
- **README.md**: プロジェクトの概要、目的、使い方などを英語で説明するメインのドキュメントです。
- **TESTING_GUIDE.md**: プロジェクトのテスト方法、テスト戦略、または特定のテスト手順を説明するガイドです。
- **_config.yml**: GitHub Pagesなどの静的サイトジェネレーターの設定ファイルである可能性があります。
- **generated-docs/**: 自動生成されたドキュメントやレポートを格納するためのディレクトリです。
- **issue-notes/**: プロジェクトの特定の課題（Issue）に関する詳細なメモや補足情報を格納するためのディレクトリです。
- **src/app.rs**: アプリケーションの主要なロジックやコア機能の一部を実装するモジュールです。
- **src/cli.rs**: コマンドラインインターフェース（CLI）の引数解析やコマンド処理ロジックを定義するモジュールです。
- **src/client_manager.rs**: クライアントサイドの接続や状態管理に関連するロジックを扱うモジュールです。
- **src/converter.rs**: MMLデータから音楽イベントへの変換など、特定のデータ形式間の変換ロジックを実装するモジュールです。
- **src/input.rs**: ユーザーからの入力（例: キーボード入力、ファイル入力）を処理するためのロジックを扱うモジュールです。
- **src/main.rs**: アプリケーションのエントリポイントであり、プログラムの起動時に最初に実行されるメイン関数が置かれます。
- **src/process_manager.rs**: 複数のプロセスやタスクの管理、実行、監視に関連するロジックを扱うモジュールです。
- **src/temp_file.rs**: 一時ファイルの作成、読み書き、削除など、一時ファイル操作に関連するユーティリティ関数やロジックを提供するモジュールです。

## 関数詳細説明
このプロジェクトはRust言語で記述されており、以下のファイルに主要な関数が分散しています。具体的な関数名やシグネチャは提供されていませんが、各モジュールが担う役割に基づき、以下のような関数群が存在すると考えられます。

- `src/main.rs`: プログラムのエントリポイントとなる`main`関数が含まれ、アプリケーションの初期化や主要な処理の流れを制御します。
- `src/cli.rs`: コマンドライン引数を解析し、ユーザーが指定したオプションやコマンドに基づいて適切な処理を呼び出す関数群が含まれます。
- `src/app.rs`: アプリケーションのコアロジックを実装する関数群が含まれます。MMLの解析結果に基づいて音楽再生を制御する主要な処理がここに含まれる可能性があります。
- `src/converter.rs`: MML形式のテキストデータを、Tone.jsやWeb Audio APIが理解できる内部的な音楽イベントやデータ構造に変換するための関数群が含まれます。
- `src/input.rs`: ファイルからのMML読み込みや、将来的なインタラクティブな入力処理など、外部からのデータ入力を扱う関数群が含まれます。
- `src/client_manager.rs`: 必要に応じて、Web Audio APIを扱うクライアント（例: 内部的なJavaScriptプロセス）との通信や管理を行う関数群が含まれる可能性があります。
- `src/process_manager.rs`: 音楽再生やMML解析など、アプリケーション内で発生する各種タスクやプロセスの起動・停止・監視を行う関数群が含まれる可能性があります。
- `src/temp_file.rs`: MMLデータの一時的な保存や、Web Audio APIへの受け渡しのために一時ファイルを作成・管理・削除するユーティリティ関数群が含まれます。

## 関数呼び出し階層ツリー
```
関数呼び出し階層を分析できませんでした

---
Generated at: 2025-11-09 21:53:30 JST
